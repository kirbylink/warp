use std::path::PathBuf;

use bincode::Options;
use serde::{Deserialize, Serialize};

pub const WARP_ARGS_MAGIC: &[u8] = b"DR1PWsJsM6KxNbng9Y38";

#[derive(Serialize, Deserialize, Debug)]
pub struct Args {
    pub target_file_name: PathBuf,
    pub prefix: Option<PathBuf>,
    pub uid: Option<String>,
    pub clean: bool,
}

pub fn bincode_options() -> impl bincode::Options {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        // Make sure bincode does not trust all size prefixes while scanning the binary
        .with_limit(1_000_000)
}
