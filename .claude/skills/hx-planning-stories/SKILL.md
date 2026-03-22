---
name: hx:planning-stories
description: Define and write the stories needed to deliver a specific epic — single, testable behavioural increments that each produce a meaningful change in what the system can do. Given an epic (or auto-selecting the next one), reads all context, deeply analyzes what behaviours are missing, writes acceptance criteria, then writes stories to planning_stories.json with full traceability. Use this skill when the user says "write stories", "plan stories", "what stories does this epic need", "break down this epic", "define stories for epic-1", or anything about decomposing an epic into testable behaviours. Also trigger after epics are written and the next step is story decomposition, or when the operator routes to story planning.
disable-model-invocation: false
user-invocable: true
---

# Planning Stories

You define the stories for a specific epic — the individual, testable behaviours that each deliver a meaningful change in what the system can do. When a story is done, you can write one sentence: "the system now does X" or "a user can now do Y."

The test for whether something is a story vs. a task: if completing it produces a meaningful change in what the system can do (even a small one), it's a story. If it's a step toward that change, it's a task. Stories are small enough to complete without a context switch — you're working in one area of the codebase, one domain concept, one concern.

Your job is to look at an epic, understand what capability it delivers, and break that capability into the discrete testable behaviours that collectively make it real. Then write them using the harnessx CLI with acceptance criteria and full traceability.

---

## Step 1: Confirm active project and identify target epic

```bash
harnessx project active
```

If no active project exists, tell the user to set one and stop.

### Determine which epic to work on

The user may specify an epic directly (e.g., "write stories for epic-3") or you may need to find the next one:

```bash
# If user specified an epic, get its full details
harnessx planning-epics list

# If no epic specified, get the next incomplete one
harnessx planning-epics next
```

You need the epic's full object — its `title`, `description`, `milestone`, `categories`, and `traces`. The description tells you what capability is being built. The traces tell you which action items feed into this epic.

### Pre-built check

Before proceeding, check if this epic already has its stories pre-built:

If the epic has `stories_written: true`, it has pre-built structure from rework milestone generation. Report to the user: "This epic already has pre-built stories (from rework milestone setup). No additional story planning needed." Stop — do not create additional stories.

---

## Step 2: Read context

You need three layers of context: the epic's parent milestone, the epic itself with any existing children, and the intake material.

### Get the parent milestone

```bash
harnessx planning-epics parent <epic-id>
```

This returns the milestone the epic belongs to. Understanding the milestone's description and success measures keeps your stories aligned — every story should contribute toward the milestone being observable.

### Check existing children

```bash
harnessx planning-epics children <epic-id>
```

This returns all stories and tasks already under the epic. If stories exist, you're filling gaps — understand what's covered before planning what's missing.

### Read intake documents

Read in parallel the files most relevant to this epic's domain. At minimum:

- `intake_actions.json` — the action items this epic traces to (focus on the ones in the epic's `traces.tags`)
- `goal.md` and `scope.md` — to stay within bounds
- `success_measures.md` — to ensure stories ladder up to measurable outcomes
- Any `interview-*.md` files from agents whose domain overlaps this epic

All files are in `harnessx/<project-id>/intake/`. Not every file will exist.

### Read existing stories across all epics

```bash
harnessx planning-stories list
```

This shows all stories across all epics. Check for overlaps or dependencies with stories under other epics.

---

## Step 3: Analyze and plan stories

Use extended thinking (ultrathink) to decompose the epic into testable behaviours. Each story should be a meaningful increment — something you could demonstrate works independently, even if the full epic isn't done yet.

### What you're looking for

**From the epic:** Its description tells you what the system can do once the epic is complete. Break that end state into the discrete steps needed to get there. Each step that produces a testable change is a story.

**From the action items:** The epic's traces reference specific action items. Some action items map directly to stories. Others may need to be split across multiple stories. Some stories won't correspond to any action item — they emerge from the decomposition itself (e.g., a normalization step that nobody explicitly asked for but that must exist for the epic to work).

**From the scope:** Don't let stories drift beyond the epic's boundaries. If an action item is partially relevant to this epic and partially to another, only the relevant part becomes a story here.

### How to think about stories

A story is a **testable behavioural change**:

- **Good:** "Fetch open positions from Uniswap v3 subgraph" — you can run it and see data come back
- **Good:** "Normalize position data into a common Position struct" — you can verify the struct has the right fields
- **Bad:** "Work on the data layer" — too vague, not testable
- **Bad:** "Write unit tests" — that's a task within a story, not a story itself
- **Bad:** "Set up the GraphQL client" — that's a step toward a story (fetching data), not a testable behaviour on its own

Ask yourself: *when this is done, what can I test that I couldn't test before?* If you can write a concrete test description, it's a story. If you can only say "this code exists now," it might be a task.

### Acceptance criteria

Every story needs acceptance criteria — concrete, testable conditions that must be true for the story to be considered done. These are what an implementing agent will use to verify their work.

Good acceptance criteria are:

- **Specific:** "Given a wallet address with 3 active Uniswap v3 positions, the module returns all 3 with token pair, tick range, and liquidity amount" — not "it works correctly"
- **Testable:** You could write an automated test for each criterion
- **Complete:** The criteria collectively define "done" — nothing is left ambiguous
- **Independent:** Each criterion tests one thing

Write them in Given/When/Then format when it fits naturally, but don't force it — clarity matters more than format.

### Story ordering and dependencies

Stories within an epic should be ordered by dependency — foundational behaviours first, then behaviours that build on them:

1. Data flows in (fetch/ingest) → 2. Data gets transformed (parse/normalize) → 3. Data is usable (expose via API/display)

Stories can depend on other stories within the same epic, and occasionally on stories from other epics. Capture dependencies with `--depends-on`.

### Write your plan

For each story, capture:

- **Title** — one sentence describing the new behaviour
- **Description** — what's testable once this story is complete
- **Epic** — which epic this belongs to
- **Dependencies** — which other stories must be complete first
- **Acceptance criteria** — concrete conditions for "done" (these are critical)
- **Trace tags** — which action items this story implements (use `#action-N` tags)
- **Trace intake sources** — which intake sections informed this story
- **Notes** — implementation hints, edge cases, or risk flags

---

## Step 4: Review your plan

Launch a subagent to critique your story plan. The reviewer needs the epic context and the intake documents:

```
Review these proposed stories for epic [epic-id]: "[epic title]"

The epic delivers this capability: "[epic description]"
It belongs to milestone [milestone-id]: "[milestone title]"

[paste your story plan here]

The epic details and intake documents are at harnessx/<project-id>/. Read the epic (planning-epics list), its parent milestone (planning-epics parent <epic-id>), action items (intake/intake_actions.json), and relevant intake markdown.

Validate:

1. When all stories are done, is the epic's capability fully delivered? Or is there a gap — a behaviour that must exist but isn't covered?
2. Is each story a genuine testable behaviour (not a task or a vague category)?
3. Are the acceptance criteria specific and testable? Could an agent unambiguously determine whether each criterion is met?
4. Are there action items in the epic's traces not covered by any story?
5. Are dependencies correctly ordered? Could any stories be parallelized?
6. Are any stories too broad (should be split) or too narrow (should be merged with another)?
7. Is the story ordering optimal for incremental development — foundational behaviours first?

Provide specific, actionable critique. If acceptance criteria are vague, rewrite them to be concrete. If a story is missing, describe the behaviour gap.
```

Incorporate feedback. Pay special attention to acceptance criteria critique — vague criteria lead to ambiguous implementations.

---

## Step 5: Write stories via CLI

Before writing, read the CLI reference. Read `docs/planning-stories.md` if you haven't this session — especially the note about pipe-separated acceptance criteria.

### Creating stories

Use `harnessx planning-stories create` for each story. The CLI auto-assigns IDs (`story-1`, `story-2`, ...) and auto-increments `order`.

```bash
harnessx planning-stories create \
  --title "Fetch open positions from Uniswap v3 subgraph" \
  --description "When complete, running the ingestion module with a wallet address returns all active Uniswap v3 positions as structured data. This is the first integration point — proving the system can talk to a real data source." \
  --epic "#epic-1" \
  --status not_started \
  --acceptance-criteria "Given a wallet with known active positions, the module returns all of them | Each position includes token pair addresses, tick range (lower and upper), liquidity amount, and pool address | Pagination is handled automatically for wallets with more than 100 positions | The module returns an empty list (not an error) for wallets with no positions | Rate limiting from the subgraph results in automatic retry with backoff, not failure" \
  --trace-tags "#action-1" \
  --trace-intake-sources "#intake-scope, #intake-resources" \
  --note "Pagination handling is important — some whales have 200+ positions."
```

### Critical: acceptance criteria are pipe-separated

Because acceptance criteria often contain commas in their text, they use **pipe (`|`) separators** — not commas. This is different from every other list field in the CLI.

```bash
# CORRECT — pipe-separated
--acceptance-criteria "Given X, the module returns Y | Each position includes token pair | Empty wallets return empty list"

# WRONG — comma-separated (will split incorrectly)
--acceptance-criteria "Given X, the module returns Y, Each position includes token pair"
```

### Other important details

- **`--epic` uses the `#` prefix** — e.g., `--epic "#epic-1"`.
- **`--depends-on` is comma-separated** — e.g., `--depends-on "story-1, story-2"`.
- **IDs are auto-assigned** — capture the returned ID for tagging.
- **Status starts as `not_started`** during planning.
- **Update flags replace** (except notes, which append).

### Updating existing stories

```bash
harnessx planning-stories update story-1 \
  --acceptance-criteria "Updated criterion 1 | Updated criterion 2 | New criterion 3" \
  --note "Added criterion 3 after review identified missing edge case."
```

Remember: `--acceptance-criteria` on update **replaces** the existing list. Include all criteria, not just new ones.

---

## Step 6: Tag intake artifacts with story references

After creating stories, add `#story-N` tags back into intake documents for bidirectional traceability.

### Tag the intake markdown files

Find the paragraphs in intake markdown that relate to each story and add the tag inline.

**Example — tagging `scope.md`:**

```markdown
The ingestion system must handle Uniswap v3 subgraph queries with pagination for large wallets. #action-1 #story-1
```

**Example — tagging `resources.md`:**

```markdown
Uniswap v3 subgraph documentation and endpoint URL for position queries. #story-1 #story-2
```

### Tag action items with story references

For each action item a story traces to, append the story tag using `add-tag`. This creates the reverse link without replacing any existing tags.

```bash
harnessx intake-actions add-tag action-1 --tags "#story-1"
harnessx intake-actions update action-1 \
  --note-author "hx-planning-stories" \
  --note-text "Mapped to story-1: Fetch open positions from Uniswap v3 subgraph."
```

`add-tag` only appends — it will not remove any existing tags on the action item, and it skips duplicates.

### Tag epics with their stories

Update the epic's trace tags to include story references:

```bash
harnessx planning-epics update epic-1 \
  --trace-tags "#action-1, #action-4, #story-1, #story-2, #story-3" \
  --note "Stories defined: subgraph fetch, data parsing, normalization."
```

### Tagging rules (from hx:tag-context-writing)

- Tags go at the **end of the line** they annotate — never on their own line
- Only use **traceable tags** (`#story-N`, `#epic-N`, `#milestone-N`, `#action-N`)
- Do not invent categorical tags
- Verify with `harnessx context search-context --query "#story-1"` — should return meaningful paragraphs

---

## Step 7: Verify traceability

After writing all stories and tagging artifacts, verify bidirectional links:

```bash
# Verify each story is findable
harnessx context search-context --query "#story-1"
harnessx context search-context --query "#story-2"
# ... for each story

# Verify stories were created correctly
harnessx planning-stories list

# Verify the epic sees its children
harnessx planning-epics children <epic-id>
```

### Completeness check

Verify:
- Every action item in the epic's traces is covered by at least one story
- Every story has acceptance criteria (no empty criteria lists)
- Every story has at least one trace tag back to an action item
- The stories are collectively sufficient — all done means the epic's capability is delivered
- No story overlaps significantly with a story under another epic
- Acceptance criteria are specific enough that an implementing agent can unambiguously verify them

If you find gaps, go back and fix them.

---

## What good stories look like

- **Behaviour-focused titles** — "Fetch open positions from X" not "Implement fetcher"
- **Testable end state** — you can describe what to test when the story is done
- **Strong acceptance criteria** — specific, testable, complete. An agent reading only the criteria knows exactly what "done" means
- **Right granularity** — produces a meaningful change but doesn't require a context switch. One area of the codebase, one domain concept, one concern
- **Clean dependency chain** — foundational behaviours ordered first
- **Full traceability** — traces to action items and intake sources, tags in intake docs

---

## What this skill does NOT do

- **Write tasks** — tasks break stories into atomic implementation steps; that's a separate skill
- **Write epics or milestones** — those must exist before stories; use `hx:planning-epics` and `hx:planning-milestones`
- **Execute any implementation** — stories are plans with acceptance criteria, not code
- **Change the pipeline stage** — the operator handles stage transitions
- **Create action items** — action items come from intake; stories trace to existing ones
