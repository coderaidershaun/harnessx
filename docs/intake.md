# Intake Commands

Tracks progress through intake sections for the active project. Stored at `harnessx/<id>/intake_progress.json`.

Each section has `status` (a Status value), `agent` (defaults to `"opus"`), and `skills` (string array).

Sections (in order): `goal`, `directory`, `scope`, `user_knowledge`, `resources`, `success_measures`, `user_acceptance_testing`, `team`, `exploration`, `ideation`, `project_risk_manager`.

## Shared Types

### Status

Used across intake and progress tracking. Serialised as snake_case in JSON.

| Value          | Description                  |
|----------------|------------------------------|
| `not_started`  | Default state                |
| `in_progress`  | Currently being worked on    |
| `completed`    | Finished                     |
| `rework`       | Needs to be redone           |

## `intake init`

Initialises the default intake progress file for the active project.

```bash
harnessx intake init
```

```json
{
  "success": true,
  "data": {
    "goal": { "status": "not_started", "agent": "opus", "skills": [] },
    ...
  }
}
```

## `intake status`

Shows intake progress for the active project.

```bash
harnessx intake status
```

## `intake next`

Returns the next incomplete intake section (first section where status is not `completed`).

```bash
harnessx intake next
```

```json
{
  "success": true,
  "data": {
    "section": "goal",
    "agent": "opus",
    "skills": []
  }
}
```

## `intake complete <section>`

Marks an intake section's status as `completed`.

```bash
harnessx intake complete goal
```
