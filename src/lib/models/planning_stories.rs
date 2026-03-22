//! Planning stories for projects.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::errors::{ParserError, ParserResult};
use crate::models::planning_milestones::{MilestoneNote, Traces};
use crate::models::project::ProjectRegistry;
use crate::models::status::Status;

const STORIES_FILE: &str = "planning_stories.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Story {
    pub id: SmolStr,
    pub order: u32,
    pub title: SmolStr,
    pub description: String,
    pub status: Status,
    pub epic: String,
    pub depends_on: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub traces: Traces,
    #[serde(default)]
    pub tasks_written: bool,
    #[serde(default)]
    pub tasks_completed: bool,
    #[serde(default)]
    pub notes: Option<Vec<MilestoneNote>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StoriesFile {
    pub stories: Vec<Story>,
}

fn file_path(project_id: &str) -> String {
    format!("harnessx/{project_id}/planning/{STORIES_FILE}")
}

/// Returns `"story-{max+1}"`, or `"story-1"` if the list is empty.
pub fn next_id(items: &[Story]) -> SmolStr {
    let max_existing = items
        .iter()
        .filter_map(|item| item.id.strip_prefix("story-")?.parse::<u64>().ok())
        .max()
        .unwrap_or(0);
    SmolStr::new(format!("story-{}", max_existing + 1))
}

/// Returns the next order value (max existing + 1, or 1 if empty).
pub fn next_order(items: &[Story]) -> u32 {
    items.iter().map(|s| s.order).max().unwrap_or(0) + 1
}

pub fn load(project_id: &str) -> ParserResult<Vec<Story>> {
    let contents = fs::read_to_string(file_path(project_id))?;
    let file: StoriesFile = serde_json::from_str(&contents)?;
    Ok(file.stories)
}

pub fn load_or_default(project_id: &str) -> Vec<Story> {
    load(project_id).unwrap_or_default()
}

pub fn save(items: &[Story], project_id: &str) -> ParserResult<()> {
    let path = file_path(project_id);
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }
    let file = StoriesFile {
        stories: items.to_vec(),
    };
    fs::write(&path, serde_json::to_string_pretty(&file)?)?;
    Ok(())
}

pub fn for_active_project() -> ParserResult<Vec<Story>> {
    let registry = ProjectRegistry::load_or_default()?;
    let id = registry.active_project_id()?;

    if !Path::new(&file_path(id)).exists() {
        return Err(ParserError::StoriesNotFound(id.to_string()));
    }

    load(id)
}

pub fn save_for_active_project(items: &[Story]) -> ParserResult<()> {
    let registry = ProjectRegistry::load_or_default()?;
    let id = registry.active_project_id()?;
    save(items, id)
}
