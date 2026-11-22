mod cli;

use bincode::Options;
use clap::Parser;
use lazy_static::lazy_static;
use rayon::prelude::*;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::{collections::HashMap, io::BufWriter};
use uuid::Uuid;
use warp_args::{bincode_options, index_bincode_options, Args, Chunk, Entry, EntryKind, Footer};

use crate::cli::Command;

lazy_static! {
    static ref RUNNER_BY_ARCH: HashMap<&'static str, &'static [u8]> = {
        let mut m = HashMap::new();
        m.insert(
            "linux-x64",
            include_bytes!("../../target/x86_64-unknown-linux-gnu/release/warp-runner").as_slice(),
        );
        m.insert(
            "linux-aarch64",
            include_bytes!("../../target/aarch64-unknown-linux-gnu/release/warp-runner").as_slice(),
        );
        m.insert(
            "macos-x64",
            include_bytes!("../../target/x86_64-apple-darwin/release/warp-runner").as_slice(),
        );
        m.insert(
            "macos-aarch64",
            include_bytes!("../../target/aarch64-apple-darwin/release/warp-runner").as_slice(),
        );
        m.insert(
            "windows-x64",
            include_bytes!("../../target/x86_64-pc-windows-gnu/release/warp-runner.exe").as_slice(),
        );
        m.insert(
            "windows-aarch64",
            include_bytes!("../../target/aarch64-pc-windows-gnullvm/release/warp-runner.exe")
                .as_slice(),
        );
        m
    };
}

/// Print a message to stderr and exit with error code 1
macro_rules! bail {
    () => (process::exit(1));
    ($($arg:tt)*) => ({
        eprint!("{}\n", format_args!($($arg)*));
        process::exit(1);
    })
}

const ZSTD_LEVEL: i32 = 19;
const CHUNK_SIZE: usize = 2 << 20;
const BATCH_SIZE: u64 = 64 << 20;

struct SourceFile {
    entry: usize,
    path: PathBuf,
    len: u64,
}

fn utf8_path(path: &Path) -> io::Result<String> {
    path.components()
        .map(|c| {
            c.as_os_str().to_str().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("non UTF-8 path {:?}", path),
                )
            })
        })
        .collect::<io::Result<Vec<_>>>()
        .map(|parts| parts.join("/"))
}

#[cfg(target_family = "unix")]
fn entry_mode(meta: &fs::Metadata) -> u32 {
    use std::os::unix::fs::PermissionsExt;

    meta.permissions().mode() & 0o7777
}

#[cfg(target_family = "windows")]
fn entry_mode(meta: &fs::Metadata) -> u32 {
    if meta.is_dir() {
        0o755
    } else {
        0o644
    }
}

fn collect_entries(
    root: &Path,
    dir: &Path,
    entries: &mut Vec<Entry>,
    files: &mut Vec<SourceFile>,
) -> io::Result<()> {
    let mut items = fs::read_dir(dir)?.collect::<io::Result<Vec<_>>>()?;
    items.sort_by_key(|item| item.file_name());
    for item in items {
        let path = item.path();
        let meta = fs::symlink_metadata(&path)?;
        let file_type = meta.file_type();
        let kind = if file_type.is_symlink() {
            let link = fs::read_link(&path)?;
            let target = link.to_str().map(str::to_owned).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("non UTF-8 symlink {:?}", link),
                )
            })?;
            EntryKind::Symlink { target }
        } else if file_type.is_dir() {
            EntryKind::Dir
        } else {
            files.push(SourceFile {
                entry: entries.len(),
                path: path.clone(),
                len: meta.len(),
            });
            EntryKind::File {
                len: 0,
                chunks: Vec::new(),
            }
        };
        entries.push(Entry {
            path: utf8_path(path.strip_prefix(root).unwrap())?,
            mode: entry_mode(&meta),
            kind,
        });
        if file_type.is_dir() {
            collect_entries(root, &path, entries, files)?;
        }
    }
    Ok(())
}

fn compress_file(path: &Path) -> io::Result<Vec<(u32, Vec<u8>)>> {
    let data = fs::read(path)?;
    data.par_chunks(CHUNK_SIZE)
        .map(|chunk| zstd::bulk::compress(chunk, ZSTD_LEVEL).map(|z| (chunk.len() as u32, z)))
        .collect()
}

fn append_payload(f: &mut impl io::Write, offset: u64, dir: &Path, args: &Args) -> io::Result<()> {
    let mut entries = Vec::new();
    let mut files = Vec::new();
    collect_entries(dir, dir, &mut entries, &mut files)?;

    let mut batches: Vec<Vec<&SourceFile>> = Vec::new();
    let mut batch_len = BATCH_SIZE;
    for file in &files {
        if batch_len >= BATCH_SIZE {
            batches.push(Vec::new());
            batch_len = 0;
        }
        batches.last_mut().unwrap().push(file);
        batch_len += file.len;
    }

    let mut pos = offset;
    for batch in batches {
        let compressed = batch
            .par_iter()
            .map(|file| compress_file(&file.path))
            .collect::<io::Result<Vec<_>>>()?;
        for (file, file_chunks) in batch.iter().zip(compressed) {
            let EntryKind::File { len, chunks } = &mut entries[file.entry].kind else {
                unreachable!()
            };
            for (chunk_len, data) in file_chunks {
                f.write_all(&data)?;
                chunks.push(Chunk {
                    offset: pos,
                    compressed_len: data.len() as u32,
                    len: chunk_len,
                });
                pos += data.len() as u64;
                *len += chunk_len as u64;
            }
        }
    }

    let args = bincode_options()
        .serialize(args)
        .map_err(io::Error::other)?;
    let index = index_bincode_options()
        .serialize(&entries)
        .map_err(io::Error::other)?;
    f.write_all(&args)?;
    f.write_all(&index)?;
    let footer = Footer {
        args_offset: pos,
        args_len: args.len() as u64,
        index_offset: pos + args.len() as u64,
        index_len: index.len() as u64,
    };
    f.write_all(&footer.to_bytes())
}

#[cfg(target_family = "unix")]
fn create_app_file(out: &Path) -> io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt;

    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .mode(0o755)
        .open(out)
}

#[cfg(target_family = "windows")]
fn create_app_file(out: &Path) -> io::Result<File> {
    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(out)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Cli::parse();

    let args = match args.command {
        Command::List => {
            for arch in RUNNER_BY_ARCH.keys() {
                println!("{arch}");
            }
            return Ok(());
        }
        Command::Pack(args) => args,
    };

    if !RUNNER_BY_ARCH.contains_key(&args.arch.as_str()) {
        bail!(
            "Unknown architecture specified: {}, supported: {:?}",
            args.arch,
            RUNNER_BY_ARCH.keys()
        );
    }

    if fs::metadata(&args.input_dir).is_err() {
        bail!(
            "Cannot access specified input directory {:?}",
            args.input_dir
        );
    }

    let exec_path = args.input_dir.join(&args.exec);
    match fs::metadata(&exec_path) {
        Err(_) => {
            bail!("Cannot find file {:?}", exec_path);
        }
        Ok(metadata) => {
            if !metadata.is_file() {
                bail!("{:?} isn't a file", exec_path);
            }
        }
    }

    println!(
        "Creating self-contained application binary {:?}...",
        args.exec
    );
    let runner = RUNNER_BY_ARCH.get(&args.arch.as_str()).unwrap();
    let mut output = BufWriter::new(create_app_file(&args.output).unwrap());
    output.write_all(runner)?;

    println!("Compressing input directory {:?}...", args.input_dir);
    append_payload(
        &mut output,
        runner.len() as u64,
        &args.input_dir,
        &Args {
            target_file_name: args.exec.to_string_lossy().into_owned(),
            prefix: args.prefix,
            uid: args.unique_id.then(|| format!("{}", Uuid::new_v4())),
            clean: args.clean,
            hidden: args.hidden,
        },
    )
    .unwrap();
    output.flush()?;

    println!("All done");
    Ok(())
}
