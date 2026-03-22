---
name: hx:planning-milestones
description: Define and write project milestones by reading all intake documents, deeply analyzing what "done" looks like at each checkpoint, then writing milestones to planning_milestones.json with full traceability tags back to action items and intake sections. Use this skill when the pipeline reaches the planning stage, when the user says "write milestones", "plan milestones", "define milestones", "create milestones", "what are the milestones", or anything about breaking the project into demonstrable checkpoints. Also trigger when the operator routes to planning or when intake_completion is done and milestones are the next step.
disable-model-invocation: false
user-invocable: true
---

# Planning Milestones

You define the project's milestones — the demonstrable checkpoints where someone could look at the system and see real, observable progress. Milestones are not tasks or deliverables. They are states the project reaches when certain work is done.

A project should have roughly 3–7 milestones. More than that and they're too granular; fewer and there's no meaningful checkpoint between "started" and "done."

Your job is to read everything the intake process produced, deeply analyze what the project needs to accomplish, and define milestones that sequence the work into observable progress checkpoints — then write them using the harnessx CLI with full traceability back to the intake artifacts.

---

## Step 1: Confirm active project

```bash
harnessx project active
```

If no active project exists, tell the user to set one and stop.

---

## Step 2: Read all intake documents

Read **everything** in `harnessx/<project-id>/intake/`. Both markdown and JSON files. You need the complete picture before you can define milestones.

Read these files in parallel:

**Markdown files** (the narrative context):
- `goal.md` — what the project is trying to achieve
- `scope.md` — what's in and out of scope
- `user_knowledge.md` — the user's expertise and domain insights
- `resources.md` — repos, links, docs, reference material
- `success_measures.md` — concrete criteria for success
- `user_acceptance_testing.md` — what gets tested before sign-off
- `intake-team.md` — team composition and agent interview notes
- Any `interview-*.md` files — individual agent interview documents

**JSON files** (the structured data):
- `intake_actions.json` — all action items with their categories, complexities, and tags
- `intake_onboarding.json` — section completion status
- `intake_team.json` — team section status
- `intake_completion.json` — completion section status

Not every file will exist — read what's there without failing on missing files.

Also check if milestones already exist:

```bash
harnessx planning-milestones list
```

If milestones already exist, you are updating/refining them — not starting from scratch.

---

## Step 3: Analyze and plan milestones

This is where your deep thinking matters most. Use extended thinking (ultrathink) to work through this carefully — the quality of your milestones determines the quality of the entire plan downstream. Take your time. Consider multiple framings. Challenge your first instinct.

### What you're looking for

**From `success_measures.md`:** Each success measure is something the user defined as "this is what done looks like." Milestones should map naturally to these — a milestone is reached when one or more success measures become demonstrably true.

**From `user_acceptance_testing.md`:** UAT scenarios describe what the user will test before sign-off. Group related UAT scenarios under the milestone that enables them.

**From `goal.md` and `scope.md`:** These tell you the shape of the project. What's the core capability? What depends on what? Where are the natural vertical slices?

**From `intake_actions.json`:** Action items are the raw material. Look at their categories, complexities, and relationships. Clusters of related action items often point to a natural milestone boundary.

**From agent interviews:** Domain experts may have flagged dependencies, risks, or phasing recommendations that should influence milestone sequencing.

### How to think about milestones

A milestone is a **state**, not a task:

- **Good:** "Live position data flowing through the pipeline and visible in a basic UI"
- **Bad:** "Build the data pipeline" (that's work to be done, not a state to reach)

Ask yourself: *could I sit someone down and show them this is working?* If yes, it's a milestone. If you'd say "well, the code is done but you can't see it yet" — that's not a milestone.

### Dependency ordering

Milestones are sequenced because later ones depend on earlier ones being true. Think about what must exist before the next thing can work:

- Data must flow before you can calculate on it
- Calculations must work before you can display them
- Core features must exist before you can polish them
- Infrastructure must exist before services can run on it

### The thinnest vertical slice

Your first milestone should be the thinnest possible vertical slice — the smallest amount of work that produces something demonstrably real. This validates the architecture, the toolchain, the data flow, and the team's ability to ship. Everything else builds on this foundation.

### Write your plan

After thinking through the milestones, write out your plan as a structured list. For each milestone, capture:

- **Title** — short, state-based description
- **Description** — what someone would observe at this checkpoint
- **Dependencies** — which milestones must be complete first
- **Success measures** — which intake success measures this satisfies (use `#success-measure-N` tags)
- **UAT criteria** — which intake UAT scenarios this enables (use `#uat-scenario-N` tags)
- **Trace tags** — which action items this milestone encompasses (use `#action-N` tags)
- **Trace intake sources** — which intake sections informed this milestone (use `#intake-goal`, `#intake-scope`, etc.)
- **Notes** — any implementation notes, phasing rationale, or risk flags

---

## Step 4: Review your plan

Launch a subagent to review and critique your milestone plan. The reviewer should have access to the same intake documents and should challenge your milestones on these dimensions:

- **Coverage:** Do the milestones collectively cover all success measures and UAT criteria? Are there gaps?
- **Granularity:** Are any milestones too broad (trying to do too much) or too narrow (not a meaningful checkpoint)?
- **Sequencing:** Are the dependencies correct? Could any milestone be reached earlier with a different ordering?
- **Vertical slicing:** Is the first milestone truly the thinnest slice? Could it be thinner?
- **Observability:** Could someone actually see each milestone is met? Or are some milestones invisible internal states?
- **Action item coverage:** Are all action items traceable to at least one milestone? Are there orphaned actions?

Spawn the review agent like this:

```
Review these proposed milestones for the harnessx project.

[paste your milestone plan here]

The intake documents are at harnessx/<project-id>/intake/. Read the success measures, UAT criteria, goal, scope, and intake_actions.json to validate:

1. Do these milestones cover all success measures and UAT criteria?
2. Are any milestones too broad or too narrow?
3. Are dependencies correctly sequenced?
4. Is the first milestone the thinnest possible vertical slice?
5. Are there any orphaned action items not covered by any milestone?
6. Could someone actually observe each milestone is met, or are some milestones invisible internal states?

Provide specific, actionable critique. Don't just say "looks good" — find things to improve.
```

Incorporate the reviewer's feedback. If the critique is substantive, revise your plan before proceeding. If the reviewer identifies gaps or sequencing issues, fix them.

---

## Step 5: Write milestones via CLI

Before writing, read the CLI reference to confirm exact flags. Read `docs/planning-milestones.md` if you haven't this session — the flag names and behavior (especially which flags replace vs. append) matter.

### Creating milestones

Use `harnessx planning-milestones create` for each milestone. The CLI auto-assigns IDs (`milestone-1`, `milestone-2`, ...) and auto-increments the `order` field.

```bash
harnessx planning-milestones create \
  --title "Live position data flowing through the pipeline" \
  --description "A reviewer can open the app and see real positions appearing from on-chain data sources. Positions display correct token pairs, amounts, and ranges. Data refreshes automatically without manual intervention." \
  --status not_started \
  --success-measures "#success-measure-1, #success-measure-3" \
  --uat-criteria "#uat-scenario-1" \
  --trace-tags "#action-1, #action-4, #action-7" \
  --trace-intake-sources "#intake-goal, #intake-scope" \
  --note "Thinnest vertical slice — validates the data pipeline architecture end-to-end."
```

Write milestones in dependency order (first milestone first). If milestone-3 depends on milestone-1 and milestone-2:

```bash
harnessx planning-milestones create \
  --title "..." \
  --depends-on "milestone-1, milestone-2" \
  ...
```

### Important details

- **IDs are auto-assigned** — don't try to set them manually. The CLI returns JSON with the assigned ID.
- **Order auto-increments** — unless you need a specific order, let it auto-assign.
- **Status starts as `not_started`** — don't set it to anything else during planning.
- **Comma-separated lists** — `--depends-on`, `--success-measures`, `--uat-criteria`, `--trace-tags`, and `--trace-intake-sources` all accept comma-separated values.
- **Notes are appended** — if you update a milestone later, new notes add to existing ones rather than replacing.

### Updating existing milestones

If milestones already exist and you're refining them:

```bash
harnessx planning-milestones update milestone-1 \
  --title "Updated title" \
  --trace-tags "#action-1, #action-4, #action-7, #action-12" \
  --note "Added action-12 after scope review identified additional data source."
```

Update flags **replace** the existing values (except notes, which append). So if updating `--trace-tags`, include all tags — not just new ones.

---

## Step 6: Generate rework milestones

Every main milestone gets a companion "Review & rework" milestone. These rework milestones automatically review the completed work, dispatch review agents, and create rework tasks if issues are found. They are interleaved into the dependency chain so that each main milestone's output is reviewed before the next main milestone begins.

### Create rework milestones and rewire dependencies

For each main milestone you just created (in order), do the following:

**1. Create the rework milestone:**

```bash
# Create the rework milestone
harnessx planning-milestones create \
  --title "Review & rework: [main milestone title]" \
  --depends-on "#[main-milestone-id]" \
  --description "Autonomous review and rework of [title]. Runs all tests, dispatches review agents, creates rework tasks if issues are found." \
  --note "Auto-generated rework companion for [main-milestone-id]"
```

Capture the new rework milestone ID from the CLI response (it will be auto-assigned, e.g. `milestone-4` if the main milestones were `milestone-1` through `milestone-3`).

**2. Update the NEXT main milestone's dependency to point to the rework milestone:**

If there is a next main milestone that originally depended on the current main milestone, update it so it depends on the rework milestone instead. This rewires the dependency chain from `main → next main` to `main → rework → next main`.

```bash
# If milestone-3 originally depends on milestone-1, update it to depend on milestone-2 (the rework)
harnessx planning-milestones update [next-main-milestone-id] --depends-on "#[rework-milestone-id]"
```

The last main milestone also gets a rework companion — it simply has no "next" milestone to rewire, so skip this sub-step for the final one.

**3. Create the pre-built epic, story, and task for the rework milestone:**

Each rework milestone ships with a ready-to-execute epic/story/task so that downstream planning skills can skip them.

```bash
# Epic
harnessx planning-epics create \
  --title "[main title] review & rework" \
  --milestone "#[rework-milestone-id]" \
  --description "Review completed work for [main title] and fix issues found"

# Story
harnessx planning-stories create \
  --title "Assess delivered work and fix issues" \
  --epic "#[rework-epic-id]" \
  --acceptance-criteria "All unit tests pass|All integration tests pass|Success measures verified|No critical issues remain"

# Review task
harnessx planning-tasks create \
  --title "Review [main title] and write rework tasks" \
  --story "#[rework-story-id]" \
  --mode review \
  --skills "hx:milestone-rework-assessment" \
  --complexity medium \
  --steps "Run all unit and integration tests sequentially (cargo test -- --test-threads=1 and cargo test -- --ignored --test-threads=1)|Dispatch 4 review agents to assess test results, code quality, integration, and success measures|Synthesize findings into structured report|Create rework tasks for any Critical or Warning issues found|Create final verification task that depends on all rework tasks and re-runs all tests|If no issues found, report clean pass" \
  --note "Auto-generated review task for [main-milestone-id]"
```

Capture the epic, story, and task IDs from each CLI response for the next sub-step.

**4. Mark all rework items as "written" so downstream planning skills skip them:**

```bash
harnessx planning-milestones mark-written [rework-milestone-id]
harnessx planning-epics mark-written [rework-epic-id]
harnessx planning-stories mark-written [rework-story-id]
```

### After all rework milestones are created

The final dependency chain should look like:

```
main-1 → rework-1 → main-2 → rework-2 → main-3 → rework-3
```

Report to the user:

> "Generated N rework milestones as companions to the main milestones. These will automatically review completed work and create rework tasks if issues are found. The dependency chain ensures: main milestone → rework review → next main milestone."

Verify the chain with:

```bash
harnessx planning-milestones list
```

Confirm each rework milestone depends on its companion main milestone, and each subsequent main milestone depends on the preceding rework milestone.

---

## Step 7: Tag intake artifacts with milestone references

This is critical. After creating milestones, you must add `#milestone-N` tags back into the intake documents so that traceability works in both directions.

### Tag the intake markdown files

Find the paragraphs in the intake markdown files that relate to each milestone and add the milestone tag inline — at the end of the most relevant line.

**Example — tagging `success_measures.md`:**

If milestone-1 satisfies success measure 1, find the paragraph describing that success measure and add `#milestone-1`:

```markdown
The system displays live position data with less than 5 seconds of latency from on-chain confirmation. #milestone-1
```

**Example — tagging `goal.md`:**

```markdown
The core goal is real-time visibility into all open DEX positions across supported protocols. #milestone-1 #milestone-2
```

**Example — tagging `user_acceptance_testing.md`:**

```markdown
User can open the dashboard and see their current Uniswap v3 positions with correct token amounts. #milestone-1
```

### Tag action items with milestone references

For each action item that a milestone traces to, append the milestone tag using `add-tag`. This creates the reverse link — from action to milestone — without replacing any existing tags.

```bash
harnessx intake-actions add-tag action-1 --tags "#milestone-1"
harnessx intake-actions update action-1 \
  --note-author "hx-planning-milestones" \
  --note-text "Mapped to milestone-1: Live position data flowing through the pipeline."
```

`add-tag` only appends — it will not remove any existing tags on the action item, and it skips duplicates.

### Tagging rules (from hx:tag-context-writing)

- Tags go at the **end of the line** they annotate — never on their own line
- Only use **traceable tags** that reference real artifacts (`#milestone-N`, `#action-N`, `#success-measure-N`)
- Do not invent categorical tags
- After tagging, verify with `harnessx context search-context --query "#milestone-1"` — the result should return meaningful paragraphs, not just the tag

---

## Step 8: Verify traceability

After writing all milestones and tagging all artifacts, verify the bidirectional links work:

```bash
# Verify each milestone is findable
harnessx context search-context --query "#milestone-1"
harnessx context search-context --query "#milestone-2"
# ... for each milestone

# Verify milestones were created correctly
harnessx planning-milestones list
```

Each `search-context` query should return meaningful paragraphs from the intake documents — not just the tag itself. If any return only the tag, fix the placement (move the tag onto the content line).

### Completeness check

Verify:
- Every success measure is mapped to at least one milestone
- Every UAT criterion is mapped to at least one milestone
- Every action item traces to at least one milestone (no orphans)
- Every milestone has at least one trace tag back to an action item
- Every milestone has at least one intake source reference

If you find gaps, go back and fix them — either by updating milestone traces or by adding tags to intake documents.

---

## What a good set of milestones looks like

- **3–7 milestones** that collectively cover the entire project
- **State-based titles** — "X is working and observable" not "build X"
- **Clean dependency chain** — first milestone has no dependencies, last milestone depends on most others
- **Thinnest first slice** — milestone-1 is the smallest thing that proves the architecture works
- **Full traceability** — every milestone traces to action items and intake sections, every action item traces back to a milestone
- **Observable** — each milestone is something you could demo to someone

---

## What this skill does NOT do

- **Write epics, stories, or tasks** — those come from separate planning skills that operate within each milestone
- **Execute any implementation** — milestones are plans, not code
- **Change the pipeline stage** — the operator handles stage transitions
- **Create action items** — action items come from intake; milestones trace to existing ones
