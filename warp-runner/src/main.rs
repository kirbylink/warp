use anyhow::Context;
use anyhow::{anyhow, Result};
use log::{trace, Level};
use remove_dir_all::remove_dir_all;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use warp_args::Args;

use crate::extractor::get_args;

mod executor;
mod extractor;

static STATIC_SUBFOLDER_NAME: &str = "static";

fn cache_path(target: &str) -> Result<PathBuf> {
    Ok(dirs::data_local_dir()
        .ok_or_else(|| anyhow!("No data local dir found"))?
        .join("warp")
        .join("packages")
        .join(target))
}

fn extract(exe_path: &Path, cache_path: &Path) -> Result<()> {
    if cache_path.exists() {
        remove_dir_all(cache_path)
            .with_context(|| format!("Failed to remove directory {}", cache_path.display()))?;
    }
    extractor::extract_to(exe_path, cache_path).with_context(|| {
        format!(
            "Failed to extract {} to {}",
            exe_path.display(),
            cache_path.display()
        )
    })
}

fn create_cache_folder(self_path: &Path, args: &Args) -> Result<PathBuf> {
    let prefix = args
        .prefix
        .as_deref()
        .unwrap_or_else(|| Path::new(self_path.file_name().unwrap()));
    let cache_path = cache_path(&prefix.to_string_lossy())?;

    trace!("self_path={:?}", self_path);
    trace!("prefix={:?}", prefix);
    trace!("cache_path={:?}", cache_path);

    if cache_path.exists() && !fs::metadata(&cache_path)?.is_dir() {
        return Err(anyhow!("cache at {:?} is not a directory", &cache_path));
    } else {
        fs::create_dir_all(&cache_path)?;
    }

    Ok(cache_path)
}

fn clean_cache(cache_folder: &Path, self_path: &Path, args: &Args) -> Result<()> {
    if args.uid.is_none() {
        let subfolder = cache_folder.join(STATIC_SUBFOLDER_NAME);
        if subfolder.exists()
            && fs::metadata(&subfolder)?.modified()? < fs::metadata(self_path)?.modified()?
        {
            trace!("static cache older than source, removing");
            remove_dir_all(subfolder)?;
        }

        return Ok(());
    }

    if !args.clean {
        return Ok(());
    }

    let uid = OsStr::new(args.uid.as_ref().unwrap()); // Checked above
    let static_folder = OsStr::new(STATIC_SUBFOLDER_NAME);

    trace!("cleaning cache");
    for entry in fs::read_dir(cache_folder)? {
        let entry = entry?;

        if entry.file_name() == static_folder {
            trace!("skipped static subfolder {:?}", static_folder);
            continue;
        }

        if entry.file_name() == uid {
            trace!("skipped own uid {:?}", uid);
            continue;
        }

        trace!("removing entry {:?}", entry);
        if let Err(err) = remove_dir_all(entry.path()) {
            eprintln!(
                "Error while attempting to remove directory {:?}: {err}",
                entry.path()
            );
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    if env::var("WARP_TRACE").is_ok() {
        simple_logger::init_with_level(Level::Trace)?;
    }

    let self_path = env::current_exe()?;
    let args = get_args(&self_path).unwrap();
    trace!("args = {:?}", args);

    let cache_path = create_cache_folder(&self_path, &args)?;
    clean_cache(&cache_path, &self_path, &args)?;

    let subfolder = args.uid.as_deref().unwrap_or(STATIC_SUBFOLDER_NAME);
    let cache_path = cache_path.join(subfolder);

    let target_path = cache_path.join(&args.target_file_name);

    trace!("target_exec={:?}", args.target_file_name);
    trace!("target_path={:?}", target_path);

    if !cache_path.exists() {
        trace!("cache empty, extracting");
        extract(&self_path, &cache_path)?;
    }

    process::exit(executor::execute(&target_path)?);
}
