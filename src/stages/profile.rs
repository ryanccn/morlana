use std::{path::Path, process::Stdio};

use eyre::{bail, Result};

use crate::util;

pub fn profile(profile: &str, out: &Path) -> Result<()> {
    let mut nix_env_cmd = util::sudo_cmd("nix-env");
    nix_env_cmd
        .arg("-p")
        .arg(profile)
        .arg("--set")
        .arg(out)
        .stderr(Stdio::inherit());

    let nix_env_status = nix_env_cmd.status()?;
    if !nix_env_status.success() {
        bail!("failed to set system with nix-env");
    }

    Ok(())
}
