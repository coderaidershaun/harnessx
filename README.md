# harnessx CLI

[Crate](https://crates.io/crates/harnessx) | [GitHub](https://github.com/coderaidershaun/harnessx)

Command-line interface for harnessx project management. All output is JSON.

> **Note:** This README is embedded inside the `harnessx/` folder that is created when you run `harnessx init`.

## Installation

You need [Rust](https://www.rust-lang.org/tools/install) installed first, then run:

```bash
cargo install harnessx
```

## Usage

```
harnessx <command> <subcommand> [args]
```

## Shared Types

### Status

Used across intake and progress tracking. Serialised as snake_case in JSON.

| Value          | Description                  |
|----------------|------------------------------|
| `not_started`  | Default state                |
| `in_progress`  | Currently being worked on    |
| `completed`    | Finished                     |
| `rework`       | Needs to be redone           |

## Commands

### Project

#### `project create <id>`

Creates a new project, its `harnessx/<id>/` metadata directory, and sets it as active.

```bash
harnessx project create my-project
```

```json
{
  "success": true,
  "data": {
    "id": "my-project",
    "title": "",
    "subtitle": "",
    "description": "",
    "takeaway_line": "",
    "directory": ""
  }
}
```

#### `project list`

Lists all projects (active and inactive).

```bash
harnessx project list
```

```json
{
  "success": true,
  "data": {
    "active": { "id": "my-project", "directory": "", ... },
    "inactive": [...]
  }
}
```

#### `project active`

Shows the currently active project. Returns an error if no project is active.

```bash
harnessx project active
```

#### `project activate <id>`

Activates an inactive project by its ID. The project must exist in the inactive list.

```bash
harnessx project activate other-project
```

#### `project remove <id>`

Removes a project from the registry and deletes its `harnessx/<id>/` metadata folder (intake_progress.json, progress.json, etc.). Does **not** touch the project's own working directory.

```bash
harnessx project remove my-project
```

```json
{
  "success": true,
  "data": {
    "id": "my-project",
    "title": "",
    "subtitle": "",
    "description": "",
    "takeaway_line": "",
    "directory": ""
  }
}
```

### Intake

Tracks progress through intake sections for the active project. Stored at `harnessx/<id>/intake_progress.json`.

Each section has `status` (a Status value), `agent` (defaults to `"opus"`), and `skills` (string array).

Sections (in order): `goal`, `directory`, `scope`, `user_knowledge`, `resources`, `success_measures`, `user_acceptance_testing`, `team`, `exploration`, `ideation`, `project_risk_manager`.

#### `intake init`

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

#### `intake status`

Shows intake progress for the active project.

```bash
harnessx intake status
```

#### `intake next`

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

#### `intake complete <section>`

Marks an intake section's status as `completed`.

```bash
harnessx intake complete goal
```

### Progress

Tracks progress through pipeline stages for the active project. Stored at `harnessx/<id>/progress.json`.

Each stage has `status` (a Status value) and `command` (string).

Stages (in order): `intake`, `planning`, `review`, `execution`, `user_acceptance`, `complete`, `user_input_required`.

#### `progress init`

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

#### `progress status`

Shows progress for the active project.

```bash
harnessx progress status
```

#### `progress next`

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

#### `progress complete <stage>`

Marks a stage's status as `completed`.

```bash
harnessx progress complete intake
```

#### `progress update <stage> <status>`

Sets a stage to any status value (`not_started`, `in_progress`, `completed`, `rework`).

```bash
harnessx progress update planning in_progress
```

## Output Format

All responses use a JSON envelope:

| Field     | Type    | Description                          |
|-----------|---------|--------------------------------------|
| `success` | bool    | `true` on success, `false` on error  |
| `data`    | object  | Present on success                   |
| `error`   | string  | Present on failure                   |

Exit code is `0` on success, `1` on error.

## Error Example

```json
{
  "success": false,
  "error": "no active project"
}
```

## Data Layout

```
harnessx/
  projects.json          # Project registry (active + inactive)
  <project-id>/
    intake_progress.json # Intake section tracking
    progress.json        # Pipeline stage tracking
```
