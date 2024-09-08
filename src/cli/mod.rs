use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use eyre::Result;

mod build;
mod completions;
mod init;
mod switch;
mod uninstall;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[enum_dispatch]
pub trait Command {
    fn action(&self, global_options: &Cli) -> Result<()>;
}

#[derive(Subcommand)]
#[enum_dispatch(Command)]
pub enum Commands {
    /// Switch to a nix-darwin configuration
    Switch(switch::SwitchCommand),
    /// Build a nix-darwin configuration
    Build(build::BuildCommand),

    /// Initialize a basic bootstrapped system
    Init(init::InitCommand),
    /// Do not go gentle into that good night
    Uninstall(uninstall::UninstallCommand),

    /// Generate completions
    Completions(completions::CompletionCommand),
}
