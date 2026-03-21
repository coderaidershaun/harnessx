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

Tags follow the pattern `#tag-name` where `tag-name` is a descriptive kebab-case identifier. No project prefix is needed — searches are already scoped to the active project's folder.

Common tag patterns:

| Pattern | Purpose | Example |
|---|---|---|
| `#action-N` | References action item N — creates a bidirectional link between a paragraph and an action | `#action-3` |

**Only use tags that are traceable.** Every tag you write must reference something that actually exists in the project — an action item, another document, or a paragraph. Do not invent categorical tags (`#exploration`, `#blindspot-api-drift`, `#goal`) that don't exist as searchable content elsewhere. Categorical information belongs in dedicated fields (`category`, `origin`, `note-text`) on action items, not as tags in markdown files.

Wikilinks follow `[[link-name]]` — use these when you want bidirectional linking between documents.

---

## Placement rules

### Rule 1: Tags go at the end of the line they annotate

Append the tag after the content, separated by a space. Never put tags on their own line.

**Correct:**
```markdown
The auth module needs a complete rewrite due to token storage compliance issues. #action-7
```

**Wrong:**
```markdown
The auth module needs a complete rewrite due to token storage compliance issues.
#action-7
```

**Wrong:**
```markdown
#action-7
The auth module needs a complete rewrite due to token storage compliance issues.
```

### Rule 2: Tag headings when the heading itself is the context

If a heading describes a topic that should be findable, tag the heading line directly. This pulls the heading text into search results.

**Correct:**
```markdown
## Authentication concerns #action-7
```

When someone searches for `#action-7`, they get back "Authentication concerns" — immediately useful.

### Rule 3: Tag the first or most descriptive line of a paragraph

For multi-line paragraphs, put the tag on the line that carries the most meaning. Usually that's the first line (the topic sentence), but use judgment — if the second line has the key detail, tag that one.

**Correct:**
```markdown
Users are churning at the password reset step because the email link expires in 60 seconds, #action-2
which isn't long enough for users on slow corporate mail systems.
The ops team confirmed this accounts for ~30% of support tickets.
```

### Rule 4: Multiple tags on one line are fine

When content relates to multiple things, stack the tags:

```markdown
## API rate limiting strategy #action-5
```

### Rule 5: Every cross-reference goes both ways

When tagging intake documents with action item references, also tag the action item's source material with the intake section reference. This creates a two-way link:

- **In `goal.md`**: The sentence that led to action-3 gets `#action-3`
- **In action-3**: The `origin` field (`intake:goal`) traces back to the source section

The markdown tag (`#action-3`) is searchable via `harnessx context search-context`, and the action's `origin` field provides the reverse link.

---

## Common tagging scenarios

### Tagging intake documents with derived action items

After creating action items during an intake section, go back to the intake markdown file and tag the specific sentences or paragraphs that each action item came from.

```markdown
## What the user described

They have an existing trading engine in Rust that processes limit orders #action-1
but needs to be extended to support market orders and stop-losses. #action-2

The current codebase has no test coverage, which is a risk for refactoring. #action-3 #action-4
```

This way, when an agent picks up action-2, it can search `#action-2` and immediately find the user's original description of what they need.

### Tagging action item notes with source references

When creating or updating action items, the `origin` field (`intake:scope`, `intake:goal`, etc.) traces back to the source section. Use the action's dedicated fields for categorization — not inline tags in `--detail` or `--note-text`:

```bash
harnessx intake-actions create \
  --title "Extend order engine to support market orders" \
  --category "implementation" \
  --origin "intake:scope" \
  --detail "User needs market order support added to existing Rust trading engine. See scope discussion for boundaries." \
  --note-author "hx-intake-specialist" \
  --note-text "Derived from scope discussion about order types."
```

### Tagging headings as section anchors

Tag section headings in intake docs so agents can find the right section fast:

```markdown
## Project goal

Build a real-time ops dashboard that replaces SSH-based monitoring. #action-1

## Scope boundaries

MVP covers the 5 core metrics only. #action-6
Mobile support is explicitly out of scope for this phase.
```

---

## Verifying your tags work

After tagging, verify searchability:

```bash
# Find all files containing a tag
harnessx context search --query "#action-3"

# Get the actual paragraph context (this is the real test)
harnessx context search-context --query "#action-3"
```

The `search-context` result should return a meaningful paragraph, not just the tag. If it returns only the tag, you've placed it on its own line — fix it.

---

## What not to do

- **Don't put tags in frontmatter** — YAML frontmatter is for properties, not inline tags. Tags in frontmatter won't be found by paragraph-level context search.
- **Don't cluster tags in a "tags section"** at the bottom of a file — this divorces them from their context. Every tag belongs next to the content it describes.
- **Don't use tags as decoration** — only tag content that an agent or person will actually need to find later. Tagging everything is the same as tagging nothing.
- **Don't invent untraceable categorical tags** — tags like `#exploration`, `#blindspot-api-drift`, `#goal`, or `#verification` are categories, not links to real content. They create noise. Use dedicated fields (`category`, `origin`, `note-text`) on action items for categorical information instead.
- **Don't invent new tag prefixes** without checking the project's existing conventions — consistency matters for search.
