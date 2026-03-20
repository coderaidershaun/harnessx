# Intake Team Commands

Tracks progress through intake team sections for the active project. Stored at `harnessx/<id>/intake/intake_team.json`.

Each section has `status` (a Status value), `agent` (defaults to `"opus"`), and `skills` (string array).

Sections (in order): `team_define`, `team_build`, `team_interview`.

## `intake-team init`

Initialises the default intake team progress file for the active project.

```bash
harnessx intake-team init
```

```json
{
  "success": true,
  "data": {
    "team_define": { "status": "not_started", "agent": "opus", "skills": [] },
    "team_build": { "status": "not_started", "agent": "opus", "skills": [] },
    "team_interview": { "status": "not_started", "agent": "opus", "skills": [] }
  }
}
```

## `intake-team status`

Shows intake team progress for the active project.

```bash
harnessx intake-team status
```

## `intake-team list`

Lists all intake team sections with their current status, agent, and skills.

```bash
harnessx intake-team list
```

```json
{
  "success": true,
  "data": [
    { "section": "team_define", "status": "not_started", "agent": "opus", "skills": [] },
    { "section": "team_build", "status": "not_started", "agent": "opus", "skills": [] },
    { "section": "team_interview", "status": "not_started", "agent": "opus", "skills": [] }
  ]
}
```

## `intake-team next`

Returns the next incomplete intake team section.

```bash
harnessx intake-team next
```

```json
{
  "success": true,
  "data": {
    "section": "team_define",
    "agent": "opus",
    "skills": []
  }
}
```

## `intake-team complete <section>`

Marks an intake team section's status as `completed`.

```bash
harnessx intake-team complete team_define
```
