# Intake Actions Commands

Manage action items for the active project. Stored at `harnessx/<id>/intake/intake_actions.json`.

## Types

### Complexity

Classifies how complex an action item is. Serialised as kebab-case in JSON.

| Value         | JSON value      |
|---------------|-----------------|
| `SuperLow`    | `"super-low"`   |
| `Low`         | `"low"`         |
| `Medium`      | `"medium"`      |
| `High`        | `"high"`        |
| `SuperHigh`   | `"super-high"`  |
| `Uncertain`   | `"uncertain"`   |

### ActionMode

The current mode for an action item. Serialised as kebab-case in JSON.

| Value     | JSON value    |
|-----------|---------------|
| `Plan`    | `"plan"`      |
| `Execute` | `"execute"`   |
| `Review`  | `"review"`    |
| `Rework`  | `"rework"`    |

### Note

Agents can attach notes to action items.

| Field    | Type   | Description          |
|----------|--------|----------------------|
| `author` | string | Author that wrote it |
| `note`   | string | Note content         |

## `intake-actions create`

Creates a new action item for the active project.

```bash
harnessx intake-actions create \
  --title "Implement auth" \
  --category "backend" \
  --complexity medium \
  --mode plan \
  --note-author "bot" \
  --note-text "initial note"
```

| Flag           | Required | Default | Description                                    |
|----------------|----------|---------|------------------------------------------------|
| `--title`      | no       | `""`    | Action item title                              |
| `--category`   | no       | `""`    | Category string                                |
| `--origin`     | no       | `""`    | Where the action originated                    |
| `--detail`     | no       | `""`    | Detailed description                           |
| `--tags`       | no       | `""`    | Comma-separated tags                           |
| `--input-docs` | no       | `""`    | Comma-separated input document references      |
| `--complexity` | no       | `""`    | Complexity level (see enum above)              |
| `--mode`       | no       | `""`    | Action mode (see enum above)                   |
| `--note-author` | no       | —       | Author name for note (requires `--note-text`)   |
| `--note-text`   | no       | —       | Note content (requires `--note-author`)         |

Notes are only created when both `--note-author` and `--note-text` are provided.

## `intake-actions list`

Lists all action items for the active project.

```bash
harnessx intake-actions list
```

## `intake-actions get <id>`

Returns a single action item by its ID.

```bash
harnessx intake-actions get action-1
```

Returns the full action item object, or an error if the ID doesn't exist.

## `intake-actions remove <id>`

Removes an action item by its numeric ID.

```bash
harnessx intake-actions remove 1
```

## `intake-actions update <id>`

Updates fields on an existing action item. Only provided flags are changed.

```bash
harnessx intake-actions update 1 \
  --complexity high \
  --mode execute \
  --note-author "reviewer" \
  --note-text "looks good"
```

| Flag           | Description                                         |
|----------------|-----------------------------------------------------|
| `--title`      | New title                                           |
| `--category`   | New category                                        |
| `--origin`     | New origin                                          |
| `--detail`     | New detail                                          |
| `--tags`       | Replacement comma-separated tags                    |
| `--input-docs` | Replacement comma-separated input docs              |
| `--complexity` | New complexity level                                |
| `--mode`       | New action mode                                     |
| `--note-author` | Author name for note to append (requires `--note-text`) |
| `--note-text`   | Note content to append (requires `--note-author`)    |

On update, notes are **appended** to the existing list (not replaced).
