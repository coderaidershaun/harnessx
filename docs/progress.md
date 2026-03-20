# Progress Commands

Tracks progress through pipeline stages for the active project. Stored at `harnessx/<id>/progress.json`.

Each stage has `status` (a Status value) and `command` (string).

Stages (in order): `intake`, `planning`, `review`, `execution`, `user_acceptance`, `complete`, `user_input_required`.

## `progress init`

Initialises the default progress file for the active project.

```bash
harnessx progress init
```

```json
{
  "success": true,
  "data": {
    "intake": { "status": "not_started", "command": "" },
    "planning": { "status": "not_started", "command": "" },
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
    "stage": "intake",
    "status": "not_started",
    "command": ""
  }
}
```

## `progress complete <stage>`

Marks a stage's status as `completed`.

```bash
harnessx progress complete intake
```

## `progress update <stage> <status>`

Sets a stage to any status value (`not_started`, `in_progress`, `completed`, `rework`).

```bash
harnessx progress update planning in_progress
```
