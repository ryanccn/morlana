use std::{
    env,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    process::{Command, Stdio, exit},
};

use eyre::Result;

mod command_ext;
pub use command_ext::CommandExt;
pub mod log;

pub fn hostname() -> Result<String> {
    let output = Command::new("scutil")
        .args(["--get", "LocalHostName"])
        .stderr(Stdio::inherit())
        .error_for_status("failed to obtain hostname from scutil")?;

    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

pub fn which(exe_name: impl AsRef<Path>) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .map(|d| d.join(&exe_name))
            .find(|f| f.is_file())
    })
}

pub fn nom_available() -> bool {
    which("nom").is_some()
}

pub fn nvd_available() -> bool {
    which("nvd").is_some()
}

pub fn safe_remove_file(path: impl AsRef<Path>) -> Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => Ok(()),
            _ => Err(err.into()),
        },
    }
}

pub fn sudo(program: impl AsRef<OsStr>) -> Command {
    if nix::unistd::Uid::effective().is_root() {
        return Command::new(program);
    }

    let mut c = Command::new("sudo");
    c.arg("-H").arg(program);
    c
}

#[derive(serde::Deserialize)]
struct NixFlakeMetadata {
    #[serde(rename = "resolvedUrl")]
    resolved_url: String,
}

pub fn default_flake() -> Result<String> {
    let output = Command::new("nix")
        .args(["flake", "metadata", "--json"])
        .stderr(Stdio::inherit())
        .error_for_status("failed to obtain flake metadata")?;

    let NixFlakeMetadata { resolved_url } = serde_json::from_slice(&output.stdout)?;
    Ok(resolved_url)
}

pub fn check_root() {
    if !nix::unistd::Uid::effective().is_root() {
        log::error("this command must be run as root!");
        exit(1);
    }
}
