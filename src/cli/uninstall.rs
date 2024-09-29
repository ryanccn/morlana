use std::{
    env, fs,
    io::ErrorKind,
    os::unix,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use walkdir::WalkDir;

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
        .args([
            "build",
            "--impure",
            "--json",
            "--no-link",
            "--extra-experimental-features",
            "nix-command",
            "--extra-experimental-features",
            "flakes",
            "--expr",
        ])
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

fn restore_etc_files() -> Result<()> {
    for (from, to) in WalkDir::new("/etc")
        .into_iter()
        .filter_map(|entry| entry.ok().map(|e| e.path().to_path_buf()))
        .filter(|entry| entry.is_file())
        .filter_map(|entry| {
            entry.file_name().and_then(|name| {
                name.to_string_lossy()
                    .strip_suffix(".before-nix-darwin")
                    .map(|orig_name| {
                        let from = entry.clone();
                        let mut to = entry.clone();
                        to.set_file_name(orig_name);

                        (from, to)
                    })
                    .filter(|(_, n)| !n.exists())
            })
        })
    {
        fs::rename(&from, &to)?;

        util::log::warn(format!(
            "{} {} {}",
            from.display(),
            "=>".dimmed(),
            to.display()
        ));
    }

    Ok(())
}

fn restore_nix_daemon(nix_daemon: &Path) -> Result<()> {
    util::safe_remove_file(nix_daemon)?;

    Command::new("launchctl")
        .args(["remove", "org.nixos.nix-daemon"])
        .stdout(Stdio::null())
        .status()?;

    match fs::copy(
        "/nix/var/nix/profiles/default/Library/LaunchDaemons/org.nixos.nix-daemon.plist",
        nix_daemon,
    ) {
        Ok(_) => {
            if !Command::new("launchctl")
                .args(["load", "-w"])
                .arg(nix_daemon)
                .stdout(Stdio::null())
                .status()?
                .success()
            {
                bail!("failed to load `org.nixos.nix-daemon` into launchctl");
            }
        }

        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                util::log::warn("could not restore nix daemon from nix-installer install!");
            }
            _ => return Err(err.into()),
        },
    }

    Ok(())
}

fn restore_ca_bundle(ca_bundle: &Path) -> Result<()> {
    match unix::fs::symlink(
        "/nix/var/nix/profiles/default/etc/ssl/certs/ca-bundle.crt",
        ca_bundle,
    ) {
        Ok(()) => {}

        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                util::log::warn("could not restore CA bundle from nix-installer install!");
            }
            _ => return Err(err.into()),
        },
    }

    Ok(())
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

        util::log::info(format!("activating uninstall {}", "(activate)".dimmed()));
        stages::activate(&out)?;

        util::log::info("removing residual files");

        util::safe_remove_file("/Applications/Nix Apps")?;
        util::safe_remove_file("/etc/static")?;

        util::log::info("restoring /etc files before nix-darwin");
        restore_etc_files()?;

        if PathBuf::from("/nix/store").is_dir() {
            let nix_daemon = PathBuf::from("/Library/LaunchDaemons/org.nixos.nix-daemon.plist");

            if !nix_daemon.exists() {
                util::log::info("restoring nix daemon into launchctl");
                restore_nix_daemon(&nix_daemon)?;
            }
        }

        let ca_bundle = PathBuf::from("/etc/ssl/certs/ca-certificates.crt");
        if !ca_bundle.exists() {
            util::log::info("restoring certificate authority bundle");
            restore_ca_bundle(&ca_bundle)?;
        }

        util::log::info("removing /run/current-system symlink");

        let current_system_path = PathBuf::from("/run/current-system");
        if current_system_path.is_symlink() {
            util::safe_remove_file(&current_system_path)?;
        }

        util::log::success("nix-darwin has been uninstalled!");

        Ok(())
    }
}
