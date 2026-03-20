# Obsidian CLI Reference

Quick reference for the [Obsidian CLI](https://github.com/obsidianmd/obsidian-cli) commands used in this project.

## Search

Find notes by content, tags, or wikilinks.

```bash
# Search by tag
obsidian search query="tag:#your-tag" format=json

# Search by wikilink (regex)
obsidian search query="/\[\[some_wikilink\]\]/" format=json
```

### Search with Context

Returns matching lines along with surrounding context, useful for understanding where a match appears within a note.

```bash
# Wikilink search with context
obsidian search:context query="/\[\[some_wikilink\]\]/" format=json

# Search within a specific section by tag
obsidian search:context query="section:(#your-tag)" format=json
```

## Outline

Display the heading structure of a note — helpful for understanding how a long document is organised before reading it.

```bash
obsidian outline file="Research Paper" format=md
```

## Properties

Obsidian notes support YAML frontmatter properties. The CLI can set these programmatically, which is useful for agent workflows that mark notes as processed.

### Set a property

```bash
obsidian property:set file="about" name="agent-status" value="analyzed"
```

### Query by property

```bash
# Find notes that do NOT have agent-status set to "analyzed"
obsidian search query="-[agent-status:analyzed]" format=json

# Find notes that DO have agent-status set to "analyzed"
obsidian search query="[agent-status:analyzed]" format=json
```

## Output Formats

Most commands accept a `format` flag:

| Value  | Description                          |
|--------|--------------------------------------|
| `json` | Machine-readable JSON output         |
| `md`   | Markdown-formatted output            |
