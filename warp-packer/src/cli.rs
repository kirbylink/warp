use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Pack(PackArgs),
    List,
}

#[derive(Args, Debug)]
pub struct PackArgs {
    /// Sets the architecture. Use list subcommand to get possible options.
    #[arg(short, long)]
    pub arch: String,

    /// Sets the input directory containing the application and dependencies
    #[arg(short, long)]
    pub input_dir: PathBuf,

    /// Sets the application executable file name
    #[arg(short, long)]
    pub exec: PathBuf,

    /// Sets the resulting self-contained application file name
    #[arg(short, long)]
    pub output: PathBuf,

    /// Generate unique id for each package build
    #[arg(short = 'q', long, default_value_t = false)]
    pub unique_id: bool,

    /// Prefix to use instead of single-file executable name
    #[arg(short, long)]
    pub prefix: Option<PathBuf>,

    /// When using unique-id, do not look for and clean obsolete versions with the same prefix from cache
    #[arg(short = 'n', long = "no-clean", action = clap::ArgAction::SetFalse)]
    pub clean: bool,
}
