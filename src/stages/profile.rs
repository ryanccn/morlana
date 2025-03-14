use std::{path::Path, process::Stdio};

use eyre::Result;

use crate::util::{self, CommandExt as _};

pub fn profile(profile: &str, out: &Path) -> Result<()> {
    util::sudo("nix-env")
        .arg("-p")
        .arg(profile)
        .arg("--set")
        .arg(out)
        .stderr(Stdio::inherit())
        .error_for_status("failed to set system with nix-env")?;

    Ok(())
}
