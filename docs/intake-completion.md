# Intake Completion Commands

Tracks progress through intake completion sections for the active project. Stored at `harnessx/<id>/intake/intake_completion.json`.

Each section has `status` (a Status value) and `skills` (string array).

Sections (in order): `exploration`, `ideation`, `project_risk_manager`.

## `intake-completion init`

Initialises the default intake completion progress file for the active project.

```bash
harnessx intake-completion init
```

```json
{
  "success": true,
  "data": {
    "exploration": { "status": "not_started", "skills": [] },
    "ideation": { "status": "not_started", "skills": [] },
    "project_risk_manager": { "status": "not_started", "skills": [] }
  }
}
```

## `intake-completion status`

Shows intake completion progress for the active project.

```bash
harnessx intake-completion status
```

## `intake-completion list`

Lists all intake completion sections with their current status and skills.

```bash
harnessx intake-completion list
```

```json
{
  "success": true,
  "data": [
    { "section": "exploration", "status": "not_started", "skills": [] },
    { "section": "ideation", "status": "not_started", "skills": [] },
    { "section": "project_risk_manager", "status": "not_started", "skills": [] }
  ]
}
```

## `intake-completion next`

Returns the next incomplete intake completion section.

```bash
harnessx intake-completion next
```

```json
{
  "success": true,
  "data": {
    "section": "exploration",
    "skills": []
  }
}
```

## `intake-completion complete <section>`

Marks an intake completion section's status as `completed`.

```bash
harnessx intake-completion complete exploration
```

## `intake-completion update <section> <status>`

Sets a section to any status value (`not_started`, `in_progress`, `completed`, `rework`).

```bash
harnessx intake-completion update ideation in_progress
```
