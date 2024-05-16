mod cli;

use bincode::Options;
use clap::Parser;
use flate2::write::GzEncoder;
use flate2::Compression;
use lazy_static::lazy_static;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process;
use std::{collections::HashMap, io::BufWriter};
use uuid::Uuid;
use warp_args::{bincode_options, Args, WARP_ARGS_MAGIC};

use crate::cli::Command;

lazy_static! {
    static ref RUNNER_BY_ARCH: HashMap<&'static str, &'static [u8]> = {
        let mut m = HashMap::new();
        m.insert(
            "linux-x64",
            include_bytes!("../../target/x86_64-unknown-linux-musl/release/warp-runner").as_slice(),
        );
        m.insert(
            "macos-x64",
            include_bytes!("../../target/x86_64-apple-darwin/release/warp-runner").as_slice(),
        );
        m.insert(
            "windows-x64",
            include_bytes!("../../target/x86_64-pc-windows-gnu/release/warp-runner.exe").as_slice(),
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

fn append_tgz(f: &mut impl io::Write, dir: &Path) -> io::Result<()> {
    let gz = GzEncoder::new(f, Compression::best());
    let mut tar = tar::Builder::new(gz);
    tar.follow_symlinks(false);
    tar.append_dir_all(".", dir)
}

fn append_args(f: &mut impl io::Write, args: &Args) {
    f.write_all(WARP_ARGS_MAGIC).unwrap();
    bincode_options().serialize_into(f, args).unwrap();
}

#[cfg(target_family = "unix")]
fn create_app_file(out: &Path) -> io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt;

    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .mode(0o755)
        .open(out)
}

#[cfg(target_family = "windows")]
fn create_app_file(out: &Path) -> io::Result<File> {
    fs::OpenOptions::new().create(true).write(true).open(out)
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
    let mut output = BufWriter::new(create_app_file(&args.output).unwrap());
    output.write_all(RUNNER_BY_ARCH.get(&args.arch.as_str()).unwrap())?;
    append_args(
        &mut output,
        &Args {
            target_file_name: args.exec,
            prefix: args.prefix,
            uid: args.unique_id.then(|| format!("{}", Uuid::new_v4())),
            clean: args.clean,
        },
    );

    println!("Compressing input directory {:?}...", args.input_dir);
    append_tgz(&mut output, &args.input_dir).unwrap();

    println!("All done");
    Ok(())
}
