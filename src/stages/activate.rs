use std::{
    path::Path,
    process::{Command, Stdio},
};

use eyre::Result;

use crate::util::{self, CommandExt as _};

pub fn activate(out: &Path) -> Result<()> {
    let system_wide_activation = !out.join("activate-user").try_exists()?;
    let darwin_rebuild = out.join("sw").join("bin").join("darwin-rebuild");

    if system_wide_activation {
        util::sudo(&darwin_rebuild)
    } else {
        Command::new(&darwin_rebuild)
    }
    .arg("activate")
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .error_for_status("failed to activate")?;

    Ok(())
}
