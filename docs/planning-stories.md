# Planning Stories Commands

Manage planning stories for the active project. Stored at `harnessx/<id>/planning/planning_stories.json`.

Stories are single, testable behaviours. When a story is done, you can write one sentence describing what changed: "the system now does X" or "a user can now do Y." Each story belongs to an epic and is small enough to complete without a context switch.

## JSON Structure

The stories file wraps the array in a top-level object:

```json
{
  "stories": [
    {
      "id": "story-1",
      "order": 1,
      "title": "Fetch open positions from Uniswap v3 subgraph",
      "description": "When complete, running the ingestion module returns live positions...",
      "status": "not_started",
      "epic": "#epic-1",
      "depends_on": [],
      "acceptance_criteria": [
        "Given a wallet address with known positions, the module returns all active positions",
        "Each position includes token pair, tick range, liquidity amount, and pool address"
      ],
      "traces": {
        "tags": ["#action-1"],
        "intake_sources": ["#intake-scope", "#intake-resources"]
      },
      "notes": [
        { "note": "Pagination handling should be built into this story." }
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

Links a story back to intake artifacts for traceability.

| Field            | Type       | Description                                         |
|------------------|------------|-----------------------------------------------------|
| `tags`           | string[]   | References to action items (e.g. `#action-1`)       |
| `intake_sources` | string[]   | References to intake sections (e.g. `#intake-scope`) |

### Note

| Field  | Type   | Description  |
|--------|--------|--------------|
| `note` | string | Note content |

## `planning-stories create`

Creates a new story for the active project. The `id` is auto-assigned (`story-1`, `story-2`, ...) and `order` defaults to the next sequential value.

```bash
harnessx planning-stories create \
  --title "Fetch open positions from Uniswap v3 subgraph" \
  --description "When complete, running the ingestion module returns live positions" \
  --epic "#epic-1" \
  --status not_started \
  --depends-on "#story-1, #story-2" \
  --acceptance-criteria "Module returns all active positions | Each position includes token pair and tick range" \
  --trace-tags "#action-1" \
  --trace-intake-sources "#intake-scope, #intake-resources" \
  --note "Pagination handling should be built into this story."
```

| Flag                     | Required | Default        | Description                                          |
|--------------------------|----------|----------------|------------------------------------------------------|
| `--title`                | no       | `""`           | One-sentence description of the new behaviour        |
| `--description`          | no       | `""`           | What's testable once this story is complete          |
| `--order`                | no       | auto-increment | Explicit ordering; defaults to next sequential       |
| `--status`               | no       | `"not_started"` | Status (see enum above)                              |
| `--epic`                 | no       | `""`           | The epic this story belongs to                       |
| `--depends-on`           | no       | `""`           | Comma-separated story dependency references          |
| `--acceptance-criteria`  | no       | `""`           | **Pipe-separated** acceptance criteria (see note)    |
| `--trace-tags`           | no       | `""`           | Comma-separated action item tag references           |
| `--trace-intake-sources` | no       | `""`           | Comma-separated intake source references             |
| `--note`                 | no       | —              | Note content to attach                               |

> **Note on acceptance criteria:** Because acceptance criteria often contain commas in their text, they are separated by pipes (`|`) rather than commas. For example: `"Given X, the module returns Y | Each position includes token pair"`.

## `planning-stories list`

Lists all stories for the active project.

```bash
harnessx planning-stories list
```

## `planning-stories remove <id>`

Removes a story by its ID.

```bash
harnessx planning-stories remove story-1
```

## `planning-stories update <id>`

Updates fields on an existing story. Only provided flags are changed.

```bash
harnessx planning-stories update story-1 \
  --status in_progress \
  --note "Work has begun on the subgraph integration."
```

| Flag                     | Description                                               |
|--------------------------|-----------------------------------------------------------|
| `--title`                | New title                                                 |
| `--description`          | New description                                           |
| `--order`                | New order value                                           |
| `--status`               | New status                                                |
| `--epic`                 | New epic reference                                        |
| `--depends-on`           | Replacement comma-separated dependency references         |
| `--acceptance-criteria`  | Replacement **pipe-separated** acceptance criteria        |
| `--trace-tags`           | Replacement comma-separated trace tag refs                |
| `--trace-intake-sources` | Replacement comma-separated trace intake source refs      |
| `--note`                 | Note content to append                                    |

On update, notes are **appended** to the existing list (not replaced).
