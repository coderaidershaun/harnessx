//! Planning tasks for projects.
//!
//! Tasks are sharded on disk by epic and story:
//! `harnessx/{project}/planning/tasks/{epic-id}/{story-id}/planning_tasks.json`

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::errors::ParserResult;
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
    #[serde(default)]
    pub epic: String,
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

// ---------------------------------------------------------------------------
// Path helpers
// ---------------------------------------------------------------------------

/// Strips a leading `#` from a reference (e.g. `"#epic-1"` → `"epic-1"`).
fn strip_hash(s: &str) -> &str {
    s.strip_prefix('#').unwrap_or(s)
}

/// Legacy flat-file path (pre-sharding).
fn legacy_file_path(project_id: &str) -> String {
    format!("harnessx/{project_id}/planning/{TASKS_FILE}")
}

/// Root directory for sharded task files.
fn tasks_dir(project_id: &str) -> String {
    format!("harnessx/{project_id}/planning/tasks")
}

/// Shard file path for a specific epic/story combination.
fn shard_file_path(project_id: &str, epic_id: &str, story_id: &str) -> String {
    let epic = strip_hash(epic_id);
    let story = strip_hash(story_id);
    format!(
        "harnessx/{project_id}/planning/tasks/{epic}/{story}/{TASKS_FILE}"
    )
}

// ---------------------------------------------------------------------------
// ID / order helpers
// ---------------------------------------------------------------------------

/// Returns `"task-{max+1}"`, or `"task-1"` if the list is empty.
/// Callers must pass the full aggregated task list (all shards) for global uniqueness.
pub fn next_id(items: &[Task]) -> SmolStr {
    let max_existing = items
        .iter()
        .filter_map(|item| item.id.strip_prefix("task-")?.parse::<u64>().ok())
        .max()
        .unwrap_or(0);
    SmolStr::new(format!("task-{}", max_existing + 1))
}

/// Returns the next order value (max existing + 1, or 1 if empty).
/// Callers must pass the full aggregated task list (all shards) for global ordering.
pub fn next_order(items: &[Task]) -> u32 {
    items.iter().map(|t| t.order).max().unwrap_or(0) + 1
}

// ---------------------------------------------------------------------------
// Shard load / save
// ---------------------------------------------------------------------------

/// Load tasks from a specific shard (epic + story combination).
pub fn load_shard(project_id: &str, epic_id: &str, story_id: &str) -> ParserResult<Vec<Task>> {
    let path = shard_file_path(project_id, epic_id, story_id);
    if !Path::new(&path).exists() {
        return Ok(Vec::new());
    }
    let contents = fs::read_to_string(&path)?;
    let file: TasksFile = serde_json::from_str(&contents)?;
    Ok(file.tasks)
}

/// Save tasks to a specific shard (epic + story combination).
pub fn save_shard(
    items: &[Task],
    project_id: &str,
    epic_id: &str,
    story_id: &str,
) -> ParserResult<()> {
    let path = shard_file_path(project_id, epic_id, story_id);
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }
    let file = TasksFile {
        tasks: items.to_vec(),
    };
    fs::write(&path, serde_json::to_string_pretty(&file)?)?;
    Ok(())
}

/// Load ALL tasks across every shard. Returns empty vec if no shards exist.
pub fn load_all_shards(project_id: &str) -> ParserResult<Vec<Task>> {
    let dir = tasks_dir(project_id);
    if !Path::new(&dir).exists() {
        return Ok(Vec::new());
    }

    let mut all_tasks = Vec::new();

    for epic_entry in fs::read_dir(&dir)? {
        let epic_entry = epic_entry?;
        if !epic_entry.file_type()?.is_dir() {
            continue;
        }
        for story_entry in fs::read_dir(epic_entry.path())? {
            let story_entry = story_entry?;
            if !story_entry.file_type()?.is_dir() {
                continue;
            }
            let tasks_file = story_entry.path().join(TASKS_FILE);
            if tasks_file.exists() {
                let contents = fs::read_to_string(&tasks_file)?;
                let file: TasksFile = serde_json::from_str(&contents)?;
                all_tasks.extend(file.tasks);
            }
        }
    }

    Ok(all_tasks)
}

// ---------------------------------------------------------------------------
// Legacy flat-file load / save (backward compatibility)
// ---------------------------------------------------------------------------

/// Load from the legacy flat file. Returns error if file does not exist.
pub fn load_legacy(project_id: &str) -> ParserResult<Vec<Task>> {
    let contents = fs::read_to_string(legacy_file_path(project_id))?;
    let file: TasksFile = serde_json::from_str(&contents)?;
    Ok(file.tasks)
}

// ---------------------------------------------------------------------------
// Active-project convenience functions
// ---------------------------------------------------------------------------

/// Load all tasks for the active project.
///
/// Tries sharded `tasks/` directory first; falls back to the legacy flat file.
/// Returns an error only if neither source exists.
pub fn for_active_project() -> ParserResult<Vec<Task>> {
    let registry = ProjectRegistry::load_or_default()?;
    let id = registry.active_project_id()?;

    // Try sharded directory first.
    let shards = load_all_shards(id)?;
    if !shards.is_empty() {
        return Ok(shards);
    }

    // Fall back to legacy flat file.
    let legacy = legacy_file_path(id);
    if Path::new(&legacy).exists() {
        return load_legacy(id);
    }

    // Neither exists — no tasks yet, return empty vec so callers can
    // proceed (e.g. create the first task).
    Ok(Vec::new())
}

/// Load tasks from a specific shard for the active project.
pub fn load_shard_for_active_project(
    epic_id: &str,
    story_id: &str,
) -> ParserResult<Vec<Task>> {
    let registry = ProjectRegistry::load_or_default()?;
    let id = registry.active_project_id()?;
    load_shard(id, epic_id, story_id)
}

/// Save tasks to a specific shard for the active project.
pub fn save_shard_for_active_project(
    items: &[Task],
    epic_id: &str,
    story_id: &str,
) -> ParserResult<()> {
    let registry = ProjectRegistry::load_or_default()?;
    let id = registry.active_project_id()?;
    save_shard(items, id, epic_id, story_id)
}

pub fn load_or_default(project_id: &str) -> Vec<Task> {
    let shards = load_all_shards(project_id).unwrap_or_default();
    if !shards.is_empty() {
        return shards;
    }
    load_legacy(project_id).unwrap_or_default()
}
