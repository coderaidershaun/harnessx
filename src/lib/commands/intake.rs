//! Intake subcommands: init, status, next, complete.

use clap::Subcommand;

use crate::errors::{ParserError, ParserResult};
use crate::models::intake::{IntakeProgress, NextItemResponse, INTAKE_SECTIONS};
use crate::models::status::Status;
use crate::output::exit_with;

#[derive(Subcommand)]
pub enum IntakeCommand {
    /// Initialise intake progress for the active project.
    Init,
    /// Show intake progress for the active project.
    Status,
    /// Show the next incomplete intake section.
    Next,
    /// Mark an intake section as complete.
    Complete { section: String },
}

impl IntakeCommand {
    pub fn run(self) -> ! {
        match self {
            Self::Init => exit_with(IntakeProgress::init_for_active_project()),
            Self::Status => exit_with(IntakeProgress::for_active_project()),
            Self::Next => exit_with(next_incomplete_section()),
            Self::Complete { section } => exit_with(complete_section(&section)),
        }
    }
}

fn next_incomplete_section() -> ParserResult<serde_json::Value> {
    let progress = IntakeProgress::for_active_project()?;

    let Some(name) = progress.next_item() else {
        return Ok(serde_json::json!({
            "message": "Intake fully completed. Confirm for user and stop agent."
        }));
    };

    let (_, item) = progress
        .items()
        .into_iter()
        .find(|(n, _)| *n == name)
        .expect("next_item returned a name not in items()");

    Ok(serde_json::to_value(NextItemResponse {
        section: name.to_string(),
        agent: item.agent.clone(),
        skills: item.skills.clone(),
    })?)
}

fn complete_section(section: &str) -> ParserResult<IntakeProgress> {
    if !INTAKE_SECTIONS.contains(&section) {
        return Err(ParserError::IntakeNotFound(format!(
            "unknown section '{section}'"
        )));
    }

    let mut progress = IntakeProgress::for_active_project()?;

    let item = progress
        .item_mut(section)
        .expect("section was validated against INTAKE_SECTIONS");
    item.status = Status::Completed;

    progress.save_for_active_project()?;
    Ok(progress)
}
