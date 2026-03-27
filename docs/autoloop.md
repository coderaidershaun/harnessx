# Autoloop Command

Autonomous session loop for fully-autonomous pipeline stages. Continuously monitors for live Claude Code sessions and launches `autorun` when the coast is clear — preventing overlapping sessions on the same project.

Only operates during autonomous stages: `planning`, `review`, `execution`, `uat_rework`. Exits automatically when the project advances past these stages.

## `autoloop`

```bash
harnessx autoloop
```

Extra arguments are forwarded to the `claude` command:

```bash
harnessx autoloop --model sonnet
```

### Loop behaviour

1. Loads the active project ID via `harnessx project active`.
2. Checks `progress.json` — if the next incomplete stage is not one of `planning`, `review`, `execution`, or `uat_rework`, exits immediately.
3. Runs `harnessx session find <project-id>` to check for live sessions.
4. **If a live session exists:** waits 30 seconds, then re-checks (repeats).
5. **If no live session:** launches `harnessx autorun` (spawns `claude --dangerously-skip-permissions -p /hx:operator --output-format json`).
6. When `autorun` exits, re-checks the pipeline stage.
7. **If still in an autonomous stage:** loops back to step 3.
8. **If the project has moved past autonomous stages:** exits.

### Output

Status messages are printed to stderr during the loop. On exit, a JSON response is returned:

```json
{
  "success": true,
  "data": {
    "message": "Autoloop complete: project 'my-project' exited autonomous stages.",
    "project_id": "my-project",
    "runs": 3
  }
}
```

If the project is not in an autonomous stage at launch:

```json
{
  "success": true,
  "data": {
    "message": "Autoloop exited: project is not in an autonomous stage (planning, review, execution, uat_rework).",
    "project_id": "my-project",
    "runs": 0
  }
}
```

### Prerequisites

- A `harnessx/` directory must exist (run `harnessx init` first).
- An active project must be set.
- Sessions must have their `customTitle` set to the project ID for live-session detection to work.
