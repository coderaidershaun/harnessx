//! Intake action subcommands: create, remove, update, list.

use clap::Subcommand;
use smol_str::SmolStr;

use crate::errors::{ParserError, ParserResult};
use crate::models::intake_actions::{self, ActionItem};
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
    },
    /// List all action items.
    List,
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
            } => exit_with(create_action(title, category, origin, detail, tags, input_docs)),

            Self::Remove { id } => exit_with(remove_action(&id)),

            Self::Update {
                id,
                title,
                category,
                origin,
                detail,
                tags,
                input_docs,
            } => exit_with(update_action(&id, title, category, origin, detail, tags, input_docs)),

            Self::List => exit_with(intake_actions::for_active_project()),
        }
    }
}

fn create_action(
    title: String,
    category: String,
    origin: String,
    detail: String,
    tags: String,
    input_docs: String,
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

fn update_action(
    id: &str,
    title: Option<String>,
    category: Option<String>,
    origin: Option<String>,
    detail: Option<String>,
    tags: Option<String>,
    input_docs: Option<String>,
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

    let updated = item.clone();
    intake_actions::save_for_active_project(&items)?;
    Ok(updated)
}
