use clap::{Parser, ValueHint};
use eyre::Result;
use owo_colors::OwoColorize as _;

use crate::stages;
use crate::util;

fn default_nom() -> bool {
    util::which("nom").is_some()
}

#[derive(Parser, Debug)]
pub struct BuildCommand {
    /// Path to a flake
    #[clap(long, value_hint = ValueHint::DirPath)]
    flake: Option<String>,

    /// Flake attribute that contains a nix-darwin configuration [defaults to `darwinConfigurations.<hostname>`]
    #[clap(long)]
    attr: Option<String>,

    /// Use nix-output-monitor [defaults to true if `nom` is in PATH]
    #[clap(long)]
    nom: Option<bool>,

    /// Extra build flags to pass to Nix
    #[clap(long)]
    extra_build_flags: Vec<String>,
}

impl super::Command for BuildCommand {
    fn action(&self, _global_options: &super::Cli) -> Result<()> {
        let build_program = if self.nom.unwrap_or_else(default_nom) {
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
            true,
        )?;

        util::log::success(out.to_string_lossy().dimmed());

        Ok(())
    }
}
