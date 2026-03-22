//! Planning milestones for projects.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::errors::{ParserError, ParserResult};
use crate::models::project::ProjectRegistry;
use crate::models::status::Status;

const MILESTONES_FILE: &str = "planning_milestones.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MilestoneNote {
    pub note: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Traces {
    pub tags: Vec<String>,
    pub intake_sources: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Milestone {
    pub id: SmolStr,
    pub order: u32,
    pub title: SmolStr,
    pub description: String,
    pub status: Status,
    pub depends_on: Vec<String>,
    pub success_measures: Vec<String>,
    pub uat_criteria: Vec<String>,
    pub traces: Traces,
    #[serde(default)]
    pub notes: Option<Vec<MilestoneNote>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MilestonesFile {
    pub milestones: Vec<Milestone>,
}

fn file_path(project_id: &str) -> String {
    format!("harnessx/{project_id}/planning/{MILESTONES_FILE}")
}

/// Returns `"milestone-{max+1}"`, or `"milestone-1"` if the list is empty.
pub fn next_id(items: &[Milestone]) -> SmolStr {
    let max_existing = items
        .iter()
        .filter_map(|item| item.id.strip_prefix("milestone-")?.parse::<u64>().ok())
        .max()
        .unwrap_or(0);
    SmolStr::new(format!("milestone-{}", max_existing + 1))
}

/// Returns the next order value (max existing + 1, or 1 if empty).
pub fn next_order(items: &[Milestone]) -> u32 {
    items.iter().map(|m| m.order).max().unwrap_or(0) + 1
}

pub fn load(project_id: &str) -> ParserResult<Vec<Milestone>> {
    let contents = fs::read_to_string(file_path(project_id))?;
    let file: MilestonesFile = serde_json::from_str(&contents)?;
    Ok(file.milestones)
}

pub fn load_or_default(project_id: &str) -> Vec<Milestone> {
    load(project_id).unwrap_or_default()
}

pub fn save(items: &[Milestone], project_id: &str) -> ParserResult<()> {
    let path = file_path(project_id);
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }
    let file = MilestonesFile {
        milestones: items.to_vec(),
    };
    fs::write(&path, serde_json::to_string_pretty(&file)?)?;
    Ok(())
}

pub fn for_active_project() -> ParserResult<Vec<Milestone>> {
    let registry = ProjectRegistry::load_or_default()?;
    let id = registry.active_project_id()?;

    if !Path::new(&file_path(id)).exists() {
        return Err(ParserError::MilestonesNotFound(id.to_string()));
    }

    load(id)
}

pub fn save_for_active_project(items: &[Milestone]) -> ParserResult<()> {
    let registry = ProjectRegistry::load_or_default()?;
    let id = registry.active_project_id()?;
    save(items, id)
}
