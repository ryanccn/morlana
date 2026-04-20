// SPDX-FileCopyrightText: 2025 Ryan Cao <hello@ryanccn.dev>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    fmt, fs, io,
    path::Path,
    process::{Command, Stdio},
};

use eyre::{Result, eyre};
use owo_colors::OwoColorize as _;

use crate::util::CommandExt as _;

struct WriteFmt<W: io::Write>(W);

impl<W: io::Write> fmt::Write for WriteFmt<W> {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.0.write_all(string.as_bytes()).map_err(|_| fmt::Error)
    }
}

pub fn dix(out: &Path) -> Result<()> {
    if !Path::new("/run/current-system").exists() {
        return Ok(());
    }

    let old_path = fs::canonicalize("/run/current-system")?;
    let new_path = out.to_path_buf();

    println!(
        "{} {}",
        "<<<".bold().red().dimmed(),
        old_path.display().red()
    );
    println!(
        "{} {}",
        ">>>".bold().green().dimmed(),
        new_path.display().green()
    );

    let closure_size_handle = dix::spawn_size_diff(old_path.clone(), new_path.clone(), true);

    let wrote_diff =
        dix::write_package_diff(&mut WriteFmt(io::stdout()), &old_path, &new_path, true)
            .map_err(|e| eyre!(Box::new(e)))?;

    if wrote_diff > 0 {
        println!();
    }

    let (size_old, size_new) = closure_size_handle
        .join()
        .map_err(|_| eyre!("failed to calculate closure sizes with dix"))?
        .map_err(|e| eyre!(Box::new(e)))?;

    dix::write_size_diff(&mut WriteFmt(io::stdout()), size_old, size_new)?;

    Ok(())
}

pub fn nvd(out: &Path) -> Result<()> {
    if !Path::new("/run/current-system").exists() {
        return Ok(());
    }

    Command::new("nvd")
        .args(["diff", "/run/current-system"])
        .arg(out)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .error_for_status("failed to diff with nvd")?;

    Ok(())
}
