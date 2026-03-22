# Planning Milestones Commands

Manage planning milestones for the active project. Stored at `harnessx/<id>/planning/planning_milestones.json`.

Milestones are demonstrable states of the project — checkpoints where you could show someone the system and they'd see real, observable progress. They map to success measures and UAT criteria from intake, and are sequenced by dependency order.

## JSON Structure

The milestones file wraps the array in a top-level object:

```json
{
  "milestones": [
    {
      "id": "milestone-1",
      "order": 1,
      "title": "Live position data flowing through the pipeline",
      "description": "A reviewer can open the app and see real positions...",
      "status": "not_started",
      "depends_on": [],
      "success_measures": ["#success-measure-1"],
      "uat_criteria": ["#uat-scenario-1"],
      "traces": {
        "tags": ["#action-1", "#action-4"],
        "intake_sources": ["#intake-goal", "#intake-scope"]
      },
      "notes": [
        { "note": "This is the thinnest possible vertical slice." }
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

### Traces

Links a milestone back to intake artifacts for traceability.

| Field            | Type       | Description                                     |
|------------------|------------|-------------------------------------------------|
| `tags`           | string[]   | References to action items (e.g. `#action-1`)   |
| `intake_sources` | string[]   | References to intake sections (e.g. `#intake-goal`) |

### MilestoneNote

| Field  | Type   | Description  |
|--------|--------|--------------|
| `note` | string | Note content |

## `planning-milestones next`

Returns the next incomplete milestone by `order`. If all milestones are completed, returns a completion message.

```bash
harnessx planning-milestones next
```

Returns the full milestone object so the calling agent has all context needed to act on it.

## `planning-milestones create`

Creates a new milestone for the active project. The `id` is auto-assigned (`milestone-1`, `milestone-2`, ...) and `order` defaults to the next sequential value.

```bash
harnessx planning-milestones create \
  --title "Live position data flowing through the pipeline" \
  --description "A reviewer can open the app and see real positions" \
  --status not_started \
  --depends-on "#milestone-1" \
  --success-measures "#success-measure-1, #success-measure-3" \
  --uat-criteria "#uat-scenario-1" \
  --trace-tags "#action-1, #action-4, #action-7" \
  --trace-intake-sources "#intake-goal, #intake-scope" \
  --note "This is the thinnest possible vertical slice."
```

| Flag                     | Required | Default        | Description                                     |
|--------------------------|----------|----------------|-------------------------------------------------|
| `--title`                | no       | `""`           | Short, state-based description of the checkpoint |
| `--description`          | no       | `""`           | What someone would observe at this checkpoint   |
| `--order`                | no       | auto-increment | Explicit ordering; defaults to next sequential  |
| `--status`               | no       | `"not_started"` | Status (see enum above)                         |
| `--depends-on`           | no       | `""`           | Comma-separated milestone dependency references |
| `--success-measures`     | no       | `""`           | Comma-separated success measure references      |
| `--uat-criteria`         | no       | `""`           | Comma-separated UAT criteria references         |
| `--trace-tags`           | no       | `""`           | Comma-separated action item tag references      |
| `--trace-intake-sources` | no       | `""`           | Comma-separated intake source references        |
| `--note`                 | no       | —              | Note content to attach                          |

## `planning-milestones list`

Lists all milestones for the active project.

```bash
harnessx planning-milestones list
```

## `planning-milestones remove <id>`

Removes a milestone by its ID.

```bash
harnessx planning-milestones remove milestone-1
```

## `planning-milestones update <id>`

Updates fields on an existing milestone. Only provided flags are changed.

```bash
harnessx planning-milestones update milestone-1 \
  --status in_progress \
  --note "Work has begun on the data pipeline."
```

| Flag                     | Description                                          |
|--------------------------|------------------------------------------------------|
| `--title`                | New title                                            |
| `--description`          | New description                                      |
| `--order`                | New order value                                      |
| `--status`               | New status                                           |
| `--depends-on`           | Replacement comma-separated dependency references    |
| `--success-measures`     | Replacement comma-separated success measure refs     |
| `--uat-criteria`         | Replacement comma-separated UAT criteria refs        |
| `--trace-tags`           | Replacement comma-separated trace tag refs           |
| `--trace-intake-sources` | Replacement comma-separated trace intake source refs |
| `--note`                 | Note content to append                               |

On update, notes are **appended** to the existing list (not replaced).

## `planning-milestones children <id>`

Returns all epics, stories, and tasks that belong to a milestone. Traverses the full hierarchy: epics whose `milestone` references this ID, stories whose `epic` references those epics, and tasks whose `story` references those stories.

```bash
harnessx planning-milestones children milestone-1
```

Returns:
```json
{
  "milestone": "milestone-1",
  "epics": [...],
  "stories": [...],
  "tasks": [...]
}
```
