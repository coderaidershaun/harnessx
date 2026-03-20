//! Intake action items for projects.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::errors::{ParserError, ParserResult};
use crate::models::project::ProjectRegistry;

const ACTIONS_FILE: &str = "intake_actions.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Complexity {
    SuperLow,
    Low,
    Medium,
    High,
    SuperHigh,
    Uncertain,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum ActionMode {
    Plan,
    Execute,
    Review,
    Rework,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Note {
    #[serde(alias = "agent")]
    pub author: SmolStr,
    pub note: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActionItem {
    pub id: SmolStr,
    pub title: SmolStr,
    pub category: String,
    pub origin: String,
    pub detail: String,
    pub tags: Vec<String>,
    pub input_docs: Vec<String>,
    pub complexity: Complexity,
    pub mode: ActionMode,
    #[serde(default)]
    pub notes: Option<Vec<Note>>,
}

fn file_path(project_id: &str) -> String {
    format!("harnessx/{project_id}/intake/{ACTIONS_FILE}")
}

/// Returns `max(numeric ids) + 1`, or `"1"` if the list is empty.
pub fn next_id(items: &[ActionItem]) -> SmolStr {
    let max_existing = items
        .iter()
        .filter_map(|item| item.id.parse::<u64>().ok())
        .max()
        .unwrap_or(0);
    SmolStr::new((max_existing + 1).to_string())
}

pub fn load(project_id: &str) -> ParserResult<Vec<ActionItem>> {
    let contents = fs::read_to_string(file_path(project_id))?;
    Ok(serde_json::from_str(&contents)?)
}

pub fn load_or_default(project_id: &str) -> Vec<ActionItem> {
    load(project_id).unwrap_or_default()
}

pub fn save(items: &[ActionItem], project_id: &str) -> ParserResult<()> {
    let path = file_path(project_id);
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, serde_json::to_string_pretty(items)?)?;
    Ok(())
}

pub fn for_active_project() -> ParserResult<Vec<ActionItem>> {
    let registry = ProjectRegistry::load_or_default()?;
    let id = registry.active_project_id()?;

    if !Path::new(&file_path(id)).exists() {
        return Err(ParserError::ActionsNotFound(id.to_string()));
    }

    load(id)
}

pub fn save_for_active_project(items: &[ActionItem]) -> ParserResult<()> {
    let registry = ProjectRegistry::load_or_default()?;
    let id = registry.active_project_id()?;
    save(items, id)
}
