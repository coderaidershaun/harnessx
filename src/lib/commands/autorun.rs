//! `harnessx autorun` — launch the Claude operator in the current workspace.

use std::path::Path;
use std::process::{self, Command};

use crate::errors::{ParserError, ParserResult};
use crate::output::exit_with;

#[derive(clap::Args)]
pub struct AutorunArgs {
    /// Extra arguments forwarded to the `claude` command.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    extra: Vec<String>,
}

impl AutorunArgs {
    pub fn run(self) -> ! {
        if let Err(e) = require_harnessx_dir() {
            exit_with::<()>(Err(e));
        }

        let mut cmd = Command::new("claude");
        cmd.args(["--dangerously-skip-permissions", "-p", "/hx:operator", "--output-format", "json"]);
        cmd.args(&self.extra);
        cmd.stdin(process::Stdio::inherit())
            .stdout(process::Stdio::inherit())
            .stderr(process::Stdio::inherit());

        match cmd.status() {
            Ok(status) => process::exit(status.code().unwrap_or(1)),
            Err(e) => exit_with::<()>(Err(ParserError::Io(e))),
        }
    }
}

fn require_harnessx_dir() -> ParserResult<()> {
    if !Path::new("harnessx").is_dir() {
        return Err(ParserError::HarnessNotInitialised);
    }
    Ok(())
}
