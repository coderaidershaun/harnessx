---
name: hx:planning
description: Orchestrate the planning stage of the harnessx pipeline — coordinate milestones, epics, stories, and task decomposition across multiple sessions with proper progress tracking via the harnessx CLI. Use this skill when the pipeline reaches the planning stage, when the user says "start planning", "plan the project", "continue planning", "write milestones", "write epics", "write stories", "write tasks", or when the operator routes to the planning stage. Also trigger when the user wants to resume planning work from a previous session, or when they ask "what's next in planning". This is the single entry point for all planning work — it determines what needs to happen and loads the right specialist skill.
disable-model-invocation: false
user-invocable: false
---

# HX Planning Coordinator

You orchestrate the four phases of planning — milestones, epics, stories, tasks — across multiple sessions, each phase building on the last. The planning stage spans **many sessions** because each phase produces substantial context. Your job is to determine which phase we're in, do that phase's work, mark progress, then stop so the user can start a fresh session.

## Session model

Planning is split across sessions to manage context:

| Session | Phase | What happens |
|---------|-------|-------------|
| 1 | milestones | Create ALL project milestones |
| 2 | epics | Create ALL epics for ALL milestones |
| 3 | stories | Create ALL stories for ALL epics |
| 4+ | tasks | Create all tasks for ONE milestone per session |

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
- `{"section": "epics", "skills": ["hx:planning-epics"]}` — go to Step 3b
- `{"section": "stories", "skills": ["hx:planning-stories"]}` — go to Step 3c
- `{"section": "tasks", "skills": ["hx:planning-tasks"]}` — go to Step 3d
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
- Next session will write epics for all milestones at once
- They should start a new session and run `/hx:operator` to continue

Do not continue to the epics phase. Do not invoke another skill.

---

## Step 3b: Epics phase

**Goal**: Create epics for ALL milestones in a single session.

The epics section processes all milestones in one session. You loop through each milestone that still needs epics, create its epics, mark it written, and continue to the next.

Mark the section in progress (only on first entry — if already in_progress, skip):

```bash
harnessx planning update epics in_progress
```

Load the `hx:planning-epics` SKILL.md:

```
Read .claude/skills/hx-planning-epics/SKILL.md
```

### Loop through all milestones

```bash
harnessx planning-milestones next-to-write
```

This returns either:
- A milestone object (has `epics_written: false`) — create epics for it
- `{"message": "All milestones have their epics written."}` — go to "All epics complete" below

For each milestone returned:

1. Follow the planning-epics skill's full protocol for this specific milestone — read intake docs, analyze what capabilities are needed, create epics via CLI, use a review agent to critique, write traceability tags.

2. After all epics for this milestone are created and verified, mark it:

```bash
harnessx planning-milestones mark-written <milestone-id>
```

3. Check for next milestone:

```bash
harnessx planning-milestones next-to-write
```

If another milestone is returned, repeat from step 1. Continue until all milestones have epics.

### All epics complete

When all milestones have their epics written:

```bash
harnessx planning complete epics
```

**Stop.** Tell the user:
- What epics were created for each milestone (brief summary)
- Next session will write stories for all epics at once
- Start a new session and run `/hx:operator`
Do not continue to the stories phase. Do not invoke another skill.

---

## Step 3c: Stories phase

**Goal**: Create stories for ALL epics in a single session.

The stories section processes all epics in one session. You loop through each epic that still needs stories, create its stories, mark it written, and continue to the next.

Mark the section in progress (only on first entry — if already in_progress, skip):

```bash
harnessx planning update stories in_progress
```

Load the `hx:planning-stories` SKILL.md:

```
Read .claude/skills/hx-planning-stories/SKILL.md
```

### Loop through all epics

```bash
harnessx planning-epics next-to-write
```

This returns either:
- An epic object (has `stories_written: false`) — create stories for it
- `{"message": "All epics have their stories written."}` — go to "All stories complete" below

For each epic returned:

1. Follow the planning-stories skill's full protocol for this specific epic — get parent milestone context, read intake docs, analyze what testable behaviours are needed, develop acceptance criteria, create stories via CLI, use a review agent to critique, write traceability tags.

2. After all stories for this epic are created and verified, mark it:

```bash
harnessx planning-epics mark-written <epic-id>
```

3. Check for next epic:

```bash
harnessx planning-epics next-to-write
```

If another epic is returned, repeat from step 1. Continue until all epics have stories.

### All stories complete

When all epics have their stories written:

```bash
harnessx planning complete stories
```

**Stop.** Tell the user:
- What stories were created for each epic (brief summary)
- Next session(s) will write tasks — one milestone per session
- Start a new session and run `/hx:operator`
Do not continue to the tasks phase. Do not invoke another skill.

---

## Step 3d: Tasks phase

**Goal**: Create tasks for ALL stories under ONE milestone per session.

The tasks section processes one milestone per session because task decomposition (with dual-agent review) consumes significant context. Each session handles all stories under a single milestone.

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
- A milestone object (has `tasks_written: false`) — create tasks for its stories
- `{"message": "All milestones have their tasks written."}` — go to "All tasks complete" below

### Create tasks for this milestone

1. Get the milestone's children to see all its epics and stories:

```bash
harnessx planning-milestones children <milestone-id>
```

2. For each story under this milestone, follow the planning-tasks skill's full protocol — walk up the hierarchy (story -> epic -> milestone), launch Agent 1 (proposer) and Agent 2 (reviewer), synthesize feedback, write tasks via CLI with `--epic` and `--story` parameters, skill assignments, complexity ratings, steps, integration tests, and full traceability.

**Important:** When creating tasks, you MUST pass the `--epic` parameter:

```bash
harnessx planning-tasks create \
  --epic "#epic-1" \
  --story "#story-1" \
  --title "..." \
  ...
```

The `--epic` and `--story` together determine where the task is stored on disk (`planning/tasks/<epic-id>/<story-id>/planning_tasks.json`).

3. After all tasks for all stories in this milestone are created, mark each story:

```bash
harnessx planning-stories mark-written <story-id>
```

4. Mark the milestone's tasks as written:

```bash
harnessx planning-milestones mark-tasks-written <milestone-id>
```

5. Check if there are more milestones:

```bash
harnessx planning-milestones next-to-write-tasks
```

If another milestone is returned:

**Stop.** Tell the user:
- What tasks were created for the milestone just completed (brief summary)
- Which milestone is next
- How many milestones remain (count milestones where `tasks_written` is false)
- Start a new session and run `/hx:operator`
Do not continue to the next milestone in this session.

### All tasks complete

When all milestones have their tasks written:

```bash
harnessx planning complete tasks
```

This automatically marks the entire planning pipeline stage as complete (the CLI handles this when all 4 sections are done).

**Stop.** Tell the user:
- Planning is fully complete
- All milestones, epics, stories, and tasks have been defined
- The pipeline will advance to the review stage on next invocation
- Start a new session and run `/hx:operator`

Do not continue to the next pipeline stage. Do not invoke another skill. The operator handles stage routing.

---

## Resuming mid-phase

The coordinator is designed to handle resume gracefully. When the user returns mid-phase:

- `harnessx planning next` tells you which section is current
- `harnessx planning-milestones next-to-write` / `harnessx planning-epics next-to-write` / `harnessx planning-milestones next-to-write-tasks` tell you which specific item still needs work within that section
- Already-written items (marked via `mark-written` / `mark-tasks-written`) are skipped automatically

You never redo work that's been marked complete. If a session was interrupted mid-item, the `next-to-write` command will return that same item again since it was never marked written.

---

## CLI command reference

Section tracking:
- `harnessx planning init` — create planning.json with 4 sections
- `harnessx planning next` — get current incomplete section
- `harnessx planning complete <section>` — mark section done (auto-completes planning stage when all 4 done)
- `harnessx planning update <section> <status>` — set section status
- `harnessx planning list` — show all sections with statuses
- `harnessx planning status` — show full planning progress object

Finding what needs work:
- `harnessx planning-milestones next-to-write` — next milestone without epics
- `harnessx planning-epics next-to-write` — next epic without stories
- `harnessx planning-milestones next-to-write-tasks` — next milestone without tasks

Marking work done:
- `harnessx planning-milestones mark-written <id>` — mark milestone's epics as written
- `harnessx planning-epics mark-written <id>` — mark epic's stories as written
- `harnessx planning-stories mark-written <id>` — mark story's tasks as written
- `harnessx planning-milestones mark-tasks-written <id>` — mark milestone's tasks as written
