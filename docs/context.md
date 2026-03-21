# Context Commands

Search for tags, wikilinks, and text across markdown files in the active project's `harnessx/<id>/` directory.

The CLI automatically resolves the active project and scopes all searches to its folder. It uses a built-in recursive search over `.md` and `.json` files.

Requires an active project. Returns `"no active project"` if none is set.

## `context search`

Returns file paths of `.md` files matching the query.

### Search by tag

```bash
harnessx context search --query "#my-tag"
```

```json
{
  "success": true,
  "data": {
    "backend": "builtin",
    "query": "#my-tag",
    "path": "harnessx/my-project",
    "results": [
      { "file": "harnessx/my-project/notes.md" },
      { "file": "harnessx/my-project/intake/scope.md" }
    ]
  }
}
```

### Search by wikilink

```bash
harnessx context search --query "[[some_link]]"
```

```json
{
  "success": true,
  "data": {
    "backend": "builtin",
    "query": "[[some_link]]",
    "path": "harnessx/my-project",
    "results": [
      { "file": "harnessx/my-project/research.md" }
    ]
  }
}
```

### Search by plain text

```bash
harnessx context search --query "authentication"
```

```json
{
  "success": true,
  "data": {
    "backend": "builtin",
    "query": "authentication",
    "path": "harnessx/my-project",
    "results": [
      { "file": "harnessx/my-project/intake/scope.md" },
      { "file": "harnessx/my-project/architecture.md" }
    ]
  }
}
```

## `context search-context`

Returns the paragraph containing each match, not just the file path. Paragraphs are delimited by blank lines.

### Tag with context

```bash
harnessx context search-context --query "#risk"
```

```json
{
  "success": true,
  "data": {
    "backend": "builtin",
    "query": "#risk",
    "path": "harnessx/my-project",
    "results": [
      {
        "file": "harnessx/my-project/planning.md",
        "context": "The main #risk is that the upstream API has no SLA and could change without notice."
      }
    ]
  }
}
```

### Wikilink with context

```bash
harnessx context search-context --query "[[auth_module]]"
```

```json
{
  "success": true,
  "data": {
    "backend": "builtin",
    "query": "[[auth_module]]",
    "path": "harnessx/my-project",
    "results": [
      {
        "file": "harnessx/my-project/architecture.md",
        "context": "The [[auth_module]] handles JWT validation and session refresh. See scope doc for boundaries."
      }
    ]
  }
}
```

## No active project

If no project is active, both commands return:

```json
{
  "success": false,
  "error": "no active project"
}
```

