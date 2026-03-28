//! Planning task subcommands: create, remove, update, list, next, parent.
//!
//! Tasks are sharded on disk by epic + story:
//! `planning/tasks/{epic-id}/{story-id}/planning_tasks.json`

use clap::Subcommand;
use smol_str::SmolStr;

use std::collections::HashSet;

use crate::errors::{ParserError, ParserResult};
use crate::models::intake_actions::{ActionMode, Complexity};
use crate::models::planning_epics;
use crate::models::planning_milestones::{self, MilestoneNote};
use crate::models::planning_stories;
use crate::models::planning_tasks::{self, Task, TaskTraces};
use crate::models::status::Status;
use crate::output::exit_with;

#[derive(Subcommand)]
pub enum PlanningTasksCommand {
    /// Create a new task.
    Create {
        #[arg(long, default_value = "")]
        title: String,
        #[arg(long, default_value = "")]
        steps: String,
        #[arg(long)]
        order: Option<u32>,
        #[arg(long, default_value = "not_started")]
        status: String,
        /// v1 legacy: parent epic reference.
        #[arg(long, default_value = "")]
        epic: String,
        /// v1 legacy: parent story reference.
        #[arg(long, default_value = "")]
        story: String,
        /// v2: direct parent milestone reference (e.g. "#milestone-1").
        #[arg(long, default_value = "")]
        milestone: String,
        /// v2: lightweight grouping label (e.g. "setup", "harness").
        #[arg(long)]
        group: Option<String>,
        /// v2: explains why this task exists.
        #[arg(long)]
        purpose: Option<String>,
        /// v2: strict execution order within the parent milestone.
        #[arg(long)]
        execution_order: Option<u32>,
        /// v2: task IDs to execute in the same agent session (comma-separated).
        #[arg(long, default_value = "")]
        batch_with: String,
        #[arg(long, default_value = "")]
        depends_on: String,
        #[arg(long, default_value = "")]
        complexity: String,
        #[arg(long, default_value = "")]
        mode: String,
        #[arg(long, default_value = "")]
        skills: String,
        #[arg(long, default_value = "")]
        integration_tests: String,
        #[arg(long, default_value = "")]
        trace_tags: String,
        #[arg(long, default_value = "")]
        trace_intake_sources: String,
        #[arg(long, default_value = "")]
        trace_output_sources: String,
        #[arg(long)]
        note: Option<String>,
    },
    /// Remove a task by ID.
    Remove { id: String },
    /// Update fields on an existing task.
    Update {
        id: String,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        steps: Option<String>,
        #[arg(long)]
        order: Option<u32>,
        #[arg(long)]
        status: Option<String>,
        /// v1 legacy: parent story reference.
        #[arg(long)]
        story: Option<String>,
        /// v2: direct parent milestone reference.
        #[arg(long)]
        milestone: Option<String>,
        /// v2: lightweight grouping label.
        #[arg(long)]
        group: Option<String>,
        /// v2: explains why this task exists.
        #[arg(long)]
        purpose: Option<String>,
        /// v2: strict execution order within the parent milestone.
        #[arg(long)]
        execution_order: Option<u32>,
        /// v2: task IDs to execute in the same agent session (comma-separated).
        #[arg(long)]
        batch_with: Option<String>,
        #[arg(long)]
        depends_on: Option<String>,
        #[arg(long)]
        complexity: Option<String>,
        #[arg(long)]
        mode: Option<String>,
        #[arg(long)]
        skills: Option<String>,
        #[arg(long)]
        integration_tests: Option<String>,
        #[arg(long)]
        trace_tags: Option<String>,
        #[arg(long)]
        trace_intake_sources: Option<String>,
        #[arg(long)]
        trace_output_sources: Option<String>,
        #[arg(long)]
        note: Option<String>,
    },
    /// List all tasks. Use --milestone or --group to filter (v2).
    List {
        /// Filter tasks by parent milestone ID.
        #[arg(long)]
        milestone: Option<String>,
        /// Filter tasks by group label.
        #[arg(long)]
        group: Option<String>,
    },
    /// Get a single task by ID.
    Get { id: String },
    /// Show the next ready task (dependency-aware).
    Next,
    /// Show the parent of this task (story for v1, milestone for v2).
    Parent { id: String },
    /// Renumber execution_order for all tasks in a milestone (v2).
    Reorder { milestone_id: String },
}

/// Splits a comma-separated string into trimmed tokens; returns empty vec for empty input.
fn parse_csv(s: &str) -> Vec<String> {
    if s.is_empty() {
        return Vec::new();
    }
    s.split(',').map(|v| v.trim().to_string()).collect()
}

/// Splits a pipe-separated string into trimmed tokens; returns empty vec for empty input.
/// Used for steps and integration tests which may contain commas in their text.
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

impl PlanningTasksCommand {
    pub fn run(self) -> ! {
        match self {
            Self::Create {
                title,
                steps,
                order,
                status,
                epic,
                story,
                milestone,
                group,
                purpose,
                execution_order,
                batch_with,
                depends_on,
                complexity,
                mode,
                skills,
                integration_tests,
                trace_tags,
                trace_intake_sources,
                trace_output_sources,
                note,
            } => exit_with(create_task(
                title,
                steps,
                order,
                status,
                epic,
                story,
                milestone,
                group,
                purpose,
                execution_order,
                batch_with,
                depends_on,
                complexity,
                mode,
                skills,
                integration_tests,
                trace_tags,
                trace_intake_sources,
                trace_output_sources,
                note,
            )),

            Self::Remove { id } => exit_with(remove_task(&id)),

            Self::Update {
                id,
                title,
                steps,
                order,
                status,
                story,
                milestone,
                group,
                purpose,
                execution_order,
                batch_with,
                depends_on,
                complexity,
                mode,
                skills,
                integration_tests,
                trace_tags,
                trace_intake_sources,
                trace_output_sources,
                note,
            } => exit_with(update_task(
                &id,
                title,
                steps,
                order,
                status,
                story,
                milestone,
                group,
                purpose,
                execution_order,
                batch_with,
                depends_on,
                complexity,
                mode,
                skills,
                integration_tests,
                trace_tags,
                trace_intake_sources,
                trace_output_sources,
                note,
            )),

            Self::List { milestone, group } => exit_with(list_tasks(milestone, group)),
            Self::Get { id } => exit_with(get_task(&id)),
            Self::Next => exit_with(next_task()),
            Self::Parent { id } => exit_with(task_parent(&id)),
            Self::Reorder { milestone_id } => exit_with(reorder_tasks(&milestone_id)),
        }
    }
}

/// Strips a leading `#` from a dependency reference if present.
/// `depends_on` values are stored as `"#task-1"` but IDs are `"task-1"`.
fn strip_hash(s: &str) -> &str {
    s.strip_prefix('#').unwrap_or(s)
}

fn get_task(id: &str) -> ParserResult<serde_json::Value> {
    let items = planning_tasks::for_active_project()?;
    let item = items
        .into_iter()
        .find(|t| t.id == id)
        .ok_or_else(|| ParserError::PlanningTaskNotFound(id.to_string()))?;
    Ok(serde_json::to_value(item)?)
}

fn task_parent(id: &str) -> ParserResult<serde_json::Value> {
    let tasks = planning_tasks::for_active_project()?;
    let task = tasks
        .iter()
        .find(|t| t.id == id)
        .ok_or_else(|| ParserError::PlanningTaskNotFound(id.to_string()))?;

    // v2: parent is a milestone
    if task.is_v2() {
        let ms_id = strip_hash(&task.milestone);
        let milestones = planning_milestones::for_active_project()?;
        let ms = milestones
            .into_iter()
            .find(|m| m.id == ms_id)
            .ok_or_else(|| ParserError::MilestoneNotFound(ms_id.to_string()))?;
        return Ok(serde_json::to_value(ms)?);
    }

    // v1: parent is a story
    let story_id = strip_hash(&task.story);
    let stories = planning_stories::for_active_project()?;
    let story = stories
        .into_iter()
        .find(|s| s.id == story_id)
        .ok_or_else(|| ParserError::StoryNotFound(story_id.to_string()))?;

    Ok(serde_json::to_value(story)?)
}

// ---------------------------------------------------------------------------
// v1 legacy: sort key and dependency helpers (4-level model)
// ---------------------------------------------------------------------------

/// Returns a sort key `(milestone.order, epic.order, story.order, task.order)` so that
/// tasks are naturally ordered by their position in the v1 planning hierarchy.
fn task_sort_key_v1(
    task: &Task,
    stories: &[planning_stories::Story],
    epics: &[planning_epics::Epic],
    milestones: &[planning_milestones::Milestone],
) -> (u32, u32, u32, u32) {
    let story = stories
        .iter()
        .find(|s| format!("#{}", s.id) == task.story);

    let epic_ref = if !task.epic.is_empty() {
        Some(task.epic.as_str())
    } else {
        story.map(|s| s.epic.as_str())
    };

    let epic = epic_ref.and_then(|e_ref| {
        epics
            .iter()
            .find(|e| format!("#{}", e.id) == e_ref)
    });

    let ms = epic.and_then(|e| {
        milestones
            .iter()
            .find(|m| format!("#{}", m.id) == e.milestone)
    });

    (
        ms.map(|m| m.order).unwrap_or(u32::MAX),
        epic.map(|e| e.order).unwrap_or(u32::MAX),
        story.map(|s| s.order).unwrap_or(u32::MAX),
        task.order,
    )
}

/// v1: Checks whether a task's parent milestone has all `depends_on` milestones completed.
fn milestone_deps_met_v1(
    task: &Task,
    stories: &[planning_stories::Story],
    epics: &[planning_epics::Epic],
    milestones: &[planning_milestones::Milestone],
) -> bool {
    let story = stories
        .iter()
        .find(|s| format!("#{}", s.id) == task.story);

    let epic_ref = if !task.epic.is_empty() {
        Some(task.epic.as_str())
    } else {
        story.map(|s| s.epic.as_str())
    };

    let epic = epic_ref.and_then(|e_ref| {
        epics
            .iter()
            .find(|e| format!("#{}", e.id) == e_ref)
    });

    let ms = epic.and_then(|e| {
        milestones
            .iter()
            .find(|m| format!("#{}", m.id) == e.milestone)
    });

    let Some(ms) = ms else { return true };

    ms.depends_on.iter().all(|dep| {
        let dep_id = strip_hash(dep);
        milestones
            .iter()
            .find(|m| m.id.as_str() == dep_id)
            .map(|m| m.status.is_completed())
            .unwrap_or(true)
    })
}

/// v1 next-task algorithm (4-level hierarchy, dependency DAG).
fn next_task_v1(
    mut items: Vec<Task>,
    milestones: &[planning_milestones::Milestone],
) -> ParserResult<serde_json::Value> {
    let stories = planning_stories::for_active_project().unwrap_or_default();
    let epics = planning_epics::for_active_project().unwrap_or_default();

    items.sort_by(|a, b| {
        task_sort_key_v1(a, &stories, &epics, milestones)
            .cmp(&task_sort_key_v1(b, &stories, &epics, milestones))
    });

    let completed_ids: HashSet<&str> = items
        .iter()
        .filter(|t| t.status.is_completed())
        .map(|t| t.id.as_str())
        .collect();

    let incomplete: Vec<&Task> = items
        .iter()
        .filter(|t| !t.status.is_completed())
        .collect();

    if incomplete.is_empty() {
        return Ok(serde_json::json!({
            "message": "All tasks completed."
        }));
    }

    let ready = incomplete.iter().find(|t| {
        let task_deps_met = t
            .depends_on
            .iter()
            .all(|dep| completed_ids.contains(strip_hash(dep)));
        let ms_deps_met = milestone_deps_met_v1(t, &stories, &epics, milestones);
        task_deps_met && ms_deps_met
    });

    match ready {
        Some(task) => Ok(serde_json::to_value((*task).clone())?),
        None => {
            let blocked: Vec<serde_json::Value> = incomplete
                .iter()
                .map(|t| {
                    let unmet: Vec<&str> = t
                        .depends_on
                        .iter()
                        .filter(|dep| !completed_ids.contains(strip_hash(dep)))
                        .map(|s| s.as_str())
                        .collect();
                    serde_json::json!({
                        "id": t.id,
                        "title": t.title,
                        "blocked_by": unmet,
                    })
                })
                .collect();

            Ok(serde_json::json!({
                "message": "All remaining tasks are blocked by unmet dependencies.",
                "blocked_tasks": blocked,
            }))
        }
    }
}

// ---------------------------------------------------------------------------
// v2: simplified next-task (2-level model, strict execution order)
// ---------------------------------------------------------------------------

/// v2 next-task algorithm: find lowest execution_order incomplete task
/// in the lowest incomplete milestone. No DAG resolution needed.
fn next_task_v2(
    items: &[Task],
    milestones: &[planning_milestones::Milestone],
) -> ParserResult<serde_json::Value> {
    let mut sorted_ms: Vec<_> = milestones.to_vec();
    sorted_ms.sort_by_key(|m| m.order);

    // Find the first milestone whose dependencies are met and is not completed.
    let current_ms = sorted_ms.iter().find(|m| {
        if m.status.is_completed() {
            return false;
        }
        // Check milestone-level dependencies
        m.depends_on.iter().all(|dep| {
            let dep_id = strip_hash(dep);
            milestones
                .iter()
                .find(|dm| dm.id.as_str() == dep_id)
                .map(|dm| dm.status.is_completed())
                .unwrap_or(true)
        })
    });

    let Some(current_ms) = current_ms else {
        // Check if all are completed or all are blocked
        let all_completed = sorted_ms.iter().all(|m| m.status.is_completed());
        if all_completed {
            return Ok(serde_json::json!({
                "message": "All tasks completed."
            }));
        }
        return Ok(serde_json::json!({
            "message": "All remaining milestones are blocked by unmet dependencies."
        }));
    };

    // Get tasks for this milestone, sorted by execution_order (then order as fallback).
    let ms_ref = format!("#{}", current_ms.id);
    let mut ms_tasks: Vec<&Task> = items
        .iter()
        .filter(|t| t.milestone == ms_ref)
        .collect();
    ms_tasks.sort_by_key(|t| t.execution_order.unwrap_or(t.order));

    let next = ms_tasks.iter().find(|t| !t.status.is_completed());

    match next {
        Some(task) => Ok(serde_json::to_value((*task).clone())?),
        None => Ok(serde_json::json!({
            "message": "All tasks in current milestone completed. Milestone ready for review.",
            "milestone": current_ms.id,
        })),
    }
}

/// Finds the next task that is ready to work on.
/// Detects model version and dispatches to v1 or v2 algorithm.
fn next_task() -> ParserResult<serde_json::Value> {
    let items = planning_tasks::for_active_project()?;
    let milestones = planning_milestones::for_active_project().unwrap_or_default();

    // Detect model: if any task has a non-empty milestone field, use v2.
    let is_v2 = items.iter().any(|t| t.is_v2());

    if is_v2 {
        next_task_v2(&items, &milestones)
    } else {
        next_task_v1(items, &milestones)
    }
}

#[allow(clippy::too_many_arguments)]
fn create_task(
    title: String,
    steps: String,
    order: Option<u32>,
    status: String,
    epic: String,
    story: String,
    milestone: String,
    group: Option<String>,
    purpose: Option<String>,
    execution_order: Option<u32>,
    batch_with: String,
    depends_on: String,
    complexity: String,
    mode: String,
    skills: String,
    integration_tests: String,
    trace_tags: String,
    trace_intake_sources: String,
    trace_output_sources: String,
    note: Option<String>,
) -> ParserResult<Task> {
    // Load ALL tasks across all shards for global ID/order uniqueness.
    let all_items = planning_tasks::for_active_project()?;

    let item = Task {
        id: planning_tasks::next_id(&all_items),
        order: order.unwrap_or_else(|| planning_tasks::next_order(&all_items)),
        title: SmolStr::new(title),
        steps: parse_pipe(&steps),
        status: parse_status(&status)?,
        epic: epic.clone(),
        story: story.clone(),
        milestone: milestone.clone(),
        group,
        purpose,
        execution_order,
        batch_with: parse_csv(&batch_with),
        depends_on: parse_csv(&depends_on),
        complexity: parse_complexity(&complexity)?,
        mode: parse_mode(&mode)?,
        skills: parse_csv(&skills),
        integration_tests: parse_pipe(&integration_tests),
        traces: TaskTraces {
            tags: parse_csv(&trace_tags),
            intake_sources: parse_csv(&trace_intake_sources),
            output_sources: parse_csv(&trace_output_sources),
        },
        notes: note.map(|n| vec![MilestoneNote { note: n }]),
    };

    // v2: shard by milestone; v1: shard by epic/story
    if !milestone.is_empty() {
        let mut shard = planning_tasks::load_milestone_shard_for_active_project(&milestone)?;
        shard.push(item.clone());
        planning_tasks::save_milestone_shard_for_active_project(&shard, &milestone)?;
    } else {
        let mut shard = planning_tasks::load_shard_for_active_project(&epic, &story)?;
        shard.push(item.clone());
        planning_tasks::save_shard_for_active_project(&shard, &epic, &story)?;
    }
    Ok(item)
}

fn remove_task(id: &str) -> ParserResult<Task> {
    let all_items = planning_tasks::for_active_project()?;
    let task = all_items
        .iter()
        .find(|t| t.id == id)
        .ok_or_else(|| ParserError::PlanningTaskNotFound(id.to_string()))?;

    if task.is_v2() {
        // v2: shard by milestone
        let milestone = task.milestone.clone();
        let mut shard = planning_tasks::load_milestone_shard_for_active_project(&milestone)?;
        let position = shard
            .iter()
            .position(|t| t.id == id)
            .ok_or_else(|| ParserError::PlanningTaskNotFound(id.to_string()))?;
        let removed = shard.remove(position);
        planning_tasks::save_milestone_shard_for_active_project(&shard, &milestone)?;
        Ok(removed)
    } else {
        // v1: shard by epic/story
        let epic = task.epic.clone();
        let story = task.story.clone();
        let mut shard = planning_tasks::load_shard_for_active_project(&epic, &story)?;
        let position = shard
            .iter()
            .position(|t| t.id == id)
            .ok_or_else(|| ParserError::PlanningTaskNotFound(id.to_string()))?;
        let removed = shard.remove(position);
        planning_tasks::save_shard_for_active_project(&shard, &epic, &story)?;
        Ok(removed)
    }
}

#[allow(clippy::too_many_arguments)]
fn update_task(
    id: &str,
    title: Option<String>,
    steps: Option<String>,
    order: Option<u32>,
    status: Option<String>,
    story: Option<String>,
    milestone: Option<String>,
    group: Option<String>,
    purpose: Option<String>,
    execution_order: Option<u32>,
    batch_with: Option<String>,
    depends_on: Option<String>,
    complexity: Option<String>,
    mode: Option<String>,
    skills: Option<String>,
    integration_tests: Option<String>,
    trace_tags: Option<String>,
    trace_intake_sources: Option<String>,
    trace_output_sources: Option<String>,
    note: Option<String>,
) -> ParserResult<Task> {
    let all_items = planning_tasks::for_active_project()?;
    let existing = all_items
        .iter()
        .find(|t| t.id == id)
        .ok_or_else(|| ParserError::PlanningTaskNotFound(id.to_string()))?;

    let is_v2 = existing.is_v2();

    // Load the shard this task currently lives in.
    let (mut shard, old_shard_key) = if is_v2 {
        let ms = existing.milestone.clone();
        (planning_tasks::load_milestone_shard_for_active_project(&ms)?, ms)
    } else {
        let epic = existing.epic.clone();
        let story_ref = existing.story.clone();
        (planning_tasks::load_shard_for_active_project(&epic, &story_ref)?, story_ref)
    };

    let item = shard
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| ParserError::PlanningTaskNotFound(id.to_string()))?;

    // Apply mutations.
    if let Some(v) = title {
        item.title = SmolStr::new(v);
    }
    if let Some(v) = steps {
        item.steps = parse_pipe(&v);
    }
    if let Some(v) = order {
        item.order = v;
    }
    if let Some(v) = status {
        item.status = parse_status(&v)?;
    }
    // v1 fields
    if let Some(v) = &story {
        item.story = v.clone();
    }
    // v2 fields
    if let Some(v) = &milestone {
        item.milestone = v.clone();
    }
    if let Some(v) = group {
        item.group = Some(v);
    }
    if let Some(v) = purpose {
        item.purpose = Some(v);
    }
    if let Some(v) = execution_order {
        item.execution_order = Some(v);
    }
    if let Some(v) = batch_with {
        item.batch_with = parse_csv(&v);
    }
    if let Some(v) = depends_on {
        item.depends_on = parse_csv(&v);
    }
    if let Some(v) = complexity {
        item.complexity = parse_complexity(&v)?;
    }
    if let Some(v) = mode {
        item.mode = parse_mode(&v)?;
    }
    if let Some(v) = skills {
        item.skills = parse_csv(&v);
    }
    if let Some(v) = integration_tests {
        item.integration_tests = parse_pipe(&v);
    }
    if let Some(v) = trace_tags {
        item.traces.tags = parse_csv(&v);
    }
    if let Some(v) = trace_intake_sources {
        item.traces.intake_sources = parse_csv(&v);
    }
    if let Some(v) = trace_output_sources {
        item.traces.output_sources = parse_csv(&v);
    }
    if let Some(n) = note {
        let new_note = MilestoneNote { note: n };
        match &mut item.notes {
            Some(notes) => notes.push(new_note),
            None => item.notes = Some(vec![new_note]),
        }
    }

    let updated = item.clone();

    // Handle shard migration if the shard key changed.
    if is_v2 {
        let new_ms = milestone.unwrap_or(old_shard_key.clone());
        if new_ms != old_shard_key {
            // Milestone changed — migrate shard
            shard.retain(|t| t.id != id);
            planning_tasks::save_milestone_shard_for_active_project(&shard, &old_shard_key)?;
            let mut new_shard = planning_tasks::load_milestone_shard_for_active_project(&new_ms)?;
            new_shard.push(updated.clone());
            planning_tasks::save_milestone_shard_for_active_project(&new_shard, &new_ms)?;
        } else {
            planning_tasks::save_milestone_shard_for_active_project(&shard, &old_shard_key)?;
        }
    } else {
        let new_story = story.clone();
        let story_changing = new_story
            .as_ref()
            .map(|s| *s != old_shard_key)
            .unwrap_or(false);

        if story_changing {
            let old_epic = existing.epic.clone();
            shard.retain(|t| t.id != id);
            planning_tasks::save_shard_for_active_project(&shard, &old_epic, &old_shard_key)?;

            let new_epic = if !updated.epic.is_empty() {
                updated.epic.clone()
            } else {
                let stories = planning_stories::for_active_project().unwrap_or_default();
                let story_id = strip_hash(&updated.story);
                stories
                    .iter()
                    .find(|s| s.id == story_id)
                    .map(|s| s.epic.clone())
                    .unwrap_or(old_epic)
            };

            let mut new_shard =
                planning_tasks::load_shard_for_active_project(&new_epic, &updated.story)?;
            new_shard.push(updated.clone());
            planning_tasks::save_shard_for_active_project(&new_shard, &new_epic, &updated.story)?;
        } else {
            let old_epic = existing.epic.clone();
            planning_tasks::save_shard_for_active_project(&shard, &old_epic, &old_shard_key)?;
        }
    }

    Ok(updated)
}

// ---------------------------------------------------------------------------
// List with filtering (v2 support)
// ---------------------------------------------------------------------------

fn list_tasks(
    milestone_filter: Option<String>,
    group_filter: Option<String>,
) -> ParserResult<serde_json::Value> {
    let mut items = planning_tasks::for_active_project()?;

    if let Some(ms) = milestone_filter {
        let ms_ref = if ms.starts_with('#') {
            ms
        } else {
            format!("#{ms}")
        };
        items.retain(|t| t.milestone == ms_ref);
    }

    if let Some(grp) = group_filter {
        items.retain(|t| t.group.as_deref() == Some(&grp));
    }

    // Sort: v2 by execution_order, v1 by order
    items.sort_by_key(|t| t.execution_order.unwrap_or(t.order));

    Ok(serde_json::to_value(items)?)
}

// ---------------------------------------------------------------------------
// Reorder tasks within a milestone (v2)
// ---------------------------------------------------------------------------

fn reorder_tasks(milestone_id: &str) -> ParserResult<serde_json::Value> {
    let ms_ref = if milestone_id.starts_with('#') {
        milestone_id.to_string()
    } else {
        format!("#{milestone_id}")
    };

    let mut shard = planning_tasks::load_milestone_shard_for_active_project(&ms_ref)?;

    // Sort by current execution_order (or order as fallback), then renumber.
    shard.sort_by_key(|t| t.execution_order.unwrap_or(t.order));
    for (i, task) in shard.iter_mut().enumerate() {
        task.execution_order = Some((i as u32) + 1);
    }

    planning_tasks::save_milestone_shard_for_active_project(&shard, &ms_ref)?;

    Ok(serde_json::json!({
        "message": format!("Renumbered {} tasks in {}", shard.len(), milestone_id),
        "tasks": serde_json::to_value(&shard)?,
    }))
}
