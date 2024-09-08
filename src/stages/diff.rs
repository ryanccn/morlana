use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use eyre::{bail, Result};

pub fn diff(out: &Path) -> Result<()> {
    if !PathBuf::from("/run/current-system").exists() {
        return Ok(());
    }

    if !Command::new("nvd")
        .args(["diff", "/run/current-system"])
        .arg(out)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?
        .success()
    {
        bail!("failed to diff with nvd");
    }

    Ok(())
}
