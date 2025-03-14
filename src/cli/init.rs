use std::path::Path;

use clap::Parser;
use eyre::Result;
use owo_colors::OwoColorize as _;

use crate::stages;
use crate::util;

#[derive(Parser, Debug)]
pub struct InitCommand {
    // Use the default choice for confirmation prompts
    #[clap(long)]
    no_confirm: bool,
}

impl super::Command for InitCommand {
    fn action(&self, _global_options: &super::Cli) -> Result<()> {
        if util::which("nix").is_none() {
            util::log::error("Nix is not installed! You need to install Nix first.");
            return Ok(());
        }

        if Path::new("/run/current-system").exists() {
            util::log::warn("nix-darwin seems to already be installed!");
            if !self.no_confirm
                && !util::log::confirm(
                    "are you sure you want to reinitialize your installation?",
                    false,
                )?
            {
                return Ok(());
            }
        }

        let flake_url = stages::bootstrap()?;
        let flake_attr = format!("darwinConfigurations.{}", util::hostname()?);

        util::log::info("building bootstrap system");

        let out = stages::build("nix", &flake_url.to_string_lossy(), &flake_attr, &[], false)?;

        util::log::success(out.display().dimmed());

        util::log::info("configuring profile");
        stages::profile("/nix/var/nix/profiles/system", &out)?;

        util::log::info("activating");
        stages::activate(&out)?;

        Ok(())
    }
}
