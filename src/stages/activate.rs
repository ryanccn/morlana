use std::{
    path::Path,
    process::{Command, Stdio},
};

use eyre::{bail, Result};

use crate::util;

pub fn activate_user(out: &Path) -> Result<()> {
    if !Command::new(out.join("activate-user"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?
        .success()
    {
        bail!("failed to activate-user");
    }

    Ok(())
}

pub fn activate(out: &Path) -> Result<()> {
    if !util::sudo_cmd(out.join("activate"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?
        .success()
    {
        bail!("failed to activate");
    }

    Ok(())
}
