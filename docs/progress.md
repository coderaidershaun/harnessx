# Progress Commands

Tracks progress through pipeline stages for the active project. Stored at `harnessx/<id>/progress.json`.

Each stage has `status` (a Status value) and `agent` (the specialist agent name that handles this stage).

Stages (in order): `user_input_required`, `intake_onboarding`, `intake_team`, `intake_exploration`, `planning`, `review`, `execution`, `user_acceptance`, `complete`.

## `progress init`

Initialises the default progress file for the active project.

```bash
harnessx progress init
```

```json
{
  "success": true,
  "data": {
    "user_input_required": { "status": "completed", "agent": "hx-user-troubleshooting-specialist" },
    "intake_onboarding": { "status": "not_started", "agent": "hx-intake-onboarding-specialist" },
    "intake_team": { "status": "not_started", "agent": "hx-intake-team-specialist" },
    "intake_exploration": { "status": "not_started", "agent": "hx-intake-exploration-specialist" },
    "planning": { "status": "not_started", "agent": "hx-planning-specialist" },
    ...
  }
}
```

## `progress status`

Shows progress for the active project.

```bash
harnessx progress status
```

## `progress next`

Returns the next incomplete stage (first stage where status is not `completed`).

```bash
harnessx progress next
```

```json
{
  "success": true,
  "data": {
    "stage": "intake_onboarding",
    "status": "not_started",
    "agent": "hx-intake-onboarding-specialist"
  }
}
```

## `progress complete <stage>`

Marks a stage's status as `completed`.

```bash
harnessx progress complete intake_onboarding
```

## `progress update <stage> <status>`

Sets a stage to any status value (`not_started`, `in_progress`, `completed`, `rework`).

```bash
harnessx progress update planning in_progress
```
