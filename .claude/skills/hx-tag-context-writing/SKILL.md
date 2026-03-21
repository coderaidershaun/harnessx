---
name: hx:tag-context-writing
description: Write tags and wikilinks into markdown files so they are searchable with full surrounding context. Use this skill whenever you need to tag intake documents, action items, or any project markdown — including cross-referencing intake sections with their derived action items, linking action items back to their source discussions, or annotating any content that agents will need to find later. Also trigger when tagging headings, paragraphs, or discussion notes for downstream searchability, or when the user says "tag this", "link these", "make this searchable", or asks about how to format tags for context search.
disable-model-invocation: false
user-invocable: false
---

# Tag Context Writing

You place tags and wikilinks into markdown files so that `harnessx context search-context` returns useful paragraphs — not orphaned tags.

---

## Why placement matters

The context search system (`harnessx context search-context`) returns **paragraphs** — blocks of text delimited by blank lines. When you search for a tag, you get back the entire paragraph containing it. This means:

- A tag on **the same line** as a sentence → search returns that sentence (and its paragraph)
- A tag on **its own line** → search returns just the tag itself, which is useless

Every tag you write must be on the same line as the content it annotates. This is the single most important rule of this skill.

---

## Tag format

Tags follow the pattern `#project-id::tag-name` where `project-id` is the active project's ID and `tag-name` is a descriptive kebab-case identifier.

Get the project ID:

```bash
harnessx project active
```

Common tag patterns:

| Pattern | Purpose | Example |
|---|---|---|
| `#proj::action-N` | References action item #N | `#my-project::action-3` |
| `#proj::intake-section` | References an intake section | `#my-project::intake-goal` |
| `#proj::agent-name` | Traces which agent produced content | `#my-project::intake-specialist` |
| `#proj::custom-label` | Any project-specific label | `#my-project::auth-concerns` |

Wikilinks follow `[[project-id::link-name]]` — use these when you want bidirectional linking between documents. In practice, tags are used far more often because they're lighter and don't imply a target document exists.

---

## Placement rules

### Rule 1: Tags go at the end of the line they annotate

Append the tag after the content, separated by a space. Never put tags on their own line.

**Correct:**
```markdown
The auth module needs a complete rewrite due to token storage compliance issues. #my-project::action-7
```

**Wrong:**
```markdown
The auth module needs a complete rewrite due to token storage compliance issues.
#my-project::action-7
```

**Wrong:**
```markdown
#my-project::action-7
The auth module needs a complete rewrite due to token storage compliance issues.
```

### Rule 2: Tag headings when the heading itself is the context

If a heading describes a topic that should be findable, tag the heading line directly. This pulls the heading text into search results.

**Correct:**
```markdown
## Authentication concerns #my-project::action-7 #my-project::intake-scope
```

When someone searches for `#my-project::action-7`, they get back "Authentication concerns" — immediately useful.

### Rule 3: Tag the first or most descriptive line of a paragraph

For multi-line paragraphs, put the tag on the line that carries the most meaning. Usually that's the first line (the topic sentence), but use judgment — if the second line has the key detail, tag that one.

**Correct:**
```markdown
Users are churning at the password reset step because the email link expires in 60 seconds, #my-project::action-2
which isn't long enough for users on slow corporate mail systems.
The ops team confirmed this accounts for ~30% of support tickets.
```

### Rule 4: Multiple tags on one line are fine

When content relates to multiple things, stack the tags:

```markdown
## API rate limiting strategy #my-project::action-5 #my-project::intake-scope #my-project::auth-concerns
```

### Rule 5: Every cross-reference goes both ways

When tagging intake documents with action item references, also tag the action item's source material with the intake section reference. This creates a two-way link:

- **In `goal.md`**: The sentence that led to action #3 gets `#proj::action-3`
- **In action #3's detail or notes**: The origin gets `#proj::intake-goal`

Both directions need to be searchable for agents to trace provenance.

---

## Common tagging scenarios

### Tagging intake documents with derived action items

After creating action items during an intake section, go back to the intake markdown file and tag the specific sentences or paragraphs that each action item came from.

```markdown
## What the user described

They have an existing trading engine in Rust that processes limit orders #my-project::action-1
but needs to be extended to support market orders and stop-losses. #my-project::action-2

The current codebase has no test coverage, which is a risk for refactoring. #my-project::action-3 #my-project::action-4
```

This way, when an agent picks up action #2, it can search `#my-project::action-2` and immediately find the user's original description of what they need.

### Tagging action item notes with source references

When creating or updating action items, include the source tag in the `--detail` or `--note-text` so the action carries a pointer back:

```bash
harnessx intake-actions create \
  --title "Extend order engine to support market orders" \
  --origin "intake:scope" \
  --detail "User needs market order support added to existing Rust trading engine. See scope discussion for boundaries. #my-project::intake-scope" \
  --note-author "hx-intake-specialist" \
  --note-text "Derived from scope discussion about order types. #my-project::intake-scope"
```

### Tagging headings as section anchors

Tag section headings in intake docs so agents can find the right section fast:

```markdown
## Project goal #my-project::intake-goal

Build a real-time ops dashboard that replaces SSH-based monitoring. #my-project::action-1

## Scope boundaries #my-project::intake-scope

MVP covers the 5 core metrics only. #my-project::action-6
Mobile support is explicitly out of scope for this phase.
```

---

## Verifying your tags work

After tagging, verify searchability:

```bash
# Find all files containing a tag
harnessx context search --query "#my-project::action-3"

# Get the actual paragraph context (this is the real test)
harnessx context search-context --query "#my-project::action-3"
```

The `search-context` result should return a meaningful paragraph, not just the tag. If it returns only the tag, you've placed it on its own line — fix it.

---

## What not to do

- **Don't put tags in frontmatter** — YAML frontmatter is for properties (`obsidian property:set`), not inline tags. Tags in frontmatter won't be found by paragraph-level context search.
- **Don't cluster tags in a "tags section"** at the bottom of a file — this divorces them from their context. Every tag belongs next to the content it describes.
- **Don't use tags as decoration** — only tag content that an agent or person will actually need to find later. Tagging everything is the same as tagging nothing.
- **Don't invent new tag prefixes** without checking the project's existing conventions — consistency matters for search.
