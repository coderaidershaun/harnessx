---
name: hx:planning
description: Orchestrate the planning stage of the harnessx pipeline — coordinate milestones and task decomposition across multiple sessions with proper progress tracking via the harnessx CLI. Use this skill when the pipeline reaches the planning stage, when the user says "start planning", "plan the project", "continue planning", "write milestones", "write tasks", or when the operator routes to the planning stage. Also trigger when the user wants to resume planning work from a previous session, or when they ask "what's next in planning". This is the single entry point for all planning work — it determines what needs to happen and loads the right specialist skill.
disable-model-invocation: false
user-invocable: false
---

# HX Planning Coordinator

You orchestrate the two phases of planning — milestones and tasks — across multiple sessions. The planning stage spans **many sessions** because task decomposition for each milestone consumes significant context. Your job is to determine which phase we're in, do that phase's work, mark progress, then stop so the user can start a fresh session.

## Session model

Planning uses a 2-level hierarchy: **milestones → tasks**. No epics or stories.

| Session | Phase | What happens |
|---------|-------|-------------|
| 1 | milestones | Create ALL project milestones (3-7 demonstrable checkpoints) |
| 2+ | tasks | Create all tasks for ONE milestone per session (5-12 tasks each) |

Each session does one unit of work, marks progress, and stops. The user returns via `/hx:operator` (which routes back here).

---

## Step 1: Startup

Confirm the active project and mark the planning stage as in progress.

```bash
harnessx project active
```

If no active project, tell the user and stop.

```bash
harnessx progress update planning in_progress
```

Initialize the planning section tracker (safe to call — errors if already exists, which is fine on resume):

```bash
harnessx planning init
```

If init returns an error about already existing, that's expected on resume sessions. Continue.

## Step 2: Get current section

```bash
harnessx planning next
```

This returns one of:
- `{"section": "milestones", "skills": ["hx:planning-milestones"]}` — go to Step 3a
- `{"section": "tasks", "skills": ["hx:planning-tasks"]}` — go to Step 3b
- `{"message": "Planning fully completed..."}` — tell the user planning is done, stop

Tell the user what phase you're entering and what it involves. Keep it brief — one or two sentences.

---

## Step 3a: Milestones phase

**Goal**: Create all project milestones in a single session.

Mark the section in progress:

```bash
harnessx planning update milestones in_progress
```

Load the `hx:planning-milestones` SKILL.md:

```
Read .claude/skills/hx-planning-milestones/SKILL.md
```

Follow its full protocol: read all intake documents, analyze what "done" looks like at each checkpoint, plan milestones, use a review agent to critique, write milestones via the CLI with full traceability.

After all milestones are created and verified, mark the section complete:

```bash
harnessx planning complete milestones
```

**Stop.** Tell the user:
- What milestones were created (brief summary)
- Next session will write tasks for the first milestone
- They should start a new session and run `/hx:operator` to continue

Do not continue to the tasks phase. Do not invoke another skill.

---

## Step 3b: Tasks phase

**Goal**: Create tasks for ONE milestone per session (5-12 tasks directly under the milestone).

Mark the section in progress (only on first entry — if already in_progress, skip):

```bash
harnessx planning update tasks in_progress
```

Load the `hx:planning-tasks` SKILL.md:

```
Read .claude/skills/hx-planning-tasks/SKILL.md
```

Find the next milestone that needs tasks:

```bash
harnessx planning-milestones next-to-write-tasks
```

This returns either:
- A milestone object (has `tasks_written: false`) — create tasks for it
- `{"message": "All milestones have their tasks written."}` — go to "All tasks complete" below

### Create tasks for this milestone

1. Get the milestone's current children to see any existing tasks:

```bash
harnessx planning-milestones children <milestone-id>
```

2. Follow the planning-tasks skill's full protocol — read intake context, load prior milestone handoff notes, decompose into 5-12 tasks with `--milestone` parameter, group labels, purpose fields, execution ordering, skill assignments, complexity ratings, steps, integration tests, and full traceability.

**Important:** When creating tasks, you MUST pass the `--milestone` parameter:

```bash
harnessx planning-tasks create \
  --milestone "#milestone-1" \
  --group "setup" \
  --purpose "Bootstrap the project so subsequent tasks can compile" \
  --execution-order 1 \
  --title "..." \
  ...
```

The `--milestone` determines where the task is stored on disk (`planning/tasks/<milestone-id>/planning_tasks.json`).

3. After all tasks for this milestone are created, mark it:

```bash
harnessx planning-milestones mark-tasks-written <milestone-id>
```

4. Check if there are more milestones:

```bash
harnessx planning-milestones next-to-write-tasks
```

If another milestone is returned:

**Stop.** Tell the user:
- What tasks were created for the milestone just completed (brief summary)
- Which milestone is next
- How many milestones remain
- Start a new session and run `/hx:operator`
Do not continue to the next milestone in this session.

### All tasks complete

When all milestones have their tasks written:

```bash
harnessx planning complete tasks
```

This automatically marks the entire planning pipeline stage as complete.

**Stop.** Tell the user:
- Planning is fully complete
- All milestones and tasks have been defined
- The pipeline will advance to the review stage on next invocation
- Start a new session and run `/hx:operator`

Do not continue to the next pipeline stage. Do not invoke another skill. The operator handles stage routing.

---

## Resuming mid-phase

The coordinator is designed to handle resume gracefully. When the user returns mid-phase:

- `harnessx planning next` tells you which section is current
- `harnessx planning-milestones next-to-write-tasks` tells you which milestone still needs tasks
- Already-written milestones (marked via `mark-tasks-written`) are skipped automatically

You never redo work that's been marked complete. If a session was interrupted mid-milestone, the `next-to-write-tasks` command will return that same milestone again since it was never marked written.

---

## CLI command reference

Section tracking:
- `harnessx planning init` — create planning.json
- `harnessx planning next` — get current incomplete section
- `harnessx planning complete <section>` — mark section done (auto-completes planning stage when milestones + tasks are done)
- `harnessx planning update <section> <status>` — set section status
- `harnessx planning list` — show all sections with statuses

Finding what needs work:
- `harnessx planning-milestones next-to-write-tasks` — next milestone without tasks

Marking work done:
- `harnessx planning-milestones mark-tasks-written <id>` — mark milestone's tasks as written
