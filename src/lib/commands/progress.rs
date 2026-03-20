//! Progress subcommands: init, status, next, complete, update.

use clap::Subcommand;

use crate::errors::{ParserError, ParserResult};
use crate::models::progress::{NextStageResponse, ProjectProgress, PROGRESS_STAGES};
use crate::models::status::Status;
use crate::output::exit_with;

#[derive(Subcommand)]
pub enum ProgressCommand {
    /// Initialise progress tracking for the active project.
    Init,
    /// Show progress for the active project.
    Status,
    /// Show the next incomplete stage.
    Next,
    /// Mark a stage as complete.
    Complete { stage: String },
    /// Update a stage's status (`not_started`, `in_progress`, `completed`, `rework`).
    Update { stage: String, status: String },
}

impl ProgressCommand {
    pub fn run(self) -> ! {
        match self {
            Self::Init => exit_with(ProjectProgress::init_for_active_project()),
            Self::Status => exit_with(ProjectProgress::for_active_project()),
            Self::Next => exit_with(next_incomplete_stage()),
            Self::Complete { stage } => exit_with(complete_stage(&stage)),
            Self::Update { stage, status } => exit_with(update_stage(&stage, &status)),
        }
    }
}

fn validate_stage_name(stage: &str) -> ParserResult<()> {
    if PROGRESS_STAGES.contains(&stage) {
        Ok(())
    } else {
        Err(ParserError::ProgressNotFound(format!(
            "unknown stage '{stage}'"
        )))
    }
}

fn next_incomplete_stage() -> ParserResult<serde_json::Value> {
    let progress = ProjectProgress::for_active_project()?;

    let Some(name) = progress.next_stage() else {
        return Ok(serde_json::json!({
            "message": "All stages completed."
        }));
    };

    let (_, stage) = progress
        .items()
        .into_iter()
        .find(|(n, _)| *n == name)
        .expect("next_stage returned a name not in items()");

    Ok(serde_json::to_value(NextStageResponse {
        stage: name.to_string(),
        status: stage.status.clone(),
        agent: stage.agent.clone(),
    })?)
}

fn complete_stage(stage: &str) -> ParserResult<ProjectProgress> {
    validate_stage_name(stage)?;
    let mut progress = ProjectProgress::for_active_project()?;

    let item = progress
        .item_mut(stage)
        .expect("stage was validated against PROGRESS_STAGES");
    item.status = Status::Completed;

    progress.save_for_active_project()?;
    Ok(progress)
}

fn update_stage(stage: &str, status_str: &str) -> ParserResult<ProjectProgress> {
    validate_stage_name(stage)?;

    let new_status: Status = status_str
        .parse()
        .map_err(ParserError::ProgressNotFound)?;

    let mut progress = ProjectProgress::for_active_project()?;

    let item = progress
        .item_mut(stage)
        .expect("stage was validated against PROGRESS_STAGES");
    item.status = new_status;

    progress.save_for_active_project()?;
    Ok(progress)
}
