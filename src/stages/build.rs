use std::{
    env,
    os::unix,
    path::PathBuf,
    process::{Command, Stdio},
};

use eyre::{eyre, Result};

use crate::util::{self, CommandExt as _};

#[derive(serde::Deserialize)]
struct NixTopLevelData {
    outputs: NixOutputsData,
}

#[derive(serde::Deserialize)]
struct NixOutputsData {
    out: String,
}

pub fn build(
    program: &str,
    flake_url: &str,
    flake_attr: &str,
    extra_build_flags: &[String],
    out_link: bool,
) -> Result<PathBuf> {
    let build_output = Command::new(program)
        .args([
            "build",
            "--json",
            "--no-link",
            "--extra-experimental-features",
            "nix-command",
            "--extra-experimental-features",
            "flakes",
        ])
        .args(extra_build_flags)
        .args(["--", &format!("{flake_url}#{flake_attr}.system")])
        .stderr(Stdio::inherit())
        .error_for_status("failed to build system with nix")?;

    let build_data: Vec<NixTopLevelData> = serde_json::from_slice(&build_output.stdout)?;

    let out = build_data
        .first()
        .ok_or_else(|| eyre!("failed to obtain system build data with nix"))?
        .outputs
        .out
        .parse::<PathBuf>()?;

    if out_link {
        let result_link = env::current_dir()?.join("result");
        util::safe_remove_file(&result_link)?;
        unix::fs::symlink(&out, &result_link)?;
    }

    Ok(out)
}
