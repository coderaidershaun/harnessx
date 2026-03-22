---
name: hx:planning-epics
description: Define and write the epics needed to reach a specific milestone — coherent capability chunks that collectively make the milestone true. Given a milestone (or auto-selecting the next one), reads all intake documents and existing planning artifacts, deeply analyzes what capabilities are missing, then writes epics to planning_epics.json with full traceability. Use this skill when the user says "write epics", "plan epics", "what epics does this milestone need", "break down this milestone", "define epics for milestone-1", or anything about decomposing a milestone into capability chunks. Also trigger after milestones are written and the next step is epic decomposition, or when the operator routes to epic planning.
disable-model-invocation: false
user-invocable: true
---

# Planning Epics

You define the epics for a specific milestone — the coherent capability chunks that, when all complete, make that milestone demonstrably true. An epic gives the system a new capability it didn't have before. Epics are capabilities, not categories — "PnL calculation engine" is an epic; "backend work" is not.

Your job is to look at a milestone, understand what needs to be true for it to be reached, and identify the distinct capabilities that must exist. Then write the missing epics using the harnessx CLI with full traceability.

---

## Step 1: Confirm active project and identify target milestone

```bash
harnessx project active
```

If no active project exists, tell the user to set one and stop.

### Determine which milestone to work on

The user may specify a milestone directly (e.g., "write epics for milestone-2") or you may need to find the next one:

```bash
# If user specified a milestone, get its full details
harnessx planning-milestones list

# If no milestone specified, get the next incomplete one
harnessx planning-milestones next
```

You need the milestone's full object — especially its `traces` (which action items and intake sources it maps to), `success_measures`, `uat_criteria`, and `description`. This is your north star for what the epics must collectively achieve.

### Pre-built check

Before proceeding, check if this milestone already has its epics pre-built:

If the milestone has `epics_written: true`, it has pre-built structure from rework milestone generation. Report to the user: "This milestone already has pre-built epics (from rework milestone setup). No additional epic planning needed." Stop — do not create additional epics.

This check is important for rework milestones, which are auto-generated with a single pre-built epic/story/task structure that should not be modified during planning.

---

## Step 2: Read context

You need two types of context: the milestone's world (what needs to be true) and the intake world (the raw material to draw from).

### Read the milestone's existing children

Check what epics already exist for this milestone:

```bash
harnessx planning-milestones children <milestone-id>
```

This returns all epics, stories, and tasks under the milestone. If epics already exist, you're filling gaps — not starting from scratch. Understand what's covered before planning what's missing.

### Read intake documents

Read the intake documents in parallel — you need the full picture to make good epic decisions:

**Markdown files** (narrative context):
- `goal.md` — the project's purpose
- `scope.md` — what's in and out of scope
- `success_measures.md` — concrete success criteria
- `user_acceptance_testing.md` — what gets tested before sign-off
- `resources.md` — repos, docs, reference material
- `user_knowledge.md` — the user's expertise and domain insights
- `intake-team.md` and any `interview-*.md` files — agent interview notes

**JSON files** (structured data):
- `intake_actions.json` — all action items with categories, complexities, tags, and relationships

All files are in `harnessx/<project-id>/intake/`. Not every file will exist — read what's there.

### Also read existing epics

```bash
harnessx planning-epics list
```

This shows all epics across all milestones. Understanding the full epic landscape helps you avoid overlaps and spot dependencies between milestones.

---

## Step 3: Analyze and plan epics

Use extended thinking (ultrathink) to deeply analyze what capabilities are needed. This step determines whether the milestone can actually be reached, so think carefully.

### What you're looking for

**From the milestone itself:** Its description tells you what someone would observe. Its traces tell you which action items are in play. Work backwards — what capabilities must exist for this observation to be possible?

**From the action items:** The milestone's `traces.tags` reference specific action items. Group related action items into clusters — each cluster often maps to an epic. But don't just mechanically group by category. Ask: "what capability does this cluster deliver?"

**From scope and success measures:** These constrain what the epics should cover. If the scope excludes mobile, no epic should touch mobile. If a success measure requires sub-second latency, the relevant epic needs to account for that.

**From agent interviews:** Domain experts may have identified capability boundaries, integration points, or natural seams in the architecture that suggest where epic boundaries should fall.

### How to think about epics

An epic is a **capability**, not a category or a task list:

- **Good:** "DEX position ingestion" — the system can now pull positions from on-chain sources
- **Good:** "PnL calculation engine" — the system can now compute profit and loss for any position
- **Bad:** "Backend work" — that's a category, not a capability
- **Bad:** "Write the Rust code" — that's a task, not a capability
- **Bad:** "Set up infrastructure" — too vague; what capability does the infrastructure enable?

Ask yourself: *what can the system do after this epic is done that it couldn't do before?* If you can answer that clearly in one sentence, it's a well-scoped epic.

### Collectively sufficient

The epics under a milestone must be collectively sufficient — when all are done, the milestone must be true. Check this by working backwards from the milestone's description:

> "A reviewer can open the app and see real positions appearing from on-chain data sources."

What must exist for this to be true?
1. Something that pulls data from on-chain sources → **ingestion epic**
2. Something that transforms raw data into displayable positions → **normalization epic**
3. Something that shows positions to a user → **position list view epic**

All three are necessary. None alone is sufficient. If you removed any one, the milestone wouldn't be met.

### Dependencies between epics

Epics within a milestone can depend on each other. If the normalization epic needs raw data to exist first, it depends on the ingestion epic. Capture these with `--depends-on`.

Epics can also depend on epics from other milestones — this is less common but valid when a capability from an earlier milestone is a prerequisite.

### Categories

Each epic spans one or more categories that describe the areas of work involved. Common categories:

- `backend`, `frontend`, `infrastructure`, `integration`, `data`, `design`, `testing`, `documentation`, `research`

Categories help planning agents assign the right specialist skills later. An ingestion epic might span `backend`, `infrastructure`, and `integration`.

### Write your plan

For each epic you propose, capture:

- **Title** — the capability being built (capability-focused, not task-focused)
- **Description** — what the system can do once this is complete that it couldn't do before
- **Milestone** — which milestone this epic contributes to
- **Dependencies** — which other epics must be complete first
- **Categories** — areas of work involved
- **Trace tags** — which action items this epic addresses (use `#action-N` tags)
- **Trace intake sources** — which intake sections informed this epic
- **Notes** — implementation considerations, risk flags, or phasing rationale

---

## Step 4: Review your plan

Launch a subagent to critique your epic plan. The reviewer should read the milestone, intake docs, and existing epics to challenge your decomposition:

```
Review these proposed epics for milestone [milestone-id]: "[milestone title]"

[paste your epic plan here]

The milestone details and intake documents are at harnessx/<project-id>/. Read the milestone (planning-milestones list), existing epics (planning-epics list), action items (intake/intake_actions.json), and relevant intake markdown files.

Validate:

1. Are these epics collectively sufficient? If all are done, is the milestone truly met — or is there a gap?
2. Is each epic a genuine capability (not a category or task list)?
3. Are there action items in the milestone's traces that aren't covered by any proposed epic?
4. Are the dependencies between epics correct? Could any be parallelized that are currently serialized?
5. Are any epics too broad (should be split) or too narrow (should be merged)?
6. Do the categories accurately reflect the work areas involved?

Provide specific, actionable critique. If you find gaps, name the missing capability. If an epic is too broad, suggest how to split it.
```

Incorporate the reviewer's feedback before writing. If they identify a gap (a capability that must exist for the milestone to be met but isn't covered), add an epic for it.

---

## Step 5: Write epics via CLI

Before writing, read the CLI reference to confirm exact flags. Read `docs/planning-epics.md` if you haven't this session.

### Creating epics

Use `harnessx planning-epics create` for each epic. The CLI auto-assigns IDs (`epic-1`, `epic-2`, ...) and auto-increments the `order` field.

```bash
harnessx planning-epics create \
  --title "DEX position ingestion" \
  --description "The system can pull live positions from Uniswap v3 and other supported DEXs, handling rate limits, connection failures, and schema variations. Raw position data is available to downstream consumers." \
  --milestone "#milestone-1" \
  --status not_started \
  --categories "backend, infrastructure, integration" \
  --trace-tags "#action-1, #action-4" \
  --trace-intake-sources "#intake-scope, #intake-resources" \
  --note "First epic in the pipeline — unblocks normalization and display."
```

Write epics in dependency order. If an epic depends on another:

```bash
harnessx planning-epics create \
  --title "Position normalization" \
  --description "Raw position data from any supported DEX is transformed into a common Position struct..." \
  --milestone "#milestone-1" \
  --depends-on "epic-1" \
  --categories "backend, data" \
  --trace-tags "#action-5, #action-6" \
  --trace-intake-sources "#intake-scope" \
  --note "Depends on ingestion epic providing raw data."
```

### Important details

- **IDs are auto-assigned** — the CLI returns JSON with the assigned ID. Capture it for tagging.
- **`--milestone` uses the `#` prefix** — e.g., `--milestone "#milestone-1"`.
- **`--depends-on` references other epics** — e.g., `--depends-on "epic-1, epic-2"` (no `#` prefix in the depends-on values, but the JSON stores them; check the docs).
- **`--categories` is comma-separated** — e.g., `--categories "backend, infrastructure"`.
- **Update flags replace** (except notes, which append). If updating `--trace-tags`, include all tags.
- **Status starts as `not_started`** during planning.

### Updating existing epics

If some epics already exist and you're filling gaps or refining:

```bash
harnessx planning-epics update epic-1 \
  --trace-tags "#action-1, #action-4, #action-9" \
  --note "Added action-9 after review identified additional data source requirement."
```

---

## Step 6: Tag intake artifacts with epic references

After creating epics, add `#epic-N` tags back into intake documents for bidirectional traceability.

### Tag the intake markdown files

Find the paragraphs in intake markdown that relate to each epic and add the tag inline — at the end of the most relevant line.

**Example — tagging `scope.md`:**

```markdown
The system must support Uniswap v3 position ingestion with automatic retry on RPC failures. #action-1 #epic-1
```

**Example — tagging `goal.md`:**

```markdown
Real-time PnL requires normalized position data across all supported protocols. #epic-2
```

### Tag action items with epic references

For each action item an epic traces to, append the epic tag using `add-tag`. This creates the reverse link without replacing any existing tags.

```bash
harnessx intake-actions add-tag action-1 --tags "#epic-1"
harnessx intake-actions update action-1 \
  --note-author "hx-planning-epics" \
  --note-text "Mapped to epic-1: DEX position ingestion."
```

`add-tag` only appends — it will not remove any existing tags on the action item, and it skips duplicates.

### Tag milestones with their epics

Update the milestone's trace tags to include the epic references, so the milestone knows which epics contribute to it:

```bash
# Get current milestone traces first
harnessx planning-milestones list

# Update with existing tags plus new epic tags
harnessx planning-milestones update milestone-1 \
  --trace-tags "#action-1, #action-4, #epic-1, #epic-2, #epic-3" \
  --note "Epics defined: ingestion, normalization, position list view."
```

### Tagging rules (from hx:tag-context-writing)

- Tags go at the **end of the line** they annotate — never on their own line
- Only use **traceable tags** that reference real artifacts (`#epic-N`, `#milestone-N`, `#action-N`)
- Do not invent categorical tags
- After tagging, verify with `harnessx context search-context --query "#epic-1"` — the result should return meaningful paragraphs, not just the tag

---

## Step 7: Verify traceability

After writing all epics and tagging artifacts, verify bidirectional links work:

```bash
# Verify each epic is findable in context
harnessx context search-context --query "#epic-1"
harnessx context search-context --query "#epic-2"
# ... for each epic

# Verify epics were created correctly
harnessx planning-epics list

# Verify the milestone sees its children
harnessx planning-milestones children <milestone-id>
```

Each `search-context` query should return meaningful paragraphs from intake documents — not just the tag. If any return only the tag, fix the placement.

### Completeness check

Verify:
- Every action item in the milestone's traces is covered by at least one epic
- Every epic has at least one trace tag back to an action item
- Every epic has at least one intake source reference
- The epics are collectively sufficient — all done means the milestone is met
- No epic overlaps significantly with an existing epic (from any milestone)

If you find gaps, go back and fix them — either by creating additional epics or updating traces.

---

## What good epics look like

- **Capability-focused titles** — "Position normalization engine" not "Backend data work"
- **Clear before/after** — you can describe what the system can do after the epic that it couldn't before
- **Collectively sufficient** — all epics under a milestone together make it true
- **Independently describable** — you can explain one epic without referencing the others
- **Correct dependencies** — parallelizable epics don't have unnecessary serial dependencies
- **Right granularity** — not so broad it's a milestone, not so narrow it's a story
- **Full traceability** — traces to action items and intake sources, tags in intake docs

---

## What this skill does NOT do

- **Write stories or tasks** — those come from separate planning skills that operate within each epic
- **Write milestones** — milestones must exist before epics; use `hx:planning-milestones` for that
- **Execute any implementation** — epics are plans, not code
- **Change the pipeline stage** — the operator handles stage transitions
- **Create action items** — action items come from intake; epics trace to existing ones
