# Session Commands

Find live Claude Code sessions by their custom title. Searches `~/.claude/projects/` and `~/.claude/sessions/` to resolve matching sessions, then filters to only those with a live process.

## `session find <name>`

Finds all running Claude Code sessions whose `customTitle` matches the given name.

```bash
harnessx session find my-harnessx-session
```

```json
{
  "success": true,
  "data": [
    {
      "pid": "12345",
      "session_id": "abc-def-123",
      "project": "-Users-shaun-Code-my-project"
    }
  ]
}
```

Returns an empty array if no live sessions match.

### How it works

1. Scans `~/.claude/projects/<project>/*.jsonl` for files containing `"customTitle":"<name>"`
2. Cross-references the matched session ID against `~/.claude/sessions/<pid>.json` to resolve the PID
3. Filters to only sessions where the process is still alive (`kill -0 <pid>`)

### Fields

| Field | Description |
|---|---|
| `pid` | OS process ID of the Claude Code session |
| `session_id` | The Claude session UUID |
| `project` | The project directory name the session belongs to |
