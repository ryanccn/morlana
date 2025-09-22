use std::{env, ffi::OsStr, path::Path, process::Command};

use clap::Parser;
use eyre::{Result, eyre};
use owo_colors::OwoColorize;

use crate::util::{self, CommandExt as _};

fn wipe(profile: impl AsRef<OsStr>, sudo: bool, older_than: Option<&str>) -> Result<()> {
    let mut command = if sudo {
        util::sudo("nix")
    } else {
        Command::new("nix")
    };
    command.args(["profile", "wipe-history", "--profile"]);
    command.arg(&profile);

    if let Some(older_than) = older_than {
        command.args(["--older-than", older_than]);
    }

    command.error_for_status("failed to execute `nix profile wipe-history`")?;

    util::log::success(format!(
        "wiped history for {}",
        profile.as_ref().display().green()
    ));

    Ok(())
}

#[derive(Parser)]
pub struct WipeHistoryCommand {
    /// Delete versions older than the specified age (passed to `nix profile wipe-history`)
    #[arg(long)]
    older_than: Option<String>,
}

impl super::Command for WipeHistoryCommand {
    fn action(&self, _global_options: &super::Cli) -> Result<()> {
        wipe(
            "/nix/var/nix/profiles/default",
            true,
            self.older_than.as_deref(),
        )?;
        wipe(
            "/nix/var/nix/profiles/system",
            true,
            self.older_than.as_deref(),
        )?;
        wipe(
            "/nix/var/nix/profiles/per-user/root/profile",
            true,
            self.older_than.as_deref(),
        )?;

        let xdg_state_home = match env::var_os("XDG_STATE_HOME") {
            Some(s) => Path::new(&s).to_path_buf(),
            None => env::home_dir()
                .ok_or_else(|| eyre!("could not obtain XDG_STATE_HOME from home directory"))?
                .join(".local/state"),
        };

        wipe(
            xdg_state_home.join("nix/profiles/profile"),
            false,
            self.older_than.as_deref(),
        )?;
        wipe(
            xdg_state_home.join("nix/profiles/home-manager"),
            false,
            self.older_than.as_deref(),
        )?;

        Ok(())
    }
}
