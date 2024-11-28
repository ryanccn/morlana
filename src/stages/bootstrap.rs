use std::{
    env, fs,
    path::{Path, PathBuf},
};

use eyre::{bail, eyre, Result};

use crate::util;

fn gen_flake_template() -> Result<String> {
    Ok(include_str!("../templates/_flake.nix").replace("HOSTNAME", &util::hostname()?))
}

fn gen_system_template() -> Result<String> {
    let mut template = include_str!("../templates/_bootstrap.nix").to_owned();

    if Path::new("/nix/var/nix/db").exists() {
        template = template.replace("# services.nix-daemon.enable", "services.nix-daemon.enable");
    }

    template = template
        .replace("<HOST_PLATFORM>", &format!("{}-darwin", env::consts::ARCH))
        .replace(
            "<USER>",
            &users::get_current_username()
                .ok_or_else(|| eyre!("could not obtain current username"))?
                .into_string()
                .map_err(|oss| eyre!("current username {oss:?} is not UTF-8"))?,
        );

    Ok(template)
}

pub fn bootstrap() -> Result<PathBuf> {
    let flake_template = gen_flake_template()?;
    let system_template = gen_system_template()?;

    let bootstrap_dir =
        Path::new(&env::var_os("HOME").ok_or_else(|| eyre!("could not locate HOME"))?)
            .join("flake");

    if bootstrap_dir.exists() {
        bail!("bootstrap directory `~/flake` exists already")
    }

    fs::create_dir_all(&bootstrap_dir)?;
    fs::write(bootstrap_dir.join("flake.nix"), &flake_template)?;
    fs::write(bootstrap_dir.join("system.nix"), &system_template)?;

    Ok(bootstrap_dir)
}
