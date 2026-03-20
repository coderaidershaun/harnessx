# Project Commands

Manage projects in the harnessx registry.

## `project create <id>`

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

## `project list`

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

## `project active`

Shows the currently active project. Returns an error if no project is active.

```bash
harnessx project active
```

## `project activate <id>`

Activates an inactive project by its ID. The project must exist in the inactive list.

```bash
harnessx project activate other-project
```

## `project remove <id>`

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
