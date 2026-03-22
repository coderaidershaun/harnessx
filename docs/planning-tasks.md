# Planning Tasks Commands

Manage planning tasks for the active project. Stored at `harnessx/<id>/planning/planning_tasks.json`.

Tasks are atomic units of implementation — each has a clear start, a clear end, and can be finished in a single focused session. Each task belongs to a story and is the thing a specialist agent actually executes. This is where complexity ratings and skill assignments matter most.

## JSON Structure

The tasks file wraps the array in a top-level object:

```json
{
  "tasks": [
    {
      "id": "task-1",
      "order": 1,
      "title": "Write the GraphQL query for fetching open positions",
      "steps": [
        "Look up the Uniswap v3 subgraph schema",
        "Write a query that filters by owner address"
      ],
      "status": "not_started",
      "story": "#story-1",
      "depends_on": [],
      "complexity": "low",
      "mode": "plan",
      "skills": ["rust:developing"],
      "integration_tests": [
        "Query returns positions for a known wallet"
      ],
      "traces": {
        "tags": ["#action-1"],
        "intake_sources": ["#intake-resources"],
        "output_sources": ["src/ingestion/queries.rs"]
      },
      "notes": [
        { "note": "Only fetch positions where liquidity > 0." }
      ]
    }
  ]
}
```

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

Returns the next task that is **ready to work on** using dependency-aware resolution.

```bash
harnessx planning-tasks next
```

### Algorithm

1. Collect the IDs of all completed tasks.
2. A task is "ready" if it is **not completed** and **all** of its `depends_on` references resolve to completed tasks. The `#` prefix on dependency references (e.g. `#task-1`) is stripped automatically when matching against task IDs.
3. Among ready tasks, return the one with the lowest `order`.
4. If no tasks are ready but incomplete tasks remain, return a `blocked_tasks` array listing each blocked task and its unmet dependencies.
5. If all tasks are completed, return a completion message.

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

**All completed:**
```json
{ "message": "All tasks completed." }
```

## `planning-tasks create`

Creates a new task for the active project. The `id` is auto-assigned (`task-1`, `task-2`, ...) and `order` defaults to the next sequential value.

```bash
harnessx planning-tasks create \
  --title "Write the GraphQL query for fetching open positions" \
  --steps "Look up the subgraph schema | Write the query | Add pagination" \
  --story "#story-1" \
  --complexity low \
  --mode plan \
  --skills "rust:developing" \
  --integration-tests "Query returns positions for a known wallet | Empty wallet returns empty results" \
  --trace-tags "#action-1" \
  --trace-intake-sources "#intake-resources" \
  --trace-output-sources "src/ingestion/queries.rs" \
  --note "Only fetch positions where liquidity > 0."
```

| Flag                      | Required | Default        | Description                                            |
|---------------------------|----------|----------------|--------------------------------------------------------|
| `--title`                 | no       | `""`           | Specific, actionable description of the work           |
| `--steps`                 | no       | `""`           | **Pipe-separated** ordered steps (see note below)      |
| `--order`                 | no       | auto-increment | Explicit ordering; defaults to next sequential         |
| `--status`                | no       | `"not_started"` | Status (see enum above)                                |
| `--story`                 | no       | `""`           | The story this task belongs to                         |
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

Lists all tasks for the active project.

```bash
harnessx planning-tasks list
```

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
| `--story`                 | New story reference                                    |
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
