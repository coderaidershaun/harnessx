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
    "directory": "",
    "user_name": ""
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
    "directory": "",
    "user_name": ""
  }
}
```

## `project update-title <value>`

Updates the active project's title. Returns an error if no project is active.

```bash
harnessx project update-title "My Project"
```

```json
{
  "success": true,
  "data": {
    "id": "my-project",
    "title": "My Project",
    "subtitle": "",
    "description": "",
    "takeaway_line": "",
    "directory": "",
    "user_name": ""
  }
}
```

## `project update-subtitle <value>`

Updates the active project's subtitle. Returns an error if no project is active.

```bash
harnessx project update-subtitle "A short tagline"
```

```json
{
  "success": true,
  "data": {
    "id": "my-project",
    "title": "My Project",
    "subtitle": "A short tagline",
    "description": "",
    "takeaway_line": "",
    "directory": "",
    "user_name": ""
  }
}
```

## `project update-description <value>`

Updates the active project's description. Returns an error if no project is active.

```bash
harnessx project update-description "Full description of the project."
```

```json
{
  "success": true,
  "data": {
    "id": "my-project",
    "title": "My Project",
    "subtitle": "A short tagline",
    "description": "Full description of the project.",
    "takeaway_line": "",
    "directory": "",
    "user_name": ""
  }
}
```

## `project update-takeaway <value>`

Updates the active project's takeaway line. Returns an error if no project is active.

```bash
harnessx project update-takeaway "The key insight."
```

```json
{
  "success": true,
  "data": {
    "id": "my-project",
    "title": "My Project",
    "subtitle": "A short tagline",
    "description": "Full description of the project.",
    "takeaway_line": "The key insight.",
    "directory": "",
    "user_name": ""
  }
}
```

## `project update-directory <value>`

Updates the active project's directory path. Returns an error if no project is active.

```bash
harnessx project update-directory "/path/to/project"
```

```json
{
  "success": true,
  "data": {
    "id": "my-project",
    "title": "My Project",
    "subtitle": "A short tagline",
    "description": "Full description of the project.",
    "takeaway_line": "The key insight.",
    "directory": "/path/to/project",
    "user_name": ""
  }
}
```

## `project update-username <value>`

Updates the active project's username. Returns an error if no project is active.

```bash
harnessx project update-username "shaun"
```

```json
{
  "success": true,
  "data": {
    "id": "my-project",
    "title": "My Project",
    "subtitle": "A short tagline",
    "description": "Full description of the project.",
    "takeaway_line": "The key insight.",
    "directory": "/path/to/project",
    "user_name": "shaun"
  }
}
```
