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

    let store = dix::Store::open();

    let report = dix::diff_store_snapshots(
        &store
            .query_snapshot_document(&old_path)
            .map_err(|e| eyre!("{e:?}"))?
            .snapshot,
        &store
            .query_snapshot_document(&new_path)
            .map_err(|e| eyre!("{e:?}"))?
            .snapshot,
    );

    dix::write_diff_report(
        &mut WriteFmt(io::stdout()),
        &report,
        dix::RenderOptions::default(),
    )?;

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
