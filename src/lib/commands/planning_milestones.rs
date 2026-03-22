//! Planning milestone subcommands: create, remove, update, list, next.

use clap::Subcommand;
use smol_str::SmolStr;

use crate::errors::{ParserError, ParserResult};
use crate::models::planning_milestones::{self, Milestone, MilestoneNote, Traces};
use crate::models::status::Status;
use crate::output::exit_with;

#[derive(Subcommand)]
pub enum PlanningMilestonesCommand {
    /// Create a new milestone.
    Create {
        #[arg(long, default_value = "")]
        title: String,
        #[arg(long, default_value = "")]
        description: String,
        #[arg(long)]
        order: Option<u32>,
        #[arg(long, default_value = "not_started")]
        status: String,
        #[arg(long, default_value = "")]
        depends_on: String,
        #[arg(long, default_value = "")]
        success_measures: String,
        #[arg(long, default_value = "")]
        uat_criteria: String,
        #[arg(long, default_value = "")]
        trace_tags: String,
        #[arg(long, default_value = "")]
        trace_intake_sources: String,
        #[arg(long)]
        note: Option<String>,
    },
    /// Remove a milestone by ID.
    Remove { id: String },
    /// Update fields on an existing milestone.
    Update {
        id: String,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        order: Option<u32>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        depends_on: Option<String>,
        #[arg(long)]
        success_measures: Option<String>,
        #[arg(long)]
        uat_criteria: Option<String>,
        #[arg(long)]
        trace_tags: Option<String>,
        #[arg(long)]
        trace_intake_sources: Option<String>,
        #[arg(long)]
        note: Option<String>,
    },
    /// List all milestones.
    List,
    /// Show the next incomplete milestone (by order).
    Next,
}

/// Splits a comma-separated string into trimmed tokens; returns empty vec for empty input.
fn parse_csv(s: &str) -> Vec<String> {
    if s.is_empty() {
        return Vec::new();
    }
    s.split(',').map(|v| v.trim().to_string()).collect()
}

fn parse_status(s: &str) -> ParserResult<Status> {
    s.parse::<Status>()
        .map_err(|e| ParserError::InvalidEnumValue(e))
}

impl PlanningMilestonesCommand {
    pub fn run(self) -> ! {
        match self {
            Self::Create {
                title,
                description,
                order,
                status,
                depends_on,
                success_measures,
                uat_criteria,
                trace_tags,
                trace_intake_sources,
                note,
            } => exit_with(create_milestone(
                title,
                description,
                order,
                status,
                depends_on,
                success_measures,
                uat_criteria,
                trace_tags,
                trace_intake_sources,
                note,
            )),

            Self::Remove { id } => exit_with(remove_milestone(&id)),

            Self::Update {
                id,
                title,
                description,
                order,
                status,
                depends_on,
                success_measures,
                uat_criteria,
                trace_tags,
                trace_intake_sources,
                note,
            } => exit_with(update_milestone(
                &id,
                title,
                description,
                order,
                status,
                depends_on,
                success_measures,
                uat_criteria,
                trace_tags,
                trace_intake_sources,
                note,
            )),

            Self::List => exit_with(planning_milestones::for_active_project()),
            Self::Next => exit_with(next_milestone()),
        }
    }
}

fn next_milestone() -> ParserResult<serde_json::Value> {
    let mut items = planning_milestones::for_active_project()?;
    items.sort_by_key(|m| m.order);

    let next = items.into_iter().find(|m| !m.status.is_completed());

    match next {
        Some(milestone) => Ok(serde_json::to_value(milestone)?),
        None => Ok(serde_json::json!({
            "message": "All milestones completed."
        })),
    }
}

#[allow(clippy::too_many_arguments)]
fn create_milestone(
    title: String,
    description: String,
    order: Option<u32>,
    status: String,
    depends_on: String,
    success_measures: String,
    uat_criteria: String,
    trace_tags: String,
    trace_intake_sources: String,
    note: Option<String>,
) -> ParserResult<Milestone> {
    let mut items = planning_milestones::for_active_project()?;

    let item = Milestone {
        id: planning_milestones::next_id(&items),
        order: order.unwrap_or_else(|| planning_milestones::next_order(&items)),
        title: SmolStr::new(title),
        description,
        status: parse_status(&status)?,
        depends_on: parse_csv(&depends_on),
        success_measures: parse_csv(&success_measures),
        uat_criteria: parse_csv(&uat_criteria),
        traces: Traces {
            tags: parse_csv(&trace_tags),
            intake_sources: parse_csv(&trace_intake_sources),
        },
        notes: note.map(|n| vec![MilestoneNote { note: n }]),
    };

    items.push(item.clone());
    planning_milestones::save_for_active_project(&items)?;
    Ok(item)
}

fn remove_milestone(id: &str) -> ParserResult<Milestone> {
    let mut items = planning_milestones::for_active_project()?;

    let position = items
        .iter()
        .position(|item| item.id == id)
        .ok_or_else(|| ParserError::MilestoneNotFound(id.to_string()))?;

    let removed = items.remove(position);
    planning_milestones::save_for_active_project(&items)?;
    Ok(removed)
}

#[allow(clippy::too_many_arguments)]
fn update_milestone(
    id: &str,
    title: Option<String>,
    description: Option<String>,
    order: Option<u32>,
    status: Option<String>,
    depends_on: Option<String>,
    success_measures: Option<String>,
    uat_criteria: Option<String>,
    trace_tags: Option<String>,
    trace_intake_sources: Option<String>,
    note: Option<String>,
) -> ParserResult<Milestone> {
    let mut items = planning_milestones::for_active_project()?;

    let item = items
        .iter_mut()
        .find(|item| item.id == id)
        .ok_or_else(|| ParserError::MilestoneNotFound(id.to_string()))?;

    if let Some(v) = title {
        item.title = SmolStr::new(v);
    }
    if let Some(v) = description {
        item.description = v;
    }
    if let Some(v) = order {
        item.order = v;
    }
    if let Some(v) = status {
        item.status = parse_status(&v)?;
    }
    if let Some(v) = depends_on {
        item.depends_on = parse_csv(&v);
    }
    if let Some(v) = success_measures {
        item.success_measures = parse_csv(&v);
    }
    if let Some(v) = uat_criteria {
        item.uat_criteria = parse_csv(&v);
    }
    if let Some(v) = trace_tags {
        item.traces.tags = parse_csv(&v);
    }
    if let Some(v) = trace_intake_sources {
        item.traces.intake_sources = parse_csv(&v);
    }
    if let Some(n) = note {
        let new_note = MilestoneNote { note: n };
        match &mut item.notes {
            Some(notes) => notes.push(new_note),
            None => item.notes = Some(vec![new_note]),
        }
    }

    let updated = item.clone();
    planning_milestones::save_for_active_project(&items)?;
    Ok(updated)
}
