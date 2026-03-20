# Intake Onboarding Commands

Tracks progress through intake onboarding sections for the active project. Stored at `harnessx/<id>/intake_progress.json`.

Each section has `status` (a Status value), `agent` (defaults to `"opus"`), and `skills` (string array).

Sections (in order): `goal`, `scope`, `user_knowledge`, `resources`, `success_measures`, `user_acceptance_testing`.

## Shared Types

### Status

Used across intake and progress tracking. Serialised as snake_case in JSON.

| Value          | Description                  |
|----------------|------------------------------|
| `not_started`  | Default state                |
| `in_progress`  | Currently being worked on    |
| `completed`    | Finished                     |
| `rework`       | Needs to be redone           |

## `intake-onboarding init`

Initialises the default intake onboarding progress file for the active project.

```bash
harnessx intake-onboarding init
```

```json
{
  "success": true,
  "data": {
    "goal": { "status": "not_started", "agent": "opus", "skills": ["hx:intake-onboarding-goal"] },
    "scope": { "status": "not_started", "agent": "opus", "skills": ["hx:intake-onboarding-scope"] },
    "user_knowledge": { "status": "not_started", "agent": "opus", "skills": ["hx:intake-onboarding-user-knowledge"] },
    "resources": { "status": "not_started", "agent": "opus", "skills": ["hx:intake-onboarding-resources"] },
    "success_measures": { "status": "not_started", "agent": "opus", "skills": ["hx:intake-onboarding-success-measures"] },
    "user_acceptance_testing": { "status": "not_started", "agent": "opus", "skills": ["hx:intake-onboarding-uat"] },
    ...
  }
}
```

## `intake-onboarding status`

Shows intake onboarding progress for the active project.

```bash
harnessx intake-onboarding status
```

## `intake-onboarding list`

Lists all intake onboarding sections with their current status, agent, and skills.

```bash
harnessx intake-onboarding list
```

```json
{
  "success": true,
  "data": [
    { "section": "goal", "status": "completed", "agent": "opus", "skills": ["hx:intake-onboarding-goal"] },
    { "section": "scope", "status": "not_started", "agent": "opus", "skills": ["hx:intake-onboarding-scope"] },
    { "section": "user_knowledge", "status": "not_started", "agent": "opus", "skills": ["hx:intake-onboarding-user-knowledge"] },
    { "section": "resources", "status": "not_started", "agent": "opus", "skills": ["hx:intake-onboarding-resources"] },
    { "section": "success_measures", "status": "not_started", "agent": "opus", "skills": ["hx:intake-onboarding-success-measures"] },
    { "section": "user_acceptance_testing", "status": "not_started", "agent": "opus", "skills": ["hx:intake-onboarding-uat"] },
    ...
  ]
}
```

## `intake-onboarding next`

Returns the next incomplete intake onboarding section (first section where status is not `completed`).

```bash
harnessx intake-onboarding next
```

```json
{
  "success": true,
  "data": {
    "section": "goal",
    "agent": "opus",
    "skills": ["hx:intake-onboarding-goal"]
  }
}
```

## `intake-onboarding complete <section>`

Marks an intake onboarding section's status as `completed`.

```bash
harnessx intake-onboarding complete goal
```
