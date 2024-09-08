use std::{
    env, fs,
    io::ErrorKind,
    path::PathBuf,
    process::{Command, Stdio},
};

use clap::Parser;
use eyre::{bail, eyre, Result};
use owo_colors::OwoColorize as _;

use crate::{stages, util};

#[derive(serde::Deserialize)]
struct NixTopLevelData {
    outputs: NixOutputsData,
}

#[derive(serde::Deserialize)]
struct NixOutputsData {
    out: String,
}

fn build_uninstall_system() -> Result<PathBuf> {
    let expr_template = include_str!("../templates/_uninstall.nix")
        .replace("<HOST_PLATFORM>", &format!("{}-darwin", env::consts::ARCH));

    let output = Command::new("nix")
        .args(["build", "--impure", "--json", "--expr"])
        .arg(&expr_template)
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        bail!("failed to build uninstall system with nix");
    }

    let build_data: Vec<NixTopLevelData> = serde_json::from_slice(&output.stdout)?;

    let out = build_data
        .first()
        .ok_or_else(|| eyre!("failed to build uninstall system with nix"))?
        .outputs
        .out
        .parse::<PathBuf>()?;

    Ok(out)
}

#[derive(Parser)]
pub struct UninstallCommand {
    // Use the default choice for confirmation prompts
    #[clap(long)]
    no_confirm: bool,
}

impl super::Command for UninstallCommand {
    fn action(&self, _global_options: &super::Cli) -> Result<()> {
        util::ensure_root();

        util::log::info("building uninstall system");
        let out = build_uninstall_system()?;
        util::log::success(out.to_string_lossy().dimmed());

        if !self.no_confirm
            && !util::log::confirm("are you sure you want to uninstall nix-darwin?", false)?
        {
            return Ok(());
        }

        eprintln!();
        util::log::info("configuring uninstall profile");
        stages::profile("/nix/var/nix/profiles/system", &out)?;

        util::log::info(format!(
            "activating uninstall {}",
            "(activate-user)".dimmed()
        ));
        stages::activate_user(&out)?;
        eprintln!();

        util::log::info(format!("activating uninstall {}", "(activate)".dimmed()));
        stages::activate(&out)?;

        util::log::info("removing residual files");

        util::safe_remove_file("/Applications/Nix Apps")?;
        util::safe_remove_file("/etc/static")?;

        util::log::info("recovering nix daemon into launchctl");

        if PathBuf::from("/nix/store").is_dir() {
            let nix_daemon = PathBuf::from("/Library/LaunchDaemons/org.nixos.nix-daemon.plist");
            if !nix_daemon.exists() {
                util::safe_remove_file(&nix_daemon)?;

                if !Command::new("launchctl")
                    .args(["remove", "org.nixos.nix-daemon"])
                    .stdout(Stdio::null())
                    .status()?
                    .success()
                {
                    bail!("failed to remove `org.nixos.nix-daemon` from launchctl");
                }

                match fs::copy("/nix/var/nix/profiles/default/Library/LaunchDaemons/org.nixos.nix-daemon.plist", &nix_daemon) {
                    Ok(_) => {
                        if !Command::new("launchctl")
                            .args(["load", "-w"])
                            .arg(&nix_daemon)
                            .stdout(Stdio::null())
                            .status()?
                            .success()
                        {
                            bail!("failed to load `org.nixos.nix-daemon` into launchctl");
                        }
                    },

                    Err(err) => {
                        match err.kind() {
                            ErrorKind::NotFound => {
                                util::log::warn("could not recover nix daemon from nix-installer install!");
                            },
                            _ => return Err(err.into()),
                        }
                    }
                }
            }
        }

        util::log::info("removing /run/current-system symlink");

        let current_system_path = PathBuf::from("/run/current-system");
        if current_system_path.is_symlink() {
            util::safe_remove_file(&current_system_path)?;
        }

        util::log::info("restoring /etc files before nix-darwin");

        for (from, to) in fs::read_dir("/etc")?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter_map(|entry| {
                if !entry.is_file() {
                    return None;
                }

                entry.file_name().and_then(|f| {
                    f.to_str().and_then(|f| {
                        f.strip_suffix(".before-nix-darwin")
                            .map(|f| (entry.clone(), PathBuf::from("/etc").join(f)))
                            .filter(|(_, n)| !n.exists())
                    })
                })
            })
        {
            fs::rename(&from, &to)?;

            util::log::info(format!(
                "{} {} {}",
                from.display(),
                "=>".dimmed(),
                to.display()
            ));
        }

        util::log::success("nix-darwin has been uninstalled!");

        Ok(())
    }
}
