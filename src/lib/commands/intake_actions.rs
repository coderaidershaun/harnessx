//! Intake action subcommands: create, remove, update, list.

use clap::Subcommand;
use smol_str::SmolStr;

use crate::errors::{ParserError, ParserResult};
use crate::models::intake_actions::{self, ActionItem, ActionMode, Complexity, Note};
use crate::output::exit_with;

#[derive(Subcommand)]
pub enum IntakeActionsCommand {
    /// Create a new action item.
    Create {
        #[arg(long, default_value = "")]
        title: String,
        #[arg(long, default_value = "")]
        category: String,
        #[arg(long, default_value = "")]
        origin: String,
        #[arg(long, default_value = "")]
        detail: String,
        #[arg(long, default_value = "")]
        tags: String,
        #[arg(long, default_value = "")]
        input_docs: String,
        #[arg(long, default_value = "")]
        complexity: String,
        #[arg(long, default_value = "")]
        mode: String,
        #[arg(long)]
        note_author: Option<String>,
        #[arg(long)]
        note_text: Option<String>,
    },
    /// Remove an action item by ID.
    Remove { id: String },
    /// Update fields on an existing action item.
    Update {
        id: String,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        category: Option<String>,
        #[arg(long)]
        origin: Option<String>,
        #[arg(long)]
        detail: Option<String>,
        #[arg(long)]
        tags: Option<String>,
        #[arg(long)]
        input_docs: Option<String>,
        #[arg(long)]
        complexity: Option<String>,
        #[arg(long)]
        mode: Option<String>,
        #[arg(long)]
        note_author: Option<String>,
        #[arg(long)]
        note_text: Option<String>,
    },
    /// List all action items.
    List,
    /// Get a single action item by ID.
    Get { id: String },
    /// Append tags to an existing action item without replacing existing ones.
    AddTag {
        id: String,
        #[arg(long)]
        tags: String,
    },
}

/// Splits a comma-separated string into trimmed tokens; returns empty vec for empty input.
fn parse_csv(s: &str) -> Vec<String> {
    if s.is_empty() {
        return Vec::new();
    }
    s.split(',').map(|v| v.trim().to_string()).collect()
}

impl IntakeActionsCommand {
    pub fn run(self) -> ! {
        match self {
            Self::Create {
                title,
                category,
                origin,
                detail,
                tags,
                input_docs,
                complexity,
                mode,
                note_author,
                note_text,
            } => exit_with(create_action(
                title, category, origin, detail, tags, input_docs, complexity, mode, note_author,
                note_text,
            )),

            Self::Remove { id } => exit_with(remove_action(&id)),

            Self::Update {
                id,
                title,
                category,
                origin,
                detail,
                tags,
                input_docs,
                complexity,
                mode,
                note_author,
                note_text,
            } => exit_with(update_action(
                &id, title, category, origin, detail, tags, input_docs, complexity, mode,
                note_author, note_text,
            )),

            Self::List => exit_with(intake_actions::for_active_project()),
            Self::Get { id } => exit_with(get_action(&id)),
            Self::AddTag { id, tags } => exit_with(add_tag_action(&id, tags)),
        }
    }
}

fn add_tag_action(id: &str, tags: String) -> ParserResult<ActionItem> {
    let mut items = intake_actions::for_active_project()?;
    let item = items
        .iter_mut()
        .find(|item| item.id == id)
        .ok_or_else(|| ParserError::ActionItemNotFound(id.to_string()))?;
    let new_tags = parse_csv(&tags);
    for tag in new_tags {
        if !item.tags.contains(&tag) {
            item.tags.push(tag);
        }
    }
    let updated = item.clone();
    intake_actions::save_for_active_project(&items)?;
    Ok(updated)
}

fn get_action(id: &str) -> ParserResult<serde_json::Value> {
    let items = intake_actions::for_active_project()?;
    let item = items
        .into_iter()
        .find(|a| a.id == id)
        .ok_or_else(|| ParserError::ActionItemNotFound(id.to_string()))?;
    Ok(serde_json::to_value(item)?)
}

fn parse_complexity(s: &str) -> ParserResult<Complexity> {
    match s {
        "super-low" => Ok(Complexity::SuperLow),
        "low" => Ok(Complexity::Low),
        "medium" => Ok(Complexity::Medium),
        "high" => Ok(Complexity::High),
        "super-high" => Ok(Complexity::SuperHigh),
        "uncertain" => Ok(Complexity::Uncertain),
        other => Err(ParserError::InvalidEnumValue(format!(
            "invalid complexity: '{other}' (expected super-low, low, medium, high, super-high, uncertain)"
        ))),
    }
}

fn parse_mode(s: &str) -> ParserResult<ActionMode> {
    match s {
        "plan" => Ok(ActionMode::Plan),
        "execute" => Ok(ActionMode::Execute),
        "review" => Ok(ActionMode::Review),
        "rework" => Ok(ActionMode::Rework),
        other => Err(ParserError::InvalidEnumValue(format!(
            "invalid mode: '{other}' (expected plan, execute, review, rework)"
        ))),
    }
}

fn build_note(author: Option<String>, text: Option<String>) -> Option<Vec<Note>> {
    match (author, text) {
        (Some(a), Some(t)) => Some(vec![Note {
            author: SmolStr::new(a),
            note: t,
        }]),
        _ => None,
    }
}

#[allow(clippy::too_many_arguments)]
fn create_action(
    title: String,
    category: String,
    origin: String,
    detail: String,
    tags: String,
    input_docs: String,
    complexity: String,
    mode: String,
    note_author: Option<String>,
    note_text: Option<String>,
) -> ParserResult<ActionItem> {
    let mut items = intake_actions::for_active_project()?;

    let item = ActionItem {
        id: intake_actions::next_id(&items),
        title: SmolStr::new(title),
        category,
        origin,
        detail,
        tags: parse_csv(&tags),
        input_docs: parse_csv(&input_docs),
        complexity: parse_complexity(&complexity)?,
        mode: parse_mode(&mode)?,
        notes: build_note(note_author, note_text),
    };

    items.push(item.clone());
    intake_actions::save_for_active_project(&items)?;
    Ok(item)
}

fn remove_action(id: &str) -> ParserResult<ActionItem> {
    let mut items = intake_actions::for_active_project()?;

    let position = items
        .iter()
        .position(|item| item.id == id)
        .ok_or_else(|| ParserError::ActionItemNotFound(id.to_string()))?;

    let removed = items.remove(position);
    intake_actions::save_for_active_project(&items)?;
    Ok(removed)
}

#[allow(clippy::too_many_arguments)]
fn update_action(
    id: &str,
    title: Option<String>,
    category: Option<String>,
    origin: Option<String>,
    detail: Option<String>,
    tags: Option<String>,
    input_docs: Option<String>,
    complexity: Option<String>,
    mode: Option<String>,
    note_author: Option<String>,
    note_text: Option<String>,
) -> ParserResult<ActionItem> {
    let mut items = intake_actions::for_active_project()?;

    let item = items
        .iter_mut()
        .find(|item| item.id == id)
        .ok_or_else(|| ParserError::ActionItemNotFound(id.to_string()))?;

    if let Some(v) = title {
        item.title = SmolStr::new(v);
    }
    if let Some(v) = category {
        item.category = v;
    }
    if let Some(v) = origin {
        item.origin = v;
    }
    if let Some(v) = detail {
        item.detail = v;
    }
    if let Some(v) = tags {
        item.tags = parse_csv(&v);
    }
    if let Some(v) = input_docs {
        item.input_docs = parse_csv(&v);
    }
    if let Some(v) = complexity {
        item.complexity = parse_complexity(&v)?;
    }
    if let Some(v) = mode {
        item.mode = parse_mode(&v)?;
    }
    if let (Some(author), Some(text)) = (note_author, note_text) {
        let new_note = Note {
            author: SmolStr::new(author),
            note: text,
        };
        match &mut item.notes {
            Some(notes) => notes.push(new_note),
            None => item.notes = Some(vec![new_note]),
        }
    }

    let updated = item.clone();
    intake_actions::save_for_active_project(&items)?;
    Ok(updated)
}
