use anyhow::{anyhow, Context, Result};
use bincode::Options;
use flate2::read::GzDecoder;
use log::trace;
use memmem::{Searcher, TwoWaySearcher};
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use tar::Archive;
use warp_args::{bincode_options, Args, WARP_ARGS_MAGIC};

struct FileSearcher<'a> {
    buf_reader: BufReader<File>,
    searcher: TwoWaySearcher<'a>,
    offs: usize,
}

impl<'a> FileSearcher<'a> {
    fn new(path: &'a Path, magic: &'a [u8]) -> io::Result<FileSearcher<'a>> {
        let file = File::open(path)?;
        Ok(FileSearcher {
            buf_reader: BufReader::new(file),
            searcher: TwoWaySearcher::new(magic),
            offs: 0,
        })
    }
}

impl<'a> Iterator for FileSearcher<'a> {
    type Item = io::Result<usize>;

    fn next(&mut self) -> Option<io::Result<usize>> {
        let mut buf = [0; 32 * 1024];
        let ret;

        match self.buf_reader.seek(SeekFrom::Start(self.offs as u64)) {
            Ok(_) => {}
            Err(e) => return Some(Err(e)),
        }

        loop {
            match self.buf_reader.read(&mut buf[..]) {
                Ok(0) => {
                    ret = None;
                    break;
                }
                Ok(n) => {
                    match self.searcher.search_in(&buf) {
                        Some(pos) => {
                            self.offs += pos;
                            ret = Some(Ok(self.offs));
                            self.offs += 1; // one past the match so we can try again if necessary
                            break;
                        }
                        None => self.offs += n,
                    }
                }
                Err(e) => {
                    ret = Some(Err(e));
                    break;
                }
            }
        }
        ret
    }
}

const GZIP_MAGIC: &[u8] = b"\x1f\x8b\x08";

pub fn extract_to(src: &Path, dst: &Path) -> Result<()> {
    FileSearcher::new(src, GZIP_MAGIC)
        .context("failed searching own binary")?
        .map(Result::unwrap)
        .find(|offs| extract_at_offset(src, *offs, dst).unwrap())
        .ok_or_else(|| anyhow!("No tarball found inside binary file {}", src.display()))
        .map(|offs| {
            trace!(
                "tarball found at offset {} was extracted successfully",
                offs
            );
        })
}

fn extract_at_offset(src: &Path, offs: usize, dst: &Path) -> Result<bool> {
    let mut f = File::open(src)
        .with_context(|| format!("Failed to open file to extract from: {}", src.display()))?;
    f.seek(SeekFrom::Start(offs as u64))
        .with_context(|| format!("Failed to read file to extract from: {}", src.display()))?;

    let gz = GzDecoder::new(f);
    let mut tar = Archive::new(gz);
    Ok(tar.unpack(dst).is_ok())
}

pub fn get_args(src: &Path) -> Result<Args> {
    FileSearcher::new(src, WARP_ARGS_MAGIC)
        .context("failed searching own binary")?
        .map(Result::unwrap)
        .find_map(|offs| {
            Some((
                offs,
                extract_args_at_offset(src, offs + WARP_ARGS_MAGIC.len()).unwrap()?,
            ))
        })
        .ok_or_else(|| anyhow!("No arguments found inside binary file {}", src.display()))
        .map(|(offs, args)| {
            trace!("args found at offset {} was extracted successfully", offs);
            args
        })
}

fn extract_args_at_offset(src: &Path, offs: usize) -> Result<Option<Args>> {
    let mut f = File::open(src)
        .with_context(|| format!("Failed to open file to extract from: {}", src.display()))?;
    f.seek(SeekFrom::Start(offs as u64))
        .with_context(|| format!("Failed to read file to extract from: {}", src.display()))?;

    Ok(bincode_options().deserialize_from(f).ok())
}
