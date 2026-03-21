//! Intake onboarding subcommands: init, status, next, complete, update, list.

use clap::Subcommand;

use crate::errors::{ParserError, ParserResult};
use crate::models::intake_onboarding::{IntakeOnboardingProgress, NextItemResponse, INTAKE_ONBOARDING_SECTIONS};
use crate::models::progress::ProjectProgress;
use crate::models::status::Status;
use crate::output::exit_with;

#[derive(Subcommand)]
pub enum IntakeOnboardingCommand {
    /// Initialise intake onboarding progress for the active project.
    Init,
    /// Show intake onboarding progress for the active project.
    Status,
    /// List all intake onboarding sections with their current status.
    List,
    /// Show the next incomplete intake onboarding section.
    Next,
    /// Mark an intake onboarding section as complete.
    Complete { section: String },
    /// Update a section's status (`not_started`, `in_progress`, `completed`, `rework`).
    Update { section: String, status: String },
}

impl IntakeOnboardingCommand {
    pub fn run(self) -> ! {
        match self {
            Self::Init => exit_with(IntakeOnboardingProgress::init_for_active_project()),
            Self::Status => exit_with(IntakeOnboardingProgress::for_active_project()),
            Self::List => exit_with(list_all_items()),
            Self::Next => exit_with(next_incomplete_section()),
            Self::Complete { section } => exit_with(complete_section(&section)),
            Self::Update { section, status } => exit_with(update_section(&section, &status)),
        }
    }
}

fn list_all_items() -> ParserResult<serde_json::Value> {
    let progress = IntakeOnboardingProgress::for_active_project()?;

    let items: Vec<serde_json::Value> = progress
        .items()
        .into_iter()
        .map(|(name, item)| {
            serde_json::json!({
                "section": name,
                "status": item.status,
                "skills": item.skills,
            })
        })
        .collect();

    Ok(serde_json::json!(items))
}

fn next_incomplete_section() -> ParserResult<serde_json::Value> {
    let progress = IntakeOnboardingProgress::for_active_project()?;

    let Some(name) = progress.next_item() else {
        return Ok(serde_json::json!({
            "message": "Intake onboarding fully completed. Confirm for user and stop agent."
        }));
    };

    let (_, item) = progress
        .items()
        .into_iter()
        .find(|(n, _)| *n == name)
        .expect("next_item returned a name not in items()");

    Ok(serde_json::to_value(NextItemResponse {
        section: name.to_string(),
        skills: item.skills.clone(),
    })?)
}

fn update_section(section: &str, status_str: &str) -> ParserResult<IntakeOnboardingProgress> {
    if !INTAKE_ONBOARDING_SECTIONS.contains(&section) {
        return Err(ParserError::IntakeOnboardingNotFound(format!(
            "unknown section '{section}'"
        )));
    }

    let new_status: Status = status_str
        .parse()
        .map_err(ParserError::IntakeOnboardingNotFound)?;

    let mut progress = IntakeOnboardingProgress::for_active_project()?;

    let item = progress
        .item_mut(section)
        .expect("section was validated against INTAKE_ONBOARDING_SECTIONS");
    item.status = new_status;

    progress.save_for_active_project()?;
    Ok(progress)
}

fn complete_section(section: &str) -> ParserResult<IntakeOnboardingProgress> {
    if !INTAKE_ONBOARDING_SECTIONS.contains(&section) {
        return Err(ParserError::IntakeOnboardingNotFound(format!(
            "unknown section '{section}'"
        )));
    }

    let mut progress = IntakeOnboardingProgress::for_active_project()?;

    let item = progress
        .item_mut(section)
        .expect("section was validated against INTAKE_ONBOARDING_SECTIONS");
    item.status = Status::Completed;

    progress.save_for_active_project()?;

    if progress.next_item().is_none() {
        ProjectProgress::complete_stage_for_active_project("intake_onboarding")?;
    }

    Ok(progress)
}
