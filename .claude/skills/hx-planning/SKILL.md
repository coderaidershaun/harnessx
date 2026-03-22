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
| 1 | milestones | Create all project milestones |
| 2+ | epics | Create epics for ONE milestone per session |
| N+ | stories | Create stories for ONE epic per session |
| M+ | tasks | Create tasks for ONE story per session |

Each session does one unit of work, marks progress, and stops. The user returns via `/hx:operator` (which routes back here) or invokes `/hx:planning` directly.

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
- Next session will write epics for each milestone
- They should start a new session and run `/hx:operator` or `/hx:planning` to continue

Do not continue to the epics phase. Do not invoke another skill.

---

## Step 3b: Epics phase

**Goal**: Create epics for ONE milestone per session.

The epics section processes one milestone per session because epic decomposition requires deep analysis of each milestone's success measures, action items, and capability requirements. Your job is to handle the next unwritten milestone, mark progress, then stop.

Mark the section in progress (only on first entry — if already in_progress, skip):

```bash
harnessx planning update epics in_progress
```

Load the `hx:planning-epics` SKILL.md:

```
Read .claude/skills/hx-planning-epics/SKILL.md
```

Find the next milestone that needs epics:

```bash
harnessx planning-milestones next-to-write
```

This returns either:
- A milestone object (has `epics_written: false`) — create epics for it
- `{"message": "All milestones have their epics written."}` — go to "All epics complete" below

### Create epics for this milestone

1. Follow the planning-epics skill's full protocol for this specific milestone — read intake docs, analyze what capabilities are needed, create epics via CLI, use a review agent to critique, write traceability tags.

2. After all epics for this milestone are created and verified, mark it:

```bash
harnessx planning-milestones mark-written <milestone-id>
```

3. Check if there are more milestones:

```bash
harnessx planning-milestones next-to-write
```

If another milestone is returned:

**Stop.** Tell the user:
- What epics were created for the milestone just completed (brief summary)
- Which milestone is next
- How many milestones remain (count milestones where `epics_written` is false)
- Start a new session and run `/hx:operator` or `/hx:planning`

Do not continue to the next milestone in this session.

### All epics complete

When all milestones have their epics written:

```bash
harnessx planning complete epics
```

**Stop.** Tell the user:
- All milestones now have epics defined
- Next session will write stories — one epic per session
- Start a new session and run `/hx:operator` or `/hx:planning`

Do not continue to the stories phase. Do not invoke another skill.

---

## Step 3c: Stories phase

**Goal**: Create stories for ONE epic per session.

The stories section processes one epic per session because story decomposition — including acceptance criteria development — requires focused attention on each epic's capability boundaries.

Mark the section in progress (only on first entry — if already in_progress, skip):

```bash
harnessx planning update stories in_progress
```

Load the `hx:planning-stories` SKILL.md:

```
Read .claude/skills/hx-planning-stories/SKILL.md
```

Find the next epic that needs stories:

```bash
harnessx planning-epics next-to-write
```

This returns either:
- An epic object (has `stories_written: false`) — create stories for it
- `{"message": "All epics have their stories written."}` — go to "All stories complete" below

### Create stories for this epic

1. Follow the planning-stories skill's full protocol for this specific epic — get parent milestone context, read intake docs, analyze what testable behaviours are needed, develop acceptance criteria, create stories via CLI, use a review agent to critique, write traceability tags.

2. After all stories for this epic are created and verified, mark it:

```bash
harnessx planning-epics mark-written <epic-id>
```

3. Check if there are more epics:

```bash
harnessx planning-epics next-to-write
```

If another epic is returned:

**Stop.** Tell the user:
- What stories were created for the epic just completed (brief summary)
- Which epic is next
- How many epics remain (count epics where `stories_written` is false)
- Start a new session and run `/hx:operator` or `/hx:planning`

Do not continue to the next epic in this session.

### All stories complete

When all epics have their stories written:

```bash
harnessx planning complete stories
```

**Stop.** Tell the user:
- All epics now have stories defined
- Next session(s) will write tasks — one story per session
- Start a new session and run `/hx:operator` or `/hx:planning`

Do not continue to the tasks phase. Do not invoke another skill.

---

## Step 3d: Tasks phase

**Goal**: Create tasks for ONE story per session.

The tasks section is different from the others — it processes one story per session because task decomposition is detailed work that consumes significant context.

Mark the section in progress (only on first entry — if already in_progress, skip):

```bash
harnessx planning update tasks in_progress
```

Load the `hx:planning-tasks` SKILL.md:

```
Read .claude/skills/hx-planning-tasks/SKILL.md
```

Find the next story that needs tasks:

```bash
harnessx planning-stories next-to-write
```

This returns either:
- A story object (has `tasks_written: false`) — create tasks for it
- `{"message": "All stories have their tasks written."}` — go to "All tasks complete" below

### Create tasks for this story

1. Follow the planning-tasks skill's full protocol for this specific story — walk up the hierarchy (story -> epic -> milestone), read intake docs, catalog available specialist skills, launch Agent 1 (proposer) and Agent 2 (reviewer), synthesize feedback, write tasks via CLI with skill assignments, complexity ratings, steps, integration tests, and full traceability.

2. After all tasks for this story are created and verified, mark it:

```bash
harnessx planning-stories mark-written <story-id>
```

3. Check if there are more stories:

```bash
harnessx planning-stories next-to-write
```

If another story is returned:

**Stop.** Tell the user:
- What tasks were created for the story just completed (brief summary)
- Which story is next
- How many stories remain (count stories where `tasks_written` is false)
- Start a new session and run `/hx:operator` or `/hx:planning`

Do not continue to the next story in this session.

### All tasks complete

When all stories have their tasks written:

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
- `harnessx planning-milestones next-to-write` / `harnessx planning-epics next-to-write` / `harnessx planning-stories next-to-write` tell you which specific item still needs work within that section
- Already-written items (marked via `mark-written`) are skipped automatically

You never redo work that's been marked complete. If a session was interrupted mid-item (e.g., crashed while writing epics for milestone-2), the `next-to-write` command will return that same item again since it was never marked written.

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
- `harnessx planning-stories next-to-write` — next story without tasks

Marking work done:
- `harnessx planning-milestones mark-written <id>` — mark milestone's epics as written
- `harnessx planning-epics mark-written <id>` — mark epic's stories as written
- `harnessx planning-stories mark-written <id>` — mark story's tasks as written
