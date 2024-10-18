use std::{
    path::Path,
    process::{Command, Stdio},
};

use eyre::Result;

use crate::util::CommandExt as _;

pub fn diff(out: &Path) -> Result<()> {
    if !Path::new("/run/current-system").exists() {
        return Ok(());
    }

    Command::new("nvd")
        .args(["diff", "/run/current-system"])
        .arg(out)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .error_for_status("failed to diff with nvd")?;

    Ok(())
}
