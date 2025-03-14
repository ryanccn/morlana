use clap::{Parser, ValueHint};
use eyre::Result;
use owo_colors::OwoColorize as _;

use crate::stages;
use crate::util;

fn default_profile() -> String {
    "/nix/var/nix/profiles/system".to_owned()
}

#[derive(Parser, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct SwitchCommand {
    /// Path to a flake
    #[clap(long, value_hint = ValueHint::DirPath)]
    flake: Option<String>,

    /// Flake attribute that contains a nix-darwin configuration [defaults to `darwinConfigurations.<hostname>`]
    #[clap(long)]
    attr: Option<String>,

    /// System profile location
    #[clap(long, default_value_t = default_profile(), value_hint = ValueHint::DirPath)]
    profile: String,

    /// Use nix-output-monitor [defaults to true if `nom` is in PATH]
    #[clap(long)]
    nom: Option<bool>,

    /// Whether to use nvd [defaults to true if `nvd` is in PATH]
    #[clap(long)]
    nvd: Option<bool>,

    /// Extra build flags to pass to Nix
    #[clap(last = true)]
    extra_build_flags: Vec<String>,

    /// Use the default choice for confirmation prompts
    #[clap(long)]
    no_confirm: bool,
}

impl super::Command for SwitchCommand {
    fn action(&self, _global_options: &super::Cli) -> Result<()> {
        let build_program = if self.nom.unwrap_or_else(util::nom_available) {
            "nom"
        } else {
            "nix"
        };

        let flake_url = match &self.flake {
            Some(flake) => flake,
            None => &util::default_flake()?,
        };

        let flake_attr = self
            .attr
            .clone()
            .unwrap_or(format!("darwinConfigurations.{}", util::hostname()?));

        util::log::info(format!(
            "building system {}",
            format!("({build_program})").dimmed()
        ));

        let out = stages::build(
            build_program,
            flake_url,
            &flake_attr,
            &self.extra_build_flags,
            false,
        )?;

        util::log::success(out.display().dimmed());

        if self.nvd.unwrap_or_else(util::nvd_available) {
            stages::diff(&out)?;
        }

        if !self.no_confirm && !util::log::confirm("switch to configuration?", false)? {
            return Ok(());
        }

        util::log::info("configuring profile");
        stages::profile(&self.profile, &out)?;

        util::log::info("activating");
        stages::activate(&out)?;

        Ok(())
    }
}
