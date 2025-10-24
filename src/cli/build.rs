// SPDX-FileCopyrightText: 2025 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::{Parser, ValueHint};
use eyre::{Result, eyre};
use owo_colors::OwoColorize as _;

use crate::stages;
use crate::util;

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
    #[clap(last = true)]
    extra_build_flags: Vec<String>,
}

impl super::Command for BuildCommand {
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
            .or_else(|| {
                util::hostname()
                    .ok()
                    .map(|hostname| format!("darwinConfigurations.{hostname}"))
            })
            .ok_or_else(|| eyre!("no attribute was provided and one could not be inferred!"))?;

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

        util::log::success(out.display().dimmed());

        Ok(())
    }
}
