use eyre::{Report, Result};

pub trait CommandExt {
    fn error_for_status(&mut self, msg: &str) -> Result<std::process::Output>;
}

impl CommandExt for std::process::Command {
    fn error_for_status(&mut self, msg: &str) -> Result<std::process::Output> {
        let output = self.output()?;

        if output.status.success() {
            Ok(output)
        } else {
            Err(Report::msg(msg.to_owned()))
        }
    }
}
