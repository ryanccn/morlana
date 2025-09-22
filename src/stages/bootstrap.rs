use std::{env, fs, path::PathBuf};

use eyre::{Result, bail, eyre};

use crate::util;

fn gen_flake_template() -> Result<String> {
    Ok(include_str!("../templates/_flake.nix")
        .replace("HOSTNAME", &util::hostname()?)
        .replace("<HOST_PLATFORM>", &format!("{}-darwin", env::consts::ARCH))
        .replace(
            "<USER>",
            &uzers::get_current_username()
                .ok_or_else(|| eyre!("could not obtain current username"))?
                .into_string()
                .map_err(|oss| eyre!("current username {oss:?} is not UTF-8"))?,
        ))
}

pub fn bootstrap() -> Result<PathBuf> {
    let flake_template = gen_flake_template()?;

    let bootstrap_dir = env::home_dir()
        .ok_or_else(|| eyre!("could not locate home directory"))?
        .join("flake");

    if bootstrap_dir.exists() {
        bail!("bootstrap directory `~/flake` exists already")
    }

    fs::create_dir_all(&bootstrap_dir)?;
    fs::write(bootstrap_dir.join("flake.nix"), &flake_template)?;

    Ok(bootstrap_dir)
}
