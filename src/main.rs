use clap::Parser;
use eyre::Result;

mod cli;
mod stages;
mod util;

use crate::cli::{Cli, Command};

#[cfg(not(target_os = "macos"))]
compile_error!("morlana is not intended for use on non-macOS platforms!");

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();
    cli.command.action(&cli)?;

    Ok(())
}
