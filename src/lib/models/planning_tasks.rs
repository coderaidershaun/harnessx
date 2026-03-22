//! Planning tasks for projects.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::errors::{ParserError, ParserResult};
use crate::models::intake_actions::{ActionMode, Complexity};
use crate::models::planning_milestones::MilestoneNote;
use crate::models::project::ProjectRegistry;
use crate::models::status::Status;

const TASKS_FILE: &str = "planning_tasks.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaskTraces {
    pub tags: Vec<String>,
    pub intake_sources: Vec<String>,
    pub output_sources: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: SmolStr,
    pub order: u32,
    pub title: SmolStr,
    pub steps: Vec<String>,
    pub status: Status,
    pub story: String,
    pub depends_on: Vec<String>,
    pub complexity: Complexity,
    pub mode: ActionMode,
    pub skills: Vec<String>,
    pub integration_tests: Vec<String>,
    pub traces: TaskTraces,
    #[serde(default)]
    pub notes: Option<Vec<MilestoneNote>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TasksFile {
    pub tasks: Vec<Task>,
}

fn file_path(project_id: &str) -> String {
    format!("harnessx/{project_id}/planning/{TASKS_FILE}")
}

/// Returns `"task-{max+1}"`, or `"task-1"` if the list is empty.
pub fn next_id(items: &[Task]) -> SmolStr {
    let max_existing = items
        .iter()
        .filter_map(|item| item.id.strip_prefix("task-")?.parse::<u64>().ok())
        .max()
        .unwrap_or(0);
    SmolStr::new(format!("task-{}", max_existing + 1))
}

/// Returns the next order value (max existing + 1, or 1 if empty).
pub fn next_order(items: &[Task]) -> u32 {
    items.iter().map(|t| t.order).max().unwrap_or(0) + 1
}

pub fn load(project_id: &str) -> ParserResult<Vec<Task>> {
    let contents = fs::read_to_string(file_path(project_id))?;
    let file: TasksFile = serde_json::from_str(&contents)?;
    Ok(file.tasks)
}

pub fn load_or_default(project_id: &str) -> Vec<Task> {
    load(project_id).unwrap_or_default()
}

pub fn save(items: &[Task], project_id: &str) -> ParserResult<()> {
    let path = file_path(project_id);
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }
    let file = TasksFile {
        tasks: items.to_vec(),
    };
    fs::write(&path, serde_json::to_string_pretty(&file)?)?;
    Ok(())
}

pub fn for_active_project() -> ParserResult<Vec<Task>> {
    let registry = ProjectRegistry::load_or_default()?;
    let id = registry.active_project_id()?;

    if !Path::new(&file_path(id)).exists() {
        return Err(ParserError::PlanningTasksNotFound(id.to_string()));
    }

    load(id)
}

pub fn save_for_active_project(items: &[Task]) -> ParserResult<()> {
    let registry = ProjectRegistry::load_or_default()?;
    let id = registry.active_project_id()?;
    save(items, id)
}
