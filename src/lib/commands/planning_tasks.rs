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
        #[arg(long, default_value = "")]
        epic: String,
        #[arg(long, default_value = "")]
        story: String,
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
        #[arg(long)]
        story: Option<String>,
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
    /// List all tasks.
    List,
    /// Get a single task by ID.
    Get { id: String },
    /// Show the next ready task (dependency-aware).
    Next,
    /// Show the story this task belongs to.
    Parent { id: String },
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

            Self::List => exit_with(planning_tasks::for_active_project()),
            Self::Get { id } => exit_with(get_task(&id)),
            Self::Next => exit_with(next_task()),
            Self::Parent { id } => exit_with(task_parent(&id)),
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

    let story_id = strip_hash(&task.story);
    let stories = planning_stories::for_active_project()?;
    let story = stories
        .into_iter()
        .find(|s| s.id == story_id)
        .ok_or_else(|| ParserError::StoryNotFound(story_id.to_string()))?;

    Ok(serde_json::to_value(story)?)
}

/// Returns a sort key `(milestone.order, epic.order, story.order, task.order)` so that
/// tasks are naturally ordered by their position in the planning hierarchy.
fn task_sort_key(
    task: &Task,
    stories: &[planning_stories::Story],
    epics: &[planning_epics::Epic],
    milestones: &[planning_milestones::Milestone],
) -> (u32, u32, u32, u32) {
    let story = stories
        .iter()
        .find(|s| format!("#{}", s.id) == task.story);

    // Try the task's epic field first, then fall back to the story's epic.
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

/// Checks whether a task's parent milestone has all its `depends_on` milestones completed.
///
/// Traces task → story → epic → milestone, then checks each milestone dependency.
/// Returns `true` if all milestone-level dependencies are satisfied (or if the
/// parent chain can't be resolved — we don't block on missing data).
fn milestone_deps_met(
    task: &Task,
    stories: &[planning_stories::Story],
    epics: &[planning_epics::Epic],
    milestones: &[planning_milestones::Milestone],
) -> bool {
    // Trace: task → story → epic → milestone
    let story = stories
        .iter()
        .find(|s| format!("#{}", s.id) == task.story);

    // Try the task's epic field first, then fall back to the story's epic.
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

    // Check each milestone dependency is completed
    ms.depends_on.iter().all(|dep| {
        let dep_id = strip_hash(dep);
        milestones
            .iter()
            .find(|m| m.id.as_str() == dep_id)
            .map(|m| m.status.is_completed())
            .unwrap_or(true) // missing dependency → don't block
    })
}

/// Finds the next task that is ready to work on.
///
/// Algorithm:
/// 1. Load all tasks from all shards.
/// 2. Sort by hierarchy: (milestone.order, epic.order, story.order, task.order).
/// 3. Collect IDs of all completed tasks.
/// 4. A task is "ready" if it is not completed, ALL of its task-level dependencies
///    are in the completed set, AND its parent milestone's dependencies are completed.
/// 5. Among ready tasks, return the first one (lowest in hierarchy order).
/// 6. If no tasks are ready but incomplete tasks remain, report them as blocked
///    with the specific unmet dependencies for each.
/// 7. If all tasks are completed, report completion.
fn next_task() -> ParserResult<serde_json::Value> {
    let mut items = planning_tasks::for_active_project()?;

    // Load hierarchy for sorting and milestone-level dependency checks
    let stories = planning_stories::for_active_project().unwrap_or_default();
    let epics = planning_epics::for_active_project().unwrap_or_default();
    let milestones = planning_milestones::for_active_project().unwrap_or_default();

    // Sort by hierarchy: milestone → epic → story → task order
    items.sort_by(|a, b| {
        task_sort_key(a, &stories, &epics, &milestones)
            .cmp(&task_sort_key(b, &stories, &epics, &milestones))
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

    // Find the first task whose task-level AND milestone-level dependencies are all satisfied.
    let ready = incomplete.iter().find(|t| {
        let task_deps_met = t
            .depends_on
            .iter()
            .all(|dep| completed_ids.contains(strip_hash(dep)));
        let ms_deps_met = milestone_deps_met(t, &stories, &epics, &milestones);
        task_deps_met && ms_deps_met
    });

    match ready {
        Some(task) => Ok(serde_json::to_value((*task).clone())?),
        None => {
            // All remaining tasks are blocked — report what's blocking each.
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

#[allow(clippy::too_many_arguments)]
fn create_task(
    title: String,
    steps: String,
    order: Option<u32>,
    status: String,
    epic: String,
    story: String,
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

    // Load existing shard for this epic/story, append, save back.
    let mut shard = planning_tasks::load_shard_for_active_project(&epic, &story)?;
    shard.push(item.clone());
    planning_tasks::save_shard_for_active_project(&shard, &epic, &story)?;
    Ok(item)
}

fn remove_task(id: &str) -> ParserResult<Task> {
    // Find the task across all shards to determine which shard it lives in.
    let all_items = planning_tasks::for_active_project()?;
    let task = all_items
        .iter()
        .find(|t| t.id == id)
        .ok_or_else(|| ParserError::PlanningTaskNotFound(id.to_string()))?;

    let epic = task.epic.clone();
    let story = task.story.clone();

    // Load the specific shard, remove the task, save back.
    let mut shard = planning_tasks::load_shard_for_active_project(&epic, &story)?;
    let position = shard
        .iter()
        .position(|t| t.id == id)
        .ok_or_else(|| ParserError::PlanningTaskNotFound(id.to_string()))?;

    let removed = shard.remove(position);
    planning_tasks::save_shard_for_active_project(&shard, &epic, &story)?;
    Ok(removed)
}

#[allow(clippy::too_many_arguments)]
fn update_task(
    id: &str,
    title: Option<String>,
    steps: Option<String>,
    order: Option<u32>,
    status: Option<String>,
    story: Option<String>,
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
    // Find the task across all shards to determine which shard it lives in.
    let all_items = planning_tasks::for_active_project()?;
    let existing = all_items
        .iter()
        .find(|t| t.id == id)
        .ok_or_else(|| ParserError::PlanningTaskNotFound(id.to_string()))?;

    let old_epic = existing.epic.clone();
    let old_story = existing.story.clone();

    // Check if the story is changing (shard migration needed).
    let new_story = story.clone();
    let story_changing = new_story
        .as_ref()
        .map(|s| *s != old_story)
        .unwrap_or(false);

    // Load the specific shard this task currently lives in.
    let mut shard = planning_tasks::load_shard_for_active_project(&old_epic, &old_story)?;
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
    if let Some(v) = &new_story {
        item.story = v.clone();
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

    if story_changing {
        // Shard migration: remove from old shard, add to new shard.
        shard.retain(|t| t.id != id);
        planning_tasks::save_shard_for_active_project(&shard, &old_epic, &old_story)?;

        // Derive new epic from the story's parent epic.
        let new_epic = if !updated.epic.is_empty() {
            updated.epic.clone()
        } else {
            // Look up the new story's epic.
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
        // Save in place.
        planning_tasks::save_shard_for_active_project(&shard, &old_epic, &old_story)?;
    }

    Ok(updated)
}
