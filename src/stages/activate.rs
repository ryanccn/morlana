use std::{
    path::Path,
    process::{Command, Stdio},
};

use eyre::Result;

use crate::util::{self, CommandExt as _};

pub fn activate_user(out: &Path) -> Result<()> {
    Command::new(out.join("activate-user"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .error_for_status("failed to activate-user")?;

    Ok(())
}

pub fn activate(out: &Path) -> Result<()> {
    util::sudo_cmd(out.join("activate"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .error_for_status("failed to activate")?;

    Ok(())
}
