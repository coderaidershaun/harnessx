//! `harnessx autoloop` — autonomous session loop for fully-autonomous pipeline stages.
//!
//! Polls for live Claude Code sessions tagged with the active project ID.
//! When no session is running, launches `harnessx autorun`. Exits once the
//! project leaves an autonomous stage (planning, review, execution, uat_rework).

use std::path::Path;
use std::process::{self, Command};
use std::thread;
use std::time::Duration;

use serde::Serialize;

use crate::errors::{ParserError, ParserResult};
use crate::models::progress::ProjectProgress;
use crate::models::project::ProjectRegistry;
use crate::output::exit_with;

use super::session;

const AUTONOMOUS_STAGES: [&str; 4] = ["planning", "review", "execution", "uat_rework"];
const POLL_INTERVAL: Duration = Duration::from_secs(30);

#[derive(clap::Args)]
pub struct AutoloopArgs {
    /// Extra arguments forwarded to the `claude` command during autorun.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    extra: Vec<String>,
}

#[derive(Serialize)]
struct AutoloopResult {
    message: String,
    project_id: String,
    runs: u32,
}

impl AutoloopArgs {
    pub fn run(self) -> ! {
        if let Err(e) = require_harnessx_dir() {
            exit_with::<()>(Err(e));
        }
        exit_with(run_loop(&self.extra))
    }
}

fn require_harnessx_dir() -> ParserResult<()> {
    if !Path::new("harnessx").is_dir() {
        return Err(ParserError::HarnessNotInitialised);
    }
    Ok(())
}

fn current_autonomous_stage() -> ParserResult<Option<String>> {
    let progress = ProjectProgress::for_active_project()?;
    match progress.next_stage() {
        Some(stage) if AUTONOMOUS_STAGES.contains(&stage) => Ok(Some(stage.to_string())),
        _ => Ok(None),
    }
}

fn run_loop(extra: &[String]) -> ParserResult<AutoloopResult> {
    let registry = ProjectRegistry::load_or_default()?;
    let project_id = registry.active_project_id()?.to_string();

    let Some(stage) = current_autonomous_stage()? else {
        return Ok(AutoloopResult {
            message: "Autoloop exited: project is not in an autonomous stage \
                      (planning, review, execution, uat_rework)."
                .into(),
            project_id,
            runs: 0,
        });
    };

    eprintln!("[autoloop] Project '{project_id}' in '{stage}' stage. Starting loop.");

    let mut runs: u32 = 0;

    loop {
        let sessions = session::find_live_sessions(&project_id)?;

        if !sessions.is_empty() {
            eprintln!(
                "[autoloop] Live session detected (pid: {}). Waiting {}s...",
                sessions[0].pid,
                POLL_INTERVAL.as_secs()
            );
            thread::sleep(POLL_INTERVAL);
            continue;
        }

        eprintln!("[autoloop] No live session. Launching autorun...");
        runs += 1;

        let mut cmd = Command::new("claude");
        cmd.args([
            "--dangerously-skip-permissions",
            "-p",
            "/hx:operator",
            "--output-format",
            "json",
        ]);
        cmd.args(extra);
        cmd.stdin(process::Stdio::inherit())
            .stdout(process::Stdio::inherit())
            .stderr(process::Stdio::inherit());

        match cmd.status() {
            Ok(status) => {
                eprintln!(
                    "[autoloop] Autorun exited with code {}.",
                    status.code().unwrap_or(-1)
                );
            }
            Err(e) => {
                eprintln!("[autoloop] Failed to launch claude: {e}");
                return Err(ParserError::Io(e));
            }
        }

        match current_autonomous_stage()? {
            Some(stage) => {
                eprintln!("[autoloop] Still in '{stage}' stage. Continuing loop.");
            }
            None => {
                return Ok(AutoloopResult {
                    message: format!(
                        "Autoloop complete: project '{project_id}' exited autonomous stages."
                    ),
                    project_id,
                    runs,
                });
            }
        }
    }
}
