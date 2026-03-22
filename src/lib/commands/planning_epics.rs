//! Planning epic subcommands: create, remove, update, list, next, parent, children.

use clap::Subcommand;
use smol_str::SmolStr;

use crate::errors::{ParserError, ParserResult};
use crate::models::planning_epics::{self, Epic};
use crate::models::planning_milestones::{self, MilestoneNote, Traces};
use crate::models::planning_stories;
use crate::models::planning_tasks;
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
    /// Get a single epic by ID.
    Get { id: String },
    /// Show the next incomplete epic (by order).
    Next,
    /// Show the milestone this epic belongs to.
    Parent { id: String },
    /// Show all stories and tasks under an epic.
    Children { id: String },
    /// Mark an epic's stories as written (or unmark with --value false).
    MarkWritten {
        id: String,
        #[arg(long, default_value = "true")]
        value: bool,
    },
    /// Mark an epic's stories as completed (or unmark with --value false).
    MarkCompleted {
        id: String,
        #[arg(long, default_value = "true")]
        value: bool,
    },
    /// Show the next epic whose stories have not been written yet (by order).
    NextToWrite,
    /// Show the next epic whose stories have not been completed yet (by order).
    NextToComplete,
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
            Self::Get { id } => exit_with(get_epic(&id)),
            Self::Next => exit_with(next_epic()),
            Self::Parent { id } => exit_with(epic_parent(&id)),
            Self::Children { id } => exit_with(epic_children(&id)),
            Self::MarkWritten { id, value } => exit_with(mark_stories_written(&id, value)),
            Self::MarkCompleted { id, value } => exit_with(mark_stories_completed(&id, value)),
            Self::NextToWrite => exit_with(next_to_write()),
            Self::NextToComplete => exit_with(next_to_complete()),
        }
    }
}

/// Strips a leading `#` from a reference if present.
fn strip_hash(s: &str) -> &str {
    s.strip_prefix('#').unwrap_or(s)
}

fn get_epic(id: &str) -> ParserResult<serde_json::Value> {
    let items = planning_epics::for_active_project()?;
    let item = items
        .into_iter()
        .find(|e| e.id == id)
        .ok_or_else(|| ParserError::EpicNotFound(id.to_string()))?;
    Ok(serde_json::to_value(item)?)
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

fn epic_parent(id: &str) -> ParserResult<serde_json::Value> {
    let epics = planning_epics::for_active_project()?;
    let epic = epics
        .iter()
        .find(|e| e.id == id)
        .ok_or_else(|| ParserError::EpicNotFound(id.to_string()))?;

    let milestone_id = strip_hash(&epic.milestone);
    let milestones = planning_milestones::for_active_project()?;
    let milestone = milestones
        .into_iter()
        .find(|m| m.id == milestone_id)
        .ok_or_else(|| ParserError::MilestoneNotFound(milestone_id.to_string()))?;

    Ok(serde_json::to_value(milestone)?)
}

fn epic_children(id: &str) -> ParserResult<serde_json::Value> {
    let epics = planning_epics::for_active_project()?;
    if !epics.iter().any(|e| e.id == id) {
        return Err(ParserError::EpicNotFound(id.to_string()));
    }

    let ref_id = format!("#{id}");

    let stories: Vec<_> = planning_stories::for_active_project()
        .unwrap_or_default()
        .into_iter()
        .filter(|s| s.epic == ref_id)
        .collect();

    let story_ids: Vec<String> = stories.iter().map(|s| format!("#{}", s.id)).collect();

    let tasks: Vec<_> = planning_tasks::for_active_project()
        .unwrap_or_default()
        .into_iter()
        .filter(|t| story_ids.contains(&t.story))
        .collect();

    Ok(serde_json::json!({
        "epic": id,
        "stories": serde_json::to_value(&stories)?,
        "tasks": serde_json::to_value(&tasks)?,
    }))
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
        stories_written: false,
        stories_completed: false,
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

fn mark_stories_written(id: &str, value: bool) -> ParserResult<Epic> {
    let mut items = planning_epics::for_active_project()?;
    let item = items
        .iter_mut()
        .find(|item| item.id == id)
        .ok_or_else(|| ParserError::EpicNotFound(id.to_string()))?;

    item.stories_written = value;
    let updated = item.clone();
    planning_epics::save_for_active_project(&items)?;
    Ok(updated)
}

fn mark_stories_completed(id: &str, value: bool) -> ParserResult<Epic> {
    let mut items = planning_epics::for_active_project()?;
    let item = items
        .iter_mut()
        .find(|item| item.id == id)
        .ok_or_else(|| ParserError::EpicNotFound(id.to_string()))?;

    item.stories_completed = value;
    let updated = item.clone();
    planning_epics::save_for_active_project(&items)?;
    Ok(updated)
}

fn next_to_write() -> ParserResult<serde_json::Value> {
    let mut items = planning_epics::for_active_project()?;
    items.sort_by_key(|e| e.order);

    let next = items.into_iter().find(|e| !e.stories_written);

    match next {
        Some(epic) => Ok(serde_json::to_value(epic)?),
        None => Ok(serde_json::json!({
            "message": "All epics have their stories written."
        })),
    }
}

fn next_to_complete() -> ParserResult<serde_json::Value> {
    let mut items = planning_epics::for_active_project()?;
    items.sort_by_key(|e| e.order);

    let next = items.into_iter().find(|e| !e.stories_completed);

    match next {
        Some(epic) => Ok(serde_json::to_value(epic)?),
        None => Ok(serde_json::json!({
            "message": "All epics have their stories completed."
        })),
    }
}
