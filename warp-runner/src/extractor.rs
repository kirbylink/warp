use anyhow::{anyhow, bail, Context, Result};
use bincode::Options;
use log::trace;
use rayon::prelude::*;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;
use warp_args::{
    bincode_options, index_bincode_options, Args, Chunk, Entry, EntryKind, Footer, FOOTER_LEN,
};

fn read_footer(file: &mut File) -> Result<Footer> {
    if file.metadata()?.len() < FOOTER_LEN as u64 {
        bail!("binary is too small to contain a payload");
    }
    file.seek(SeekFrom::End(-(FOOTER_LEN as i64)))?;
    let mut bytes = [0u8; FOOTER_LEN];
    file.read_exact(&mut bytes)?;
    Footer::from_bytes(&bytes).ok_or_else(|| anyhow!("no payload found"))
}

fn read_section(file: &mut File, offset: u64, len: u64) -> Result<Vec<u8>> {
    let file_len = file.metadata()?.len();
    if offset.checked_add(len).is_none_or(|end| end > file_len) {
        bail!("payload section out of bounds");
    }
    file.seek(SeekFrom::Start(offset))?;
    let mut bytes = vec![0u8; len as usize];
    file.read_exact(&mut bytes)?;
    Ok(bytes)
}

pub fn get_args(src: &Path) -> Result<Args> {
    let mut file = File::open(src)
        .with_context(|| format!("Failed to open file to extract from: {}", src.display()))?;
    let footer = read_footer(&mut file)?;
    trace!("footer={:?}", footer);
    let bytes = read_section(&mut file, footer.args_offset, footer.args_len)?;
    Ok(bincode_options().deserialize(&bytes)?)
}

pub fn extract_to(src: &Path, dst: &Path) -> Result<()> {
    let mut file = File::open(src)
        .with_context(|| format!("Failed to open file to extract from: {}", src.display()))?;
    let footer = read_footer(&mut file)?;
    let index = read_section(&mut file, footer.index_offset, footer.index_len)?;
    let entries: Vec<Entry> = index_bincode_options().deserialize(&index)?;

    fs::create_dir_all(dst)?;
    for entry in &entries {
        if let EntryKind::Dir = entry.kind {
            fs::create_dir_all(dst.join(&entry.path))?;
        }
    }

    let files: Vec<(&Entry, u64, &[Chunk])> = entries
        .iter()
        .filter_map(|entry| match &entry.kind {
            EntryKind::File { len, chunks } => Some((entry, *len, chunks.as_slice())),
            _ => None,
        })
        .collect();
    let handles = files
        .par_iter()
        .map(|(entry, len, _)| create_file(&dst.join(&entry.path), entry.mode, *len))
        .collect::<Result<Vec<_>>>()?;

    let mut tasks = Vec::new();
    for (handle, (_, _, chunks)) in handles.iter().zip(&files) {
        let mut pos = 0u64;
        for chunk in chunks.iter() {
            tasks.push((handle, pos, chunk));
            pos += chunk.len as u64;
        }
    }
    tasks.par_iter().try_for_each_init(
        || (Vec::new(), Vec::new()),
        |(compressed, buf): &mut (Vec<u8>, Vec<u8>), &(handle, pos, chunk)| -> Result<()> {
            let compressed_len = chunk.compressed_len as usize;
            if compressed.len() < compressed_len {
                compressed.resize(compressed_len, 0);
            }
            read_at(&file, &mut compressed[..compressed_len], chunk.offset)?;
            let len = chunk.len as usize;
            if buf.len() < len {
                buf.resize(len, 0);
            }
            if zstd::bulk::decompress_to_buffer(&compressed[..compressed_len], &mut buf[..len])?
                != len
            {
                bail!("corrupt chunk");
            }
            write_at(handle, &buf[..len], pos)?;
            Ok(())
        },
    )?;

    for entry in &entries {
        if let EntryKind::Symlink { target } = &entry.kind {
            create_symlink(target, &dst.join(&entry.path))?;
        }
    }
    trace!("extracted {} entries", entries.len());
    Ok(())
}

#[cfg(target_family = "unix")]
fn create_file(path: &Path, mode: u32, len: u64) -> Result<File> {
    use std::os::unix::fs::OpenOptionsExt;

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(mode)
        .open(path)
        .with_context(|| format!("Failed to create {}", path.display()))?;
    file.set_len(len)?;
    Ok(file)
}

#[cfg(target_family = "windows")]
fn create_file(path: &Path, _mode: u32, len: u64) -> Result<File> {
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .with_context(|| format!("Failed to create {}", path.display()))?;
    file.set_len(len)?;
    Ok(file)
}

#[cfg(target_family = "unix")]
fn read_at(file: &File, buf: &mut [u8], offset: u64) -> io::Result<()> {
    use std::os::unix::fs::FileExt;

    file.read_exact_at(buf, offset)
}

#[cfg(target_family = "windows")]
fn read_at(file: &File, mut buf: &mut [u8], mut offset: u64) -> io::Result<()> {
    use std::os::windows::fs::FileExt;

    while !buf.is_empty() {
        let n = file.seek_read(buf, offset)?;
        if n == 0 {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        buf = &mut std::mem::take(&mut buf)[n..];
        offset += n as u64;
    }
    Ok(())
}

#[cfg(target_family = "unix")]
fn write_at(file: &File, buf: &[u8], offset: u64) -> io::Result<()> {
    use std::os::unix::fs::FileExt;

    file.write_all_at(buf, offset)
}

#[cfg(target_family = "windows")]
fn write_at(file: &File, mut buf: &[u8], mut offset: u64) -> io::Result<()> {
    use std::os::windows::fs::FileExt;

    while !buf.is_empty() {
        let n = file.seek_write(buf, offset)?;
        if n == 0 {
            return Err(io::ErrorKind::WriteZero.into());
        }
        buf = &buf[n..];
        offset += n as u64;
    }
    Ok(())
}

#[cfg(target_family = "unix")]
fn create_symlink(target: &str, link: &Path) -> io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(target_family = "windows")]
fn create_symlink(target: &str, link: &Path) -> io::Result<()> {
    let resolved = link.parent().map(|parent| parent.join(target));
    if resolved.is_some_and(|path| path.is_dir()) {
        std::os::windows::fs::symlink_dir(target, link)
    } else {
        std::os::windows::fs::symlink_file(target, link)
    }
}
