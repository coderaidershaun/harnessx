---
name: hx:planning-tasks
description: Define and write the implementation tasks needed to deliver a milestone — right-sized units of work (15-60 minutes each) that a specialist agent can complete in a single focused session. Given a milestone (from the coordinator), reads full context (intake docs, prior milestone handoff), decomposes into 5-12 tasks with group labels, purpose fields, strict execution ordering, skill assignments, complexity ratings, steps, integration tests, and full traceability. Tasks belong directly to milestones (no epics or stories). Use this skill when the user says "write tasks", "plan tasks", or anything about decomposing milestones into implementation work. Also trigger after milestones are written and the next step is task decomposition, or when the operator routes to task planning.
disable-model-invocation: false
user-invocable: false
---

# Planning Tasks

You define the tasks for a milestone — the implementation steps that a specialist agent will actually sit down and execute. Tasks belong directly to milestones with optional `group` labels for organization. No epics or stories.

Each task has a clear start, a clear end, and can be finished in a single focused session. When a task is done, the agent can point to a concrete change: a new file, a modified function, a passing test.

Your job is to look at the target milestone, understand what it needs to deliver, and break that into discrete implementation steps that collectively make the milestone's success measures pass. Then write them using the harnessx CLI with the right skill assignments, complexity ratings, and full traceability.

**Scope discipline:** You work on ALL tasks for a single milestone in one session. Once all tasks for the milestone are written, you stop.

---

## Task Sizing Rules

These rules are critical. Follow them precisely.

### The 15-60 Minute Rule
Each task should represent 15-60 minutes of focused agent work.
- **< 10 minutes?** Merge with an adjacent task (e.g., "add .env to .gitignore" merges into "project skeleton")
- **> 90 minutes?** Split it (e.g., "test all 28 endpoints" splits into 3-4 by API surface)

### The "One Meaningful Change" Test
After this task completes, can you describe what changed in one sentence?
- "The project compiles with all SDK deps and env is configured" — meaningful
- "Added .env to .gitignore" — trivial, merge with setup task
- "SafeTestOrder guard is built and tested" — meaningful (even though substantial)

### Count Targets
- **Per milestone:** 5-12 tasks (target 8)
- **Per project:** 25-50 tasks total
- If a milestone would have > 12 tasks, the milestone should have been split
- If a milestone would have < 3 tasks, it should have been merged

### Batch-Eligible Tasks
- `super-low` or `low` complexity tasks sharing a `group` with an adjacent task
- Maximum 3 tasks per batch
- Link via `--batch-with` field

---

## Step 1: Confirm active project and identify target milestone

```bash
harnessx project active
```

If no active project exists, tell the user to set one and stop.

### Get the target milestone and its children

```bash
harnessx planning-milestones next-to-write-tasks
```

This returns the next milestone with `tasks_written: false`. Capture the milestone ID.

```bash
harnessx planning-milestones children <milestone-id>
```

This returns any existing tasks under this milestone (for resume scenarios). If tasks already exist, you're filling gaps — not starting from scratch.

---

## Step 2: Read the full context

### Milestone context (already gathered)

You already have the milestone from Step 1. Its description and success measures define what the tasks must collectively deliver.

### Prior milestone context (critical for continuity)

If this is NOT the first milestone (i.e., `milestone.depends_on` references earlier milestones), you must understand what those milestones produced.

```bash
# For each milestone this one depends on, load it and read its handoff notes
harnessx planning-milestones get <prior-milestone-id>
```

Read the **notes** on each prior milestone — especially any note starting with "HANDOFF:". These contain:
- Key output files and what's in them
- Exit-point task IDs whose outputs downstream work builds on
- Interfaces and contracts — function signatures, struct shapes, API patterns

If no handoff notes exist, fall back to loading prior milestone tasks:

```bash
harnessx planning-milestones children <prior-milestone-id>
```

Scan the tasks' `traces.output_sources` fields to build a picture of what files and modules were created.

**Key principle:** Every task step that says "Read the existing X" must reference something that a prior milestone's task actually produces. If you can't trace it to a specific output_source, the step is making an assumption.

### Check existing tasks across the project

```bash
harnessx planning-tasks list
```

### Read intake documents

Read in parallel the files most relevant to this milestone's domain. At minimum:

- `intake_actions.json` — the action items this milestone traces to
- `goal.md` and `scope.md` — to stay within bounds
- `success_measures.md` — to ensure tasks ladder up to measurable outcomes
- Any `interview-*.md` files from agents whose domain overlaps this milestone

All files are in `harnessx/<project-id>/intake/`. Not every file will exist.

### Catalog available specialist skills

```bash
ls .claude/skills/
```

Identify which skill families exist and whether each has a **team lead** (coordinator). Common families:

- **`rust:*`** — Rust development. **Team lead: `rust:team-coordinator`**
- **`mermaid-diagrams`** — Diagram creation (standalone)
- **`research:reducer`** — URL analysis and distillation (standalone)
- **Other teams** — Any skills created during intake team

When a team lead exists, **assign the team lead by default**. Only assign directly to a specialist for trivially single-concern tasks (commenting, simple refactoring).

---

## Step 3: Decompose into tasks

Use extended thinking (ultrathink) to work through decomposition carefully. You are producing the FINAL task list — no dual-agent review cycle needed.

### What to produce for each task

1. **Title** — specific, actionable (verb + noun + context)
2. **Purpose** — WHY this task exists, what it enables
3. **Group** — lightweight label grouping related tasks (e.g. "setup", "harness", "ws-market")
4. **Execution order** — strict integer ordering (1, 2, 3, ...) — the order tasks will execute
5. **Steps** — ordered implementation steps (3-8 per task). Concrete enough for an agent to follow without guessing
6. **Complexity** — super-low, low, medium, high, or super-high
7. **Skills** — which specialist skill(s) should execute this task
8. **Integration tests** — how to verify this specific task works
9. **Output sources** — which files this task will create or modify
10. **Trace tags** — which action items this task implements
11. **Batch with** — if this task should execute in the same session as an adjacent task
12. **Notes** — implementation hints, edge cases, risk flags

### Decomposition guidelines

- **Target 5-12 tasks** for this milestone. If you're over 12, you're decomposing too finely.
- **Each task should produce a meaningful, testable change.** If a task is "add a dependency to Cargo.toml," merge it into the task that uses that dependency.
- **Group related tasks** with the same `group` label. Groups are just strings — use whatever makes the grouping clear.
- **Execution order is strict.** Task 5 can assume tasks 1-4 are done. No need for `depends_on` within a milestone.
- **Use `depends_on` only for cross-milestone references** (rare).
- **Skill assignment matters.** Team lead for non-trivial work, direct specialist only for trivially simple tasks.
- **Steps are the agent's roadmap.** Write them as instructions: "Read the existing Position struct in src/models/position.rs" not "Understand the data model."

### Merge pass

After decomposition, scan your task list for:
- Adjacent tasks with the same group that are each < 10 minutes → merge them
- Tasks that only modify one line or add one dependency → merge with the task that uses the change
- Tasks that test something just created → consider merging the impl+test if total is under 60 minutes

---

## Step 4: Write tasks via CLI

Before writing, read `docs/planning-tasks.md` if you haven't this session — especially the notes about pipe-separated fields and v2 flags.

### Creating tasks

Use `harnessx planning-tasks create` for each task. The CLI auto-assigns IDs (`task-1`, `task-2`, ...).

```bash
harnessx planning-tasks create \
  --milestone "#milestone-1" \
  --title "Project skeleton: Cargo.toml, .env, .gitignore, lib.rs" \
  --purpose "Bootstrap the project so all subsequent tasks can compile and run" \
  --group "setup" \
  --execution-order 1 \
  --steps "Configure Cargo.toml with SDK dependencies and all 6 feature flags | Create .env.example with placeholder keys and add .env to .gitignore | Set up src/lib.rs as crate root with pub mod declarations | Run cargo check to verify compilation" \
  --status not_started \
  --complexity low \
  --mode plan \
  --skills "rust:team-coordinator" \
  --integration-tests "cargo build exits with code 0 | .env.example exists with all required keys" \
  --trace-tags "#action-1, #action-23" \
  --trace-intake-sources "#intake-resources, #intake-scope" \
  --trace-output-sources "Cargo.toml, src/lib.rs, .env.example" \
  --note "Ensure edition = 2024 and all 6 SDK features enabled."
```

### Critical details

- **`--milestone` is required for v2.** Uses `#` prefix. Determines shard path: `planning/tasks/<milestone-id>/planning_tasks.json`.
- **`--execution-order` determines run sequence.** Lower numbers run first. Assign consecutive integers starting from 1.
- **`--group` is a free-form label.** Use it to group logically related tasks.
- **`--purpose` explains WHY.** One sentence — what this task enables or unblocks.
- **`--batch-with` links same-session tasks.** Comma-separated task IDs.
- **`--steps` and `--integration-tests` are pipe-separated** — not commas (text commonly contains commas).
- **`--mode`** starts as `plan` during planning. Changes to `execute` when an agent begins work.
- **IDs are auto-assigned** — capture the returned ID for dependency references.

### Writing tasks in execution order

Write tasks in the order they should execute. Task with `--execution-order 1` first, then 2, etc.

---

## Step 5: Tag intake artifacts with task references

After creating tasks, add `#task-N` tags back into intake documents for bidirectional traceability.

### Tag the intake markdown files

Find the paragraphs in intake markdown that relate to each task and add the tag inline at the end of the most relevant line.

### Tag action items with task references

```bash
harnessx intake-actions add-tag action-1 --tags "#task-1, #task-2"
harnessx intake-actions update action-1 \
  --note-author "hx-planning-tasks" \
  --note-text "Mapped to task-1: Project skeleton, task-2: Auth module."
```

### Tagging rules (from hx:tag-context-writing)

- Tags go at the **end of the line** they annotate — never on their own line
- Only use **traceable tags** (`#task-N`, `#milestone-N`, `#action-N`)
- Verify with `harnessx context search-context --query "#task-1"` — should return meaningful paragraphs

---

## Step 6: Write milestone handoff notes

Before stopping, write a structured handoff note onto the milestone. This is read by the NEXT task session (for the next milestone) so it knows what files, interfaces, and structures now exist.

```bash
harnessx planning-milestones update <milestone-id> \
  --note "HANDOFF: Key outputs — [list files and what they contain, referencing task IDs]. Exit-point tasks: [task IDs that downstream milestones build on]. Interfaces: [key structs, functions, APIs]. Conventions: [any patterns established]."
```

---

## Step 7: Final verification and stop

```bash
# Spot-check traceability
harnessx context search-context --query "#task-1"

# Verify tasks for this milestone
harnessx planning-milestones children <milestone-id>
```

### Completeness check

Verify:
- Every milestone success measure is addressed by at least one task
- Every task has steps, at least one integration test, a skill assignment, and a complexity rating
- Every task has a `group` label and `purpose`
- Execution order is consecutive (no gaps)
- Total tasks for this milestone is between 5 and 12
- Dependencies form a valid sequence (no task references something created by a later task)

Once verified, you are done. The coordinator (hx:planning) will mark the milestone with `mark-tasks-written` and check for more milestones.

---

## What good tasks look like

- **Action-oriented titles** — "Build and test the SafeTestOrder guard" not "Safety work"
- **Concrete steps** — an agent can follow them without guessing
- **Right-sized** — 15-60 minutes of focused work, producing a meaningful testable change
- **Correct skill assignment** — team lead for non-trivial work
- **Verifiable** — integration tests describe how to check the task worked
- **Grouped logically** — related tasks share a group label
- **Strictly ordered** — execution_order determines the sequence, no ambiguity

---

## What this skill does NOT do

- **Write milestones** — those must exist before tasks; use hx:planning-milestones
- **Execute any implementation** — tasks are plans with steps and skill assignments, not code
- **Write tasks for milestones beyond the target** — scope is one milestone per invocation
- **Change the pipeline stage** — the operator handles stage transitions
- **Create epics or stories** — the v2 model uses milestones → tasks directly

## Task file storage

Tasks are sharded by milestone:

```
planning/tasks/<milestone-id>/planning_tasks.json
```

The `--milestone` flag determines the shard location. Task IDs and order values are globally unique across all shards.
