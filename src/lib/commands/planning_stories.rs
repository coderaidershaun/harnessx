//! Planning story subcommands: create, remove, update, list, next.

use clap::Subcommand;
use smol_str::SmolStr;

use crate::errors::{ParserError, ParserResult};
use crate::models::planning_milestones::{MilestoneNote, Traces};
use crate::models::planning_stories::{self, Story};
use crate::models::status::Status;
use crate::output::exit_with;

#[derive(Subcommand)]
pub enum PlanningStoriesCommand {
    /// Create a new story.
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
        epic: String,
        #[arg(long, default_value = "")]
        depends_on: String,
        #[arg(long, default_value = "")]
        acceptance_criteria: String,
        #[arg(long, default_value = "")]
        trace_tags: String,
        #[arg(long, default_value = "")]
        trace_intake_sources: String,
        #[arg(long)]
        note: Option<String>,
    },
    /// Remove a story by ID.
    Remove { id: String },
    /// Update fields on an existing story.
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
        epic: Option<String>,
        #[arg(long)]
        depends_on: Option<String>,
        #[arg(long)]
        acceptance_criteria: Option<String>,
        #[arg(long)]
        trace_tags: Option<String>,
        #[arg(long)]
        trace_intake_sources: Option<String>,
        #[arg(long)]
        note: Option<String>,
    },
    /// List all stories.
    List,
    /// Show the next incomplete story (by order).
    Next,
}

/// Splits a comma-separated string into trimmed tokens; returns empty vec for empty input.
fn parse_csv(s: &str) -> Vec<String> {
    if s.is_empty() {
        return Vec::new();
    }
    s.split(',').map(|v| v.trim().to_string()).collect()
}

/// Splits a pipe-separated string into trimmed tokens; returns empty vec for empty input.
/// Used for acceptance criteria which may contain commas in their text.
fn parse_pipe(s: &str) -> Vec<String> {
    if s.is_empty() {
        return Vec::new();
    }
    s.split('|').map(|v| v.trim().to_string()).collect()
}

fn parse_status(s: &str) -> ParserResult<Status> {
    s.parse::<Status>()
        .map_err(|e| ParserError::InvalidEnumValue(e))
}

impl PlanningStoriesCommand {
    pub fn run(self) -> ! {
        match self {
            Self::Create {
                title,
                description,
                order,
                status,
                epic,
                depends_on,
                acceptance_criteria,
                trace_tags,
                trace_intake_sources,
                note,
            } => exit_with(create_story(
                title,
                description,
                order,
                status,
                epic,
                depends_on,
                acceptance_criteria,
                trace_tags,
                trace_intake_sources,
                note,
            )),

            Self::Remove { id } => exit_with(remove_story(&id)),

            Self::Update {
                id,
                title,
                description,
                order,
                status,
                epic,
                depends_on,
                acceptance_criteria,
                trace_tags,
                trace_intake_sources,
                note,
            } => exit_with(update_story(
                &id,
                title,
                description,
                order,
                status,
                epic,
                depends_on,
                acceptance_criteria,
                trace_tags,
                trace_intake_sources,
                note,
            )),

            Self::List => exit_with(planning_stories::for_active_project()),
            Self::Next => exit_with(next_story()),
        }
    }
}

fn next_story() -> ParserResult<serde_json::Value> {
    let mut items = planning_stories::for_active_project()?;
    items.sort_by_key(|s| s.order);

    let next = items.into_iter().find(|s| !s.status.is_completed());

    match next {
        Some(story) => Ok(serde_json::to_value(story)?),
        None => Ok(serde_json::json!({
            "message": "All stories completed."
        })),
    }
}

#[allow(clippy::too_many_arguments)]
fn create_story(
    title: String,
    description: String,
    order: Option<u32>,
    status: String,
    epic: String,
    depends_on: String,
    acceptance_criteria: String,
    trace_tags: String,
    trace_intake_sources: String,
    note: Option<String>,
) -> ParserResult<Story> {
    let mut items = planning_stories::for_active_project()?;

    let item = Story {
        id: planning_stories::next_id(&items),
        order: order.unwrap_or_else(|| planning_stories::next_order(&items)),
        title: SmolStr::new(title),
        description,
        status: parse_status(&status)?,
        epic,
        depends_on: parse_csv(&depends_on),
        acceptance_criteria: parse_pipe(&acceptance_criteria),
        traces: Traces {
            tags: parse_csv(&trace_tags),
            intake_sources: parse_csv(&trace_intake_sources),
        },
        notes: note.map(|n| vec![MilestoneNote { note: n }]),
    };

    items.push(item.clone());
    planning_stories::save_for_active_project(&items)?;
    Ok(item)
}

fn remove_story(id: &str) -> ParserResult<Story> {
    let mut items = planning_stories::for_active_project()?;

    let position = items
        .iter()
        .position(|item| item.id == id)
        .ok_or_else(|| ParserError::StoryNotFound(id.to_string()))?;

    let removed = items.remove(position);
    planning_stories::save_for_active_project(&items)?;
    Ok(removed)
}

#[allow(clippy::too_many_arguments)]
fn update_story(
    id: &str,
    title: Option<String>,
    description: Option<String>,
    order: Option<u32>,
    status: Option<String>,
    epic: Option<String>,
    depends_on: Option<String>,
    acceptance_criteria: Option<String>,
    trace_tags: Option<String>,
    trace_intake_sources: Option<String>,
    note: Option<String>,
) -> ParserResult<Story> {
    let mut items = planning_stories::for_active_project()?;

    let item = items
        .iter_mut()
        .find(|item| item.id == id)
        .ok_or_else(|| ParserError::StoryNotFound(id.to_string()))?;

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
    if let Some(v) = epic {
        item.epic = v;
    }
    if let Some(v) = depends_on {
        item.depends_on = parse_csv(&v);
    }
    if let Some(v) = acceptance_criteria {
        item.acceptance_criteria = parse_pipe(&v);
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
    planning_stories::save_for_active_project(&items)?;
    Ok(updated)
}
