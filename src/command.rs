use std::process::Command;

use miette::miette;
use miette::Context;
use miette::IntoDiagnostic;

pub trait CommandExt {
    /// Run the command, collect stdout, trim whitespace, decode as UTF-8.
    fn stdout_utf8(&mut self) -> miette::Result<String>;

    /// Display the command as a string, suitable for user output.
    ///
    /// Arguments and program names should be quoted with [`shell_words::quote`].
    fn display(&self) -> String;
}

impl CommandExt for Command {
    fn stdout_utf8(&mut self) -> miette::Result<String> {
        let output = self
            .output()
            .into_diagnostic()
            .wrap_err_with(|| format!("Failed to execute: {}", self.display()))?;

        let status = output.status;
        if status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
            Ok(stdout)
        } else {
            let program = self.get_program().to_string_lossy();
            let mut message = format!(
                "{program} failed with exit code {status}: {}",
                self.display()
            );

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stdout = stdout.trim();
            if !stdout.is_empty() {
                message.push_str(&format!("\nStdout: {stdout}"));
            }

            let stderr = String::from_utf8_lossy(&output.stderr);
            let stderr = stderr.trim();
            if !stdout.is_empty() {
                message.push_str(&format!("\nStderr: {stderr}"));
            }

            Err(miette!("{message}"))
        }
    }

    fn display(&self) -> String {
        let program = self.get_program().to_string_lossy();

        let args = self.get_args().map(|arg| arg.to_string_lossy());

        let tokens = std::iter::once(program).chain(args);

        shell_words::join(tokens)
    }
}
