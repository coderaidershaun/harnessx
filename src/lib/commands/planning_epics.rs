//! Planning epic subcommands: create, remove, update, list, next.

use clap::Subcommand;
use smol_str::SmolStr;

use crate::errors::{ParserError, ParserResult};
use crate::models::planning_epics::{self, Epic};
use crate::models::planning_milestones::{MilestoneNote, Traces};
use crate::models::status::Status;
use crate::output::exit_with;

#[derive(Subcommand)]
pub enum PlanningEpicsCommand {
    /// Create a new epic.
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
        milestone: String,
        #[arg(long, default_value = "")]
        depends_on: String,
        #[arg(long, default_value = "")]
        categories: String,
        #[arg(long, default_value = "")]
        trace_tags: String,
        #[arg(long, default_value = "")]
        trace_intake_sources: String,
        #[arg(long)]
        note: Option<String>,
    },
    /// Remove an epic by ID.
    Remove { id: String },
    /// Update fields on an existing epic.
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
        milestone: Option<String>,
        #[arg(long)]
        depends_on: Option<String>,
        #[arg(long)]
        categories: Option<String>,
        #[arg(long)]
        trace_tags: Option<String>,
        #[arg(long)]
        trace_intake_sources: Option<String>,
        #[arg(long)]
        note: Option<String>,
    },
    /// List all epics.
    List,
    /// Show the next incomplete epic (by order).
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

impl PlanningEpicsCommand {
    pub fn run(self) -> ! {
        match self {
            Self::Create {
                title,
                description,
                order,
                status,
                milestone,
                depends_on,
                categories,
                trace_tags,
                trace_intake_sources,
                note,
            } => exit_with(create_epic(
                title,
                description,
                order,
                status,
                milestone,
                depends_on,
                categories,
                trace_tags,
                trace_intake_sources,
                note,
            )),

            Self::Remove { id } => exit_with(remove_epic(&id)),

            Self::Update {
                id,
                title,
                description,
                order,
                status,
                milestone,
                depends_on,
                categories,
                trace_tags,
                trace_intake_sources,
                note,
            } => exit_with(update_epic(
                &id,
                title,
                description,
                order,
                status,
                milestone,
                depends_on,
                categories,
                trace_tags,
                trace_intake_sources,
                note,
            )),

            Self::List => exit_with(planning_epics::for_active_project()),
            Self::Next => exit_with(next_epic()),
        }
    }
}

fn next_epic() -> ParserResult<serde_json::Value> {
    let mut items = planning_epics::for_active_project()?;
    items.sort_by_key(|e| e.order);

    let next = items.into_iter().find(|e| !e.status.is_completed());

    match next {
        Some(epic) => Ok(serde_json::to_value(epic)?),
        None => Ok(serde_json::json!({
            "message": "All epics completed."
        })),
    }
}

#[allow(clippy::too_many_arguments)]
fn create_epic(
    title: String,
    description: String,
    order: Option<u32>,
    status: String,
    milestone: String,
    depends_on: String,
    categories: String,
    trace_tags: String,
    trace_intake_sources: String,
    note: Option<String>,
) -> ParserResult<Epic> {
    let mut items = planning_epics::for_active_project()?;

    let item = Epic {
        id: planning_epics::next_id(&items),
        order: order.unwrap_or_else(|| planning_epics::next_order(&items)),
        title: SmolStr::new(title),
        description,
        status: parse_status(&status)?,
        milestone,
        depends_on: parse_csv(&depends_on),
        categories: parse_csv(&categories),
        traces: Traces {
            tags: parse_csv(&trace_tags),
            intake_sources: parse_csv(&trace_intake_sources),
        },
        notes: note.map(|n| vec![MilestoneNote { note: n }]),
    };

    items.push(item.clone());
    planning_epics::save_for_active_project(&items)?;
    Ok(item)
}

fn remove_epic(id: &str) -> ParserResult<Epic> {
    let mut items = planning_epics::for_active_project()?;

    let position = items
        .iter()
        .position(|item| item.id == id)
        .ok_or_else(|| ParserError::EpicNotFound(id.to_string()))?;

    let removed = items.remove(position);
    planning_epics::save_for_active_project(&items)?;
    Ok(removed)
}

#[allow(clippy::too_many_arguments)]
fn update_epic(
    id: &str,
    title: Option<String>,
    description: Option<String>,
    order: Option<u32>,
    status: Option<String>,
    milestone: Option<String>,
    depends_on: Option<String>,
    categories: Option<String>,
    trace_tags: Option<String>,
    trace_intake_sources: Option<String>,
    note: Option<String>,
) -> ParserResult<Epic> {
    let mut items = planning_epics::for_active_project()?;

    let item = items
        .iter_mut()
        .find(|item| item.id == id)
        .ok_or_else(|| ParserError::EpicNotFound(id.to_string()))?;

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
    if let Some(v) = milestone {
        item.milestone = v;
    }
    if let Some(v) = depends_on {
        item.depends_on = parse_csv(&v);
    }
    if let Some(v) = categories {
        item.categories = parse_csv(&v);
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
    planning_epics::save_for_active_project(&items)?;
    Ok(updated)
}
