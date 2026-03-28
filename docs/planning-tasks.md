# Planning Tasks Commands

Manage planning tasks for the active project.

## Storage Models

**v2 (current):** Tasks are sharded by milestone:

```
harnessx/<id>/planning/tasks/<milestone-id>/planning_tasks.json
```

**v1 (legacy):** Tasks are sharded by epic and story:

```
harnessx/<id>/planning/tasks/<epic-id>/<story-id>/planning_tasks.json
```

Tasks are atomic units of implementation — each has a clear start, a clear end, and can be finished in a single focused session. In the v2 model, tasks belong directly to milestones with optional `group` labels for organization. In v1, tasks belong to stories under epics.

## JSON Structure

The tasks file wraps the array in a top-level object:

```json
{
  "tasks": [
    {
      "id": "task-1",
      "order": 1,
      "title": "Project skeleton: Cargo.toml, .env, .gitignore, lib.rs",
      "steps": [
        "Configure Cargo.toml with SDK dependencies",
        "Create .env.example and add .env to .gitignore",
        "Set up src/lib.rs with crate root"
      ],
      "status": "not_started",
      "milestone": "#milestone-1",
      "group": "setup",
      "purpose": "Bootstrap the project so all subsequent tasks can compile and run",
      "execution_order": 1,
      "batch_with": [],
      "epic": "",
      "story": "",
      "depends_on": [],
      "complexity": "low",
      "mode": "plan",
      "skills": ["rust:developing"],
      "integration_tests": [
        "cargo build exits with code 0",
        ".env.example exists with placeholder keys"
      ],
      "traces": {
        "tags": ["#action-1"],
        "intake_sources": ["#intake-resources"],
        "output_sources": ["Cargo.toml", "src/lib.rs"]
      },
      "notes": [
        { "note": "Ensure edition = 2024 and all 6 SDK features enabled." }
      ]
    }
  ]
}
```

## Fields

### v2 Fields (Milestone-Direct Model)

| Field             | Type       | Description                                                                 |
|-------------------|------------|-----------------------------------------------------------------------------|
| `milestone`       | string     | Direct parent milestone ref (e.g. `#milestone-1`). Determines shard path.   |
| `group`           | string?    | Lightweight label replacing epics (e.g. `"setup"`, `"harness"`, `"ws-market"`) |
| `purpose`         | string?    | The WHY — explains why this task exists, replacing story descriptions       |
| `execution_order` | u32?       | Strict ordering within the milestone. Lower runs first.                     |
| `batch_with`      | string[]   | Task IDs to execute in the same agent session (e.g. `["#task-2"]`)          |

### v1 Legacy Fields (4-Level Hierarchy)

| Field   | Type   | Description                                                   |
|---------|--------|---------------------------------------------------------------|
| `epic`  | string | Parent epic ref (e.g. `#epic-1`). Used for v1 shard path.    |
| `story` | string | Parent story ref (e.g. `#story-1`). Used for v1 shard path.  |

### Common Fields

| Field              | Type       | Description                                                |
|--------------------|------------|------------------------------------------------------------|
| `id`               | string     | Auto-assigned (`task-1`, `task-2`, ...)                    |
| `order`            | u32        | Global order (auto-increment). v2 uses `execution_order` instead. |
| `title`            | string     | Specific, actionable description of the work               |
| `steps`            | string[]   | Ordered implementation steps                               |
| `status`           | Status     | Current status (see enum below)                            |
| `depends_on`       | string[]   | Task dependency refs. In v2, only for cross-milestone deps. |
| `complexity`       | Complexity | Complexity rating (see enum below)                         |
| `mode`             | ActionMode | Type of work (see enum below)                              |
| `skills`           | string[]   | Skill identifiers for the executing agent                  |
| `integration_tests`| string[]   | Descriptions of how to verify the task                     |
| `traces`           | TaskTraces | Traceability links (see below)                             |
| `notes`            | Note[]?    | Historical notes                                           |

## Types

### Status

Reuses the shared `Status` enum. Serialised as snake_case in JSON.

| Value        | JSON value       |
|--------------|------------------|
| `NotStarted` | `"not_started"` |
| `InProgress` | `"in_progress"` |
| `Completed`  | `"completed"`   |
| `Rework`     | `"rework"`      |

### Complexity

Reuses the `Complexity` enum from intake actions. Serialised as kebab-case in JSON.

| Value       | JSON value     |
|-------------|----------------|
| `SuperLow`  | `"super-low"`  |
| `Low`       | `"low"`        |
| `Medium`    | `"medium"`     |
| `High`      | `"high"`       |
| `SuperHigh` | `"super-high"` |
| `Uncertain` | `"uncertain"`  |

### ActionMode

Reuses the `ActionMode` enum from intake actions. Serialised as kebab-case in JSON.

| Value     | JSON value  |
|-----------|-------------|
| `Plan`    | `"plan"`    |
| `Execute` | `"execute"` |
| `Review`  | `"review"`  |
| `Rework`  | `"rework"`  |

### TaskTraces

Extended traces that include output sources for traceability to generated code.

| Field            | Type       | Description                                              |
|------------------|------------|----------------------------------------------------------|
| `tags`           | string[]   | References to action items (e.g. `#action-1`)            |
| `intake_sources` | string[]   | References to intake sections (e.g. `#intake-resources`) |
| `output_sources` | string[]   | File paths the task is expected to produce or modify      |

### Note

| Field  | Type   | Description  |
|--------|--------|--------------|
| `note` | string | Note content |

## `planning-tasks next`

Returns the next task that is **ready to work on**. Auto-detects model version.

```bash
harnessx planning-tasks next
```

### v2 Algorithm (Strict Execution Order)

1. Load all tasks from all shards.
2. Find the lowest-order incomplete milestone whose dependencies are met.
3. Get tasks for that milestone, sorted by `execution_order`.
4. Return the first incomplete task.

Within a milestone, `execution_order` IS the dependency — task 5 can assume tasks 1-4 are done. No DAG resolution needed. `depends_on` is only checked for cross-milestone references.

### v1 Algorithm (Dependency DAG)

1. Load all tasks from all shards.
2. Sort by hierarchy: `(milestone.order, epic.order, story.order, task.order)`.
3. A task is "ready" if all `depends_on` refs are completed AND its parent milestone's dependencies are met.
4. Return the first ready task in hierarchy order.

### Response shapes

**Ready task found** — returns the full task object:
```json
{ "id": "task-3", "order": 3, "title": "...", "steps": [...], ... }
```

**All blocked** — returns diagnostics:
```json
{
  "message": "All remaining tasks are blocked by unmet dependencies.",
  "blocked_tasks": [
    { "id": "task-5", "title": "...", "blocked_by": ["#task-3", "#task-4"] }
  ]
}
```

**Milestone tasks done (v2):**
```json
{
  "message": "All tasks in current milestone completed. Milestone ready for review.",
  "milestone": "milestone-1"
}
```

**All completed:**
```json
{ "message": "All tasks completed." }
```

## `planning-tasks create`

Creates a new task for the active project. The `id` is auto-assigned (`task-1`, `task-2`, ...) and `order` defaults to the next sequential value.

### v2 Example (Milestone-Direct)

```bash
harnessx planning-tasks create \
  --title "Project skeleton: Cargo.toml, .env, .gitignore, lib.rs" \
  --steps "Configure Cargo.toml with SDK deps | Create .env.example | Set up src/lib.rs" \
  --milestone "#milestone-1" \
  --group "setup" \
  --purpose "Bootstrap the project so all subsequent tasks can compile" \
  --execution-order 1 \
  --complexity low \
  --mode plan \
  --skills "rust:developing" \
  --integration-tests "cargo build exits with code 0 | .env.example exists" \
  --trace-tags "#action-1" \
  --trace-intake-sources "#intake-resources" \
  --trace-output-sources "Cargo.toml,src/lib.rs"
```

### v1 Example (Legacy Epic/Story)

```bash
harnessx planning-tasks create \
  --title "Write the GraphQL query for fetching open positions" \
  --steps "Look up the subgraph schema | Write the query | Add pagination" \
  --epic "#epic-1" \
  --story "#story-1" \
  --complexity low \
  --mode plan \
  --skills "rust:developing"
```

### Flags

| Flag                      | Required | Default        | Description                                            |
|---------------------------|----------|----------------|--------------------------------------------------------|
| `--title`                 | no       | `""`           | Specific, actionable description of the work           |
| `--steps`                 | no       | `""`           | **Pipe-separated** ordered steps (see note below)      |
| `--order`                 | no       | auto-increment | Explicit ordering; defaults to next sequential         |
| `--status`                | no       | `"not_started"` | Status (see enum above)                                |
| `--milestone`             | no       | `""`           | **v2:** Parent milestone ref (e.g. `#milestone-1`). Determines shard path. |
| `--group`                 | no       | —              | **v2:** Lightweight grouping label (e.g. `"setup"`)    |
| `--purpose`               | no       | —              | **v2:** Why this task exists                           |
| `--execution-order`       | no       | —              | **v2:** Strict ordering within milestone (lower first) |
| `--batch-with`            | no       | `""`           | **v2:** Comma-separated task IDs for same-session execution |
| `--epic`                  | no       | `""`           | **v1 legacy:** Parent epic ref. Determines v1 shard path. |
| `--story`                 | no       | `""`           | **v1 legacy:** Parent story ref                        |
| `--depends-on`            | no       | `""`           | Comma-separated task dependency references             |
| `--complexity`            | no       | `""`           | Complexity level (see enum above)                      |
| `--mode`                  | no       | `""`           | Action mode (see enum above)                           |
| `--skills`                | no       | `""`           | Comma-separated skill identifiers needed               |
| `--integration-tests`     | no       | `""`           | **Pipe-separated** integration test descriptions       |
| `--trace-tags`            | no       | `""`           | Comma-separated action item tag references             |
| `--trace-intake-sources`  | no       | `""`           | Comma-separated intake source references               |
| `--trace-output-sources`  | no       | `""`           | Comma-separated output file path references            |
| `--note`                  | no       | —              | Note content to attach                                 |

> **Note on pipe separation:** `steps` and `integration_tests` use pipe (`|`) as the delimiter instead of commas, since their text commonly contains commas. Example: `"Write a query that filters by owner address, liquidity > 0 | Add pagination for the 1000-entity limit"`.

## `planning-tasks list`

Lists all tasks for the active project. Supports filtering by milestone or group (v2).

```bash
harnessx planning-tasks list
harnessx planning-tasks list --milestone milestone-1
harnessx planning-tasks list --group "setup"
```

| Flag          | Description                              |
|---------------|------------------------------------------|
| `--milestone` | Filter tasks by parent milestone ID      |
| `--group`     | Filter tasks by group label              |

Tasks are sorted by `execution_order` (v2) or `order` (v1).

## `planning-tasks get <id>`

Returns a single task by its ID.

```bash
harnessx planning-tasks get task-1
```

Returns the full task object, or an error if the ID doesn't exist.

## `planning-tasks remove <id>`

Removes a task by its ID.

```bash
harnessx planning-tasks remove task-1
```

## `planning-tasks update <id>`

Updates fields on an existing task. Only provided flags are changed.

```bash
harnessx planning-tasks update task-1 \
  --status in_progress \
  --mode execute \
  --note "Implementation started."
```

| Flag                      | Description                                            |
|---------------------------|--------------------------------------------------------|
| `--title`                 | New title                                              |
| `--steps`                 | Replacement **pipe-separated** steps                   |
| `--order`                 | New order value                                        |
| `--status`                | New status                                             |
| `--milestone`             | **v2:** New milestone reference (triggers shard migration) |
| `--group`                 | **v2:** New group label                                |
| `--purpose`               | **v2:** New purpose text                               |
| `--execution-order`       | **v2:** New execution order value                      |
| `--batch-with`            | **v2:** Replacement comma-separated batch task refs    |
| `--story`                 | **v1 legacy:** New story reference (triggers shard migration) |
| `--depends-on`            | Replacement comma-separated dependency references      |
| `--complexity`            | New complexity level                                   |
| `--mode`                  | New action mode                                        |
| `--skills`                | Replacement comma-separated skill identifiers          |
| `--integration-tests`     | Replacement **pipe-separated** integration tests       |
| `--trace-tags`            | Replacement comma-separated trace tag refs             |
| `--trace-intake-sources`  | Replacement comma-separated trace intake source refs   |
| `--trace-output-sources`  | Replacement comma-separated trace output source refs   |
| `--note`                  | Note content to append                                 |

On update, notes are **appended** to the existing list (not replaced).

## `planning-tasks parent <id>`

Returns the parent of this task: the milestone (v2) or the story (v1), resolved from the task's `milestone` or `story` field.

```bash
harnessx planning-tasks parent task-1
```

## `planning-tasks reorder <milestone-id>`

Renumbers `execution_order` for all tasks in a milestone (v2). Tasks are sorted by current `execution_order` (or `order` as fallback) and assigned consecutive integers starting at 1.

Useful after inserting or removing tasks to eliminate gaps in the ordering.

```bash
harnessx planning-tasks reorder milestone-1
```

Returns the renumbered task list.
