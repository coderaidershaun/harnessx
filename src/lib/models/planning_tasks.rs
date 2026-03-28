//! Planning tasks for projects.
//!
//! Supports two storage models:
//!
//! **v2 (current):** Tasks sharded by milestone:
//! `harnessx/{project}/planning/tasks/{milestone-id}/planning_tasks.json`
//!
//! **v1 (legacy):** Tasks sharded by epic and story:
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

    // --- v1 legacy fields (4-level model: milestone → epic → story → task) ---
    #[serde(default)]
    pub epic: String,
    #[serde(default)]
    pub story: String,

    // --- v2 fields (2-level model: milestone → task) ---
    /// Direct parent milestone reference (e.g. "#milestone-1"). Empty for v1 tasks.
    #[serde(default)]
    pub milestone: String,
    /// Lightweight grouping label replacing epics (e.g. "setup", "harness", "ws-market").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// The WHY — explains this task's purpose, replacing story descriptions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    /// Strict execution order within the parent milestone. Lower runs first.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_order: Option<u32>,
    /// Task IDs to execute in the same agent session (e.g. ["#task-2", "#task-3"]).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub batch_with: Vec<String>,

    // --- Common fields ---
    pub depends_on: Vec<String>,
    pub complexity: Complexity,
    pub mode: ActionMode,
    pub skills: Vec<String>,
    pub integration_tests: Vec<String>,
    pub traces: TaskTraces,
    #[serde(default)]
    pub notes: Option<Vec<MilestoneNote>>,
}

impl Task {
    /// Returns `true` if this task uses the v2 model (milestone-direct).
    pub fn is_v2(&self) -> bool {
        !self.milestone.is_empty()
    }
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

/// v1 shard file path: `tasks/{epic}/{story}/planning_tasks.json`
fn shard_file_path(project_id: &str, epic_id: &str, story_id: &str) -> String {
    let epic = strip_hash(epic_id);
    let story = strip_hash(story_id);
    format!(
        "harnessx/{project_id}/planning/tasks/{epic}/{story}/{TASKS_FILE}"
    )
}

/// v2 shard file path: `tasks/{milestone}/planning_tasks.json`
fn milestone_shard_path(project_id: &str, milestone_id: &str) -> String {
    let ms = strip_hash(milestone_id);
    format!(
        "harnessx/{project_id}/planning/tasks/{ms}/{TASKS_FILE}"
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

/// Load tasks from a v1 shard (epic + story combination).
pub fn load_shard(project_id: &str, epic_id: &str, story_id: &str) -> ParserResult<Vec<Task>> {
    let path = shard_file_path(project_id, epic_id, story_id);
    if !Path::new(&path).exists() {
        return Ok(Vec::new());
    }
    let contents = fs::read_to_string(&path)?;
    let file: TasksFile = serde_json::from_str(&contents)?;
    Ok(file.tasks)
}

/// Save tasks to a v1 shard (epic + story combination).
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

/// Load tasks from a v2 milestone shard.
pub fn load_milestone_shard(project_id: &str, milestone_id: &str) -> ParserResult<Vec<Task>> {
    let path = milestone_shard_path(project_id, milestone_id);
    if !Path::new(&path).exists() {
        return Ok(Vec::new());
    }
    let contents = fs::read_to_string(&path)?;
    let file: TasksFile = serde_json::from_str(&contents)?;
    Ok(file.tasks)
}

/// Save tasks to a v2 milestone shard.
pub fn save_milestone_shard(
    items: &[Task],
    project_id: &str,
    milestone_id: &str,
) -> ParserResult<()> {
    let path = milestone_shard_path(project_id, milestone_id);
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }
    let file = TasksFile {
        tasks: items.to_vec(),
    };
    fs::write(&path, serde_json::to_string_pretty(&file)?)?;
    Ok(())
}

/// Recursively collect tasks from all `planning_tasks.json` files under a directory.
fn collect_tasks_recursive(dir: &Path, tasks: &mut Vec<Task>) -> ParserResult<()> {
    let tasks_file = dir.join(TASKS_FILE);
    if tasks_file.exists() {
        let contents = fs::read_to_string(&tasks_file)?;
        let file: TasksFile = serde_json::from_str(&contents)?;
        tasks.extend(file.tasks);
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            collect_tasks_recursive(&entry.path(), tasks)?;
        }
    }
    Ok(())
}

/// Load ALL tasks across every shard (handles both v1 and v2 directory structures).
pub fn load_all_shards(project_id: &str) -> ParserResult<Vec<Task>> {
    let dir = tasks_dir(project_id);
    if !Path::new(&dir).exists() {
        return Ok(Vec::new());
    }

    let mut all_tasks = Vec::new();
    collect_tasks_recursive(Path::new(&dir), &mut all_tasks)?;
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

/// Load tasks from a v1 shard for the active project.
pub fn load_shard_for_active_project(
    epic_id: &str,
    story_id: &str,
) -> ParserResult<Vec<Task>> {
    let registry = ProjectRegistry::load_or_default()?;
    let id = registry.active_project_id()?;
    load_shard(id, epic_id, story_id)
}

/// Save tasks to a v1 shard for the active project.
pub fn save_shard_for_active_project(
    items: &[Task],
    epic_id: &str,
    story_id: &str,
) -> ParserResult<()> {
    let registry = ProjectRegistry::load_or_default()?;
    let id = registry.active_project_id()?;
    save_shard(items, id, epic_id, story_id)
}

/// Load tasks from a v2 milestone shard for the active project.
pub fn load_milestone_shard_for_active_project(
    milestone_id: &str,
) -> ParserResult<Vec<Task>> {
    let registry = ProjectRegistry::load_or_default()?;
    let id = registry.active_project_id()?;
    load_milestone_shard(id, milestone_id)
}

/// Save tasks to a v2 milestone shard for the active project.
pub fn save_milestone_shard_for_active_project(
    items: &[Task],
    milestone_id: &str,
) -> ParserResult<()> {
    let registry = ProjectRegistry::load_or_default()?;
    let id = registry.active_project_id()?;
    save_milestone_shard(items, id, milestone_id)
}

pub fn load_or_default(project_id: &str) -> Vec<Task> {
    let shards = load_all_shards(project_id).unwrap_or_default();
    if !shards.is_empty() {
        return shards;
    }
    load_legacy(project_id).unwrap_or_default()
}
