# Planning Epics Commands

Manage planning epics for the active project. Stored at `harnessx/<id>/planning/planning_epics.json`.

Epics are coherent chunks of functionality that, when complete, give the system a new capability it didn't have before. Each epic belongs to a milestone, and they collectively make that milestone true. An epic typically maps to one or more action items from intake and often spans multiple categories.

## JSON Structure

The epics file wraps the array in a top-level object:

```json
{
  "epics": [
    {
      "id": "epic-1",
      "order": 1,
      "title": "DEX position ingestion",
      "description": "The system can pull live positions from Uniswap v3...",
      "status": "not_started",
      "milestone": "#milestone-1",
      "depends_on": [],
      "categories": ["backend", "infrastructure", "integration"],
      "traces": {
        "tags": ["#action-1", "#action-4"],
        "intake_sources": ["#intake-scope", "#intake-resources"]
      },
      "notes": [
        { "note": "This is the first epic in the pipeline." }
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

Links an epic back to intake artifacts for traceability.

| Field            | Type       | Description                                         |
|------------------|------------|-----------------------------------------------------|
| `tags`           | string[]   | References to action items (e.g. `#action-1`)       |
| `intake_sources` | string[]   | References to intake sections (e.g. `#intake-scope`) |

### Note

| Field  | Type   | Description  |
|--------|--------|--------------|
| `note` | string | Note content |

## `planning-epics next`

Returns the next incomplete epic by `order`. If all epics are completed, returns a completion message.

```bash
harnessx planning-epics next
```

Returns the full epic object so the calling agent has all context needed to act on it.

## `planning-epics create`

Creates a new epic for the active project. The `id` is auto-assigned (`epic-1`, `epic-2`, ...) and `order` defaults to the next sequential value.

```bash
harnessx planning-epics create \
  --title "DEX position ingestion" \
  --description "The system can pull live positions from Uniswap v3" \
  --milestone "#milestone-1" \
  --status not_started \
  --depends-on "#epic-1" \
  --categories "backend, infrastructure, integration" \
  --trace-tags "#action-1, #action-4" \
  --trace-intake-sources "#intake-scope, #intake-resources" \
  --note "This is the first epic in the pipeline."
```

| Flag                     | Required | Default        | Description                                     |
|--------------------------|----------|----------------|-------------------------------------------------|
| `--title`                | no       | `""`           | The capability being built                      |
| `--description`          | no       | `""`           | What the system can do once this epic is complete |
| `--order`                | no       | auto-increment | Explicit ordering; defaults to next sequential  |
| `--status`               | no       | `"not_started"` | Status (see enum above)                         |
| `--milestone`            | no       | `""`           | The milestone this epic contributes to          |
| `--depends-on`           | no       | `""`           | Comma-separated epic dependency references      |
| `--categories`           | no       | `""`           | Comma-separated categories (e.g. backend, frontend) |
| `--trace-tags`           | no       | `""`           | Comma-separated action item tag references      |
| `--trace-intake-sources` | no       | `""`           | Comma-separated intake source references        |
| `--note`                 | no       | —              | Note content to attach                          |

## `planning-epics list`

Lists all epics for the active project.

```bash
harnessx planning-epics list
```

## `planning-epics remove <id>`

Removes an epic by its ID.

```bash
harnessx planning-epics remove epic-1
```

## `planning-epics update <id>`

Updates fields on an existing epic. Only provided flags are changed.

```bash
harnessx planning-epics update epic-1 \
  --status in_progress \
  --note "Work has begun on the ingestion pipeline."
```

| Flag                     | Description                                          |
|--------------------------|------------------------------------------------------|
| `--title`                | New title                                            |
| `--description`          | New description                                      |
| `--order`                | New order value                                      |
| `--status`               | New status                                           |
| `--milestone`            | New milestone reference                              |
| `--depends-on`           | Replacement comma-separated dependency references    |
| `--categories`           | Replacement comma-separated categories               |
| `--trace-tags`           | Replacement comma-separated trace tag refs           |
| `--trace-intake-sources` | Replacement comma-separated trace intake source refs |
| `--note`                 | Note content to append                               |

On update, notes are **appended** to the existing list (not replaced).

## `planning-epics parent <id>`

Returns the milestone that this epic belongs to, resolved from the epic's `milestone` field.

```bash
harnessx planning-epics parent epic-1
```

Returns the full milestone object.

## `planning-epics children <id>`

Returns all stories and tasks that belong to an epic. Stories whose `epic` references this ID, and tasks whose `story` references those stories.

```bash
harnessx planning-epics children epic-1
```

Returns:
```json
{
  "epic": "epic-1",
  "stories": [...],
  "tasks": [...]
}
```
