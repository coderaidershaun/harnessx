//! Completion percentage for a project.

use clap::Args;
use serde::Serialize;

use crate::errors::ParserResult;
use crate::models::intake_completion::IntakeCompletionProgress;
use crate::models::intake_onboarding::IntakeOnboardingProgress;
use crate::models::intake_team::IntakeTeamProgress;
use crate::models::planning::PlanningProgress;
use crate::models::planning_tasks;
use crate::models::progress::ProjectProgress;
use crate::models::status::Status;
use crate::output::exit_with;

#[derive(Args)]
pub struct CompletionArgs {
    /// The project ID to check completion for.
    pub project_id: String,
}

#[derive(Serialize)]
pub struct CompletionResponse {
    pub phase: String,
    pub completed: usize,
    pub total: usize,
    pub percentage: String,
}

impl CompletionArgs {
    pub fn run(self) -> ! {
        exit_with(compute_completion(&self.project_id))
    }
}

fn compute_completion(project_id: &str) -> ParserResult<CompletionResponse> {
    let progress = ProjectProgress::load(project_id)?;

    let in_execution = matches!(
        progress.execution.status,
        Status::InProgress | Status::Completed | Status::Rework
    );

    if in_execution {
        let tasks = planning_tasks::load_or_default(project_id);
        let total = tasks.len();
        let completed = tasks.iter().filter(|t| t.status.is_completed()).count();
        let pct = if total == 0 {
            0.0
        } else {
            (completed as f64 / total as f64) * 100.0
        };

        Ok(CompletionResponse {
            phase: "execution".into(),
            completed,
            total,
            percentage: format!("{pct:.1}%"),
        })
    } else {
        let mut completed = 0usize;
        let mut total = 0usize;

        // user_input_required — single item, no sub-sections
        total += 1;
        if progress.user_input_required.status.is_completed() {
            completed += 1;
        }

        // intake_onboarding — 6 sub-sections
        let onboarding = IntakeOnboardingProgress::load_or_default(project_id);
        for (_, item) in onboarding.items() {
            total += 1;
            if item.status.is_completed() {
                completed += 1;
            }
        }

        // intake_team — 3 sub-sections
        let team = IntakeTeamProgress::load_or_default(project_id);
        for (_, item) in team.items() {
            total += 1;
            if item.status.is_completed() {
                completed += 1;
            }
        }

        // intake_completion — 3 sub-sections
        let ic = IntakeCompletionProgress::load_or_default(project_id);
        for (_, item) in ic.items() {
            total += 1;
            if item.status.is_completed() {
                completed += 1;
            }
        }

        // planning — 4 sub-sections
        let planning = PlanningProgress::load_or_default(project_id);
        for (_, item) in planning.items() {
            total += 1;
            if item.status.is_completed() {
                completed += 1;
            }
        }

        // review — single item, no sub-sections
        total += 1;
        if progress.review.status.is_completed() {
            completed += 1;
        }

        let pct = if total == 0 {
            0.0
        } else {
            (completed as f64 / total as f64) * 100.0
        };

        Ok(CompletionResponse {
            phase: "pre_execution".into(),
            completed,
            total,
            percentage: format!("{pct:.1}%"),
        })
    }
}
