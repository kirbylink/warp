use log::trace;
use std::env;
#[cfg(target_family = "unix")]
use std::fs;
#[cfg(target_family = "unix")]
use std::fs::Permissions;
use std::io;
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
#[cfg(target_family = "windows")]
use std::os::windows::process::CommandExt;

pub fn execute(target: &Path, hidden: bool) -> io::Result<i32> {
    trace!("target={:?}, hidden={}", target, hidden);

    let args: Vec<String> = env::args().skip(1).collect();
    trace!("args={:?}", args);

    do_execute(target, &args, hidden)
}

#[cfg(target_family = "unix")]
fn ensure_executable(target: &Path) {
    let perms = Permissions::from_mode(0o770);
    fs::set_permissions(target, perms).unwrap();
}

#[cfg(target_family = "unix")]
fn do_execute(target: &Path, args: &[String], _hidden: bool) -> io::Result<i32> 
    ensure_executable(target);

    Ok(Command::new(target)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?
        .code()
        .unwrap_or(1))
}

#[cfg(target_family = "windows")]
fn is_script(target: &Path) -> bool {
    const SCRIPT_EXTENSIONS: &[&str] = &["bat", "cmd"];
    SCRIPT_EXTENSIONS.contains(
        &target
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase()
            .as_str(),
    )
}

#[cfg(target_family = "windows")]
fn do_execute(target: &Path, args: &[String], hidden: bool) -> io::Result<i32> {
    let mut cmd = if is_script(target) {
        let mut c = Command::new("cmd");
        c.arg("/c").arg(target).args(args);
        c
    } else {
        let mut c = Command::new(target);
        c.args(args);
        c
    };

    if hidden {
        // 0x08000000 = CREATE_NO_WINDOW
        cmd.creation_flags(0x08000000);
        cmd.stdin(Stdio::null())
           .stdout(Stdio::null())
           .stderr(Stdio::null());
    } else {
        cmd.stdin(Stdio::inherit())
           .stdout(Stdio::inherit())
           .stderr(Stdio::inherit());
    }

    Ok(cmd.spawn()?.wait()?.code().unwrap_or(1))
}
