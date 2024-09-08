use clap_complete::{generate, Shell};
use std::io::stdout;

use clap::{CommandFactory, Parser};
use eyre::Result;

#[derive(Parser)]
pub struct CompletionCommand {
    /// Shell
    #[arg(value_enum)]
    shell: Shell,
}

impl super::Command for CompletionCommand {
    fn action(&self, _global_options: &super::Cli) -> Result<()> {
        let cmd = &mut super::Cli::command();
        generate(self.shell, cmd, cmd.get_name().to_string(), &mut stdout());

        Ok(())
    }
}
