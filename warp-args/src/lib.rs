use std::path::PathBuf;

use bincode::Options;
use serde::{Deserialize, Serialize};

pub const FOOTER_MAGIC: &[u8; 12] = b"WARP-PAYLOAD";
pub const FOOTER_LEN: usize = 32 + FOOTER_MAGIC.len();

#[derive(Serialize, Deserialize, Debug)]
pub struct Args {
    pub target_file_name: String,
    pub prefix: Option<PathBuf>,
    pub uid: Option<String>,
    pub clean: bool,

    #[serde(default)]
    pub hidden: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chunk {
    pub offset: u64,
    pub compressed_len: u32,
    pub len: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EntryKind {
    Dir,
    File { len: u64, chunks: Vec<Chunk> },
    Symlink { target: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub path: String,
    pub mode: u32,
    pub kind: EntryKind,
}

#[derive(Debug)]
pub struct Footer {
    pub args_offset: u64,
    pub args_len: u64,
    pub index_offset: u64,
    pub index_len: u64,
}

impl Footer {
    pub fn to_bytes(&self) -> [u8; FOOTER_LEN] {
        let mut bytes = [0u8; FOOTER_LEN];
        let fields = [
            self.args_offset,
            self.args_len,
            self.index_offset,
            self.index_len,
        ];
        for (i, value) in fields.iter().enumerate() {
            bytes[i * 8..i * 8 + 8].copy_from_slice(&value.to_le_bytes());
        }
        bytes[32..].copy_from_slice(FOOTER_MAGIC);
        bytes
    }

    pub fn from_bytes(bytes: &[u8; FOOTER_LEN]) -> Option<Footer> {
        if bytes[32..] != FOOTER_MAGIC[..] {
            return None;
        }
        let field = |i: usize| u64::from_le_bytes(bytes[i * 8..i * 8 + 8].try_into().unwrap());
        Some(Footer {
            args_offset: field(0),
            args_len: field(1),
            index_offset: field(2),
            index_len: field(3),
        })
    }
}

pub fn bincode_options() -> impl bincode::Options {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .with_limit(1_000_000)
}

pub fn index_bincode_options() -> impl bincode::Options {
    bincode::DefaultOptions::new().with_fixint_encoding()
}
