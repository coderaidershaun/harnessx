---
name: hx:execution-next-task
description: Pick up the next ready task, gather full project context via parallel lightweight agents, synthesize a precision-targeted execution brief, then dispatch the task to the right specialist agent with the right model and thinking depth. This is the execution engine — the skill that turns planning artifacts into working code by orchestrating context gathering and agent dispatch with surgical precision. Use this skill when the pipeline reaches the execution stage, when the user says "execute next task", "run next task", "do the next task", "start working", "pick up next task", "continue execution", or anything about executing planned work. Also trigger when the operator routes to execution, after all planning is complete and implementation should begin, or when the user just says "next" during the execution phase. This skill should be used repeatedly — each invocation picks up one task, executes it, records progress, and stops.
disable-model-invocation: false
user-invocable: false
---

# Execution: Next Task

You are the execution orchestrator. Your job is to pick up the next ready task from the planning hierarchy, gather exactly the right context from multiple sources in parallel, synthesize that context into a precision-targeted brief, and dispatch the task to the specialist agent who will do the work.

This is the most context-sensitive skill in the system. Every decision you make about what context to include — and what to leave out — directly affects whether the executing agent succeeds or drifts. Too much context and the agent drowns. Too little and it guesses. Your job is to find the sweet spot: enough for the agent to understand WHY it's doing what it's doing, and exactly what "done" looks like.

**One task per invocation.** Pick up one task. Execute it. Record the result. Stop. The user or operator invokes you again for the next task.

---

## Why this skill exists

Specialist agents (like `rust:developing` or `rust:team-coordinator`) are excellent at their craft but terrible at knowing why they're doing what they're doing. They don't know the user's goal, the milestone checkpoint, or what the agent before them just shipped. Without that context, they make locally reasonable decisions that drift from the project's intent.

Meanwhile, dumping the entire project context into an agent's prompt is wasteful and counterproductive — it dilutes the signal with noise, fills context windows with irrelevant intake paragraphs, and makes the agent spend tokens parsing information it doesn't need.

This skill solves both problems. It gathers intelligence from multiple sources cheaply (parallel lightweight agents), distills it into a lean brief (the signal without the noise), and dispatches the executing agent with exactly what it needs to do excellent work.

---

## Phase 1: Identify the next task

```bash
harnessx project active
```

Capture the project ID from `data.id`. You'll need this for file paths throughout.

```bash
harnessx planning-tasks next
```

Parse the JSON response. There are three possible shapes:

**Ready task found** — the response is the full task object. Capture everything: `id`, `title`, `steps`, `story`, `depends_on`, `complexity`, `mode`, `skills`, `integration_tests`, `traces`, `notes`. Proceed to Phase 2.

**All blocked** — the response has `"message": "All remaining tasks are blocked by unmet dependencies."` plus a `blocked_tasks` array. Report this to the user with the specific blockers for each task. Do not attempt to unblock — ask the user what to do. Stop.

**All completed** — the response has `"message": "All tasks completed."`. Check whether the parent story needs its `tasks_completed` flag set:

```bash
harnessx planning-stories list
```

Find any stories where `tasks_completed` is false but all their tasks are done. For each:

```bash
harnessx planning-stories mark-completed <story-id>
```

Then cascade upward — check epics, then milestones, marking completion flags as appropriate.

After all completion flags are cascaded, mark the execution stage complete:

```bash
harnessx progress complete execution
```

Report to the user that all tasks are complete and the pipeline will advance past execution on next invocation. Stop.

---

## Phase 2: Gather intelligence (parallel agents)

Launch three agents in parallel. Each has a focused job and returns a compact result. The key insight: these agents are cheap scouts, not deep thinkers. They gather and summarize — the synthesis happens in Phase 3 (your job).

**IMPORTANT: Do NOT set `run_in_background: true`.** All agents must run in foreground — their results are needed before the next phase can proceed.

### Agent A: Project context via tag tracing

This is the heavyweight. Launch an agent that follows the `hx:tag-context-reading` skill's process for the task you just identified.

```
You are gathering project context for task [TASK-ID]: "[TASK-TITLE]"

Follow the hx:tag-context-reading skill process:

1. Walk up the hierarchy:
   harnessx planning-tasks parent [TASK-ID]
   harnessx planning-stories parent [STORY-ID]
   harnessx planning-epics parent [EPIC-ID]

2. Collect traced action items:
   harnessx intake-actions list
   (extract only the action items referenced in traces.tags across all four levels)

3. Pull relevant intake context:
   harnessx context search-context --query "#action-N"
   (for each traced action item)

4. Check sibling and next tasks:
   harnessx planning-tasks list
   (find what comes before and after this task in the same story)

Return the complete Context Brief and Task Brief as described in the hx:tag-context-reading skill.
Keep it to 40-80 lines. Prioritize the WHY chain over raw detail.
```

**Model:** opus (this agent needs to synthesize well)

### Agent B: Recent progress from history

```
Read the file harnessx/[PROJECT-ID]/history.md and summarize the most recent progress.

Focus on:
- What was the last task completed?
- Were there any issues, blockers, or decisions noted?
- What's the current momentum (smooth progress or rough patch)?

If the file is empty or doesn't exist, say "No history recorded yet — this appears to be the first task execution."

Keep your summary to 5-10 lines maximum.
```

**Model:** sonnet (lightweight read-and-summarize)

### Agent C: Recent git activity

```
Run these commands and summarize what happened:

git log --oneline -5
git diff HEAD~2..HEAD --stat

Focus on:
- What files were changed in the last 2 commits?
- What was the nature of the changes (new feature, refactor, fix, planning)?
- Is there anything the next executing agent should know about the codebase state?

Keep your summary to 5-10 lines maximum.
```

**Model:** sonnet (lightweight shell-and-summarize)

### Wait for all three agents

Collect all three results before proceeding. If Agent A fails (e.g., parent not found), you still have enough context from the task object itself to proceed — but flag the missing context in the execution brief.

---

## Phase 3: Synthesize the execution brief

This is where you earn your keep. You have:
- The full task object (from Phase 1)
- The project context narrative (from Agent A)
- The recent progress summary (from Agent B)
- The git activity summary (from Agent C)

Synthesize these into a **single, focused execution brief**. This brief is the ONLY context the executing agent will receive. It must be complete enough to work from but lean enough to not waste tokens.

### The execution brief structure

```
EXECUTION BRIEF
===============

TASK: [task-id] — "[task title]"
MODE: [plan|execute|review|rework]
COMPLEXITY: [complexity]
SKILLS: [assigned skills]

PROJECT DIRECTORY: [directory from project metadata]

SITUATION
---------
[2-4 sentences from Agent A's Context Brief — the "we are building X for Y so that Z" framing.
Include the milestone → epic → story chain as a single flowing sentence, not a bullet list.
This gives the agent the WHY without the noise.]

RECENT PROGRESS
---------------
[2-3 sentences from Agent B and C — what just happened, what state the codebase is in.
If this task depends on recently completed tasks, call that out specifically.
If this is the first task, say so.]

YOUR WORK
---------
Steps:
1. [step 1 — copied verbatim from task]
2. [step 2]
...

Key context from action items:
- [1-2 sentences from the most relevant traced action item details — the stuff
  that won't be obvious from the steps alone]

Scope boundaries:
- [Any constraints from scope.md that affect this task — what NOT to do]
- [If none are relevant, omit this section entirely]

VERIFICATION
------------
Integration tests to pass:
- [test 1]
- [test 2]

Story acceptance criteria this task contributes to:
- [criterion 1 — from the parent story]
- [criterion 2]

Output files expected:
- [output_sources entries, if any]

WHAT COMES NEXT
---------------
[1-2 sentences about the task that follows this one in the story.
"Your work should leave the codebase in a state where [next task title] can begin cleanly."
OR if this is the last task: "This is the final task in the story. When complete,
all acceptance criteria should pass."]
```

### Brief guidelines

- **40-60 lines total.** If you're over 60 lines, you're including too much. Cut the least actionable content.
- **No intake document dumps.** The agent doesn't need to read the user's raw words from goal.md. It needs the synthesized WHY.
- **Steps are sacred.** Copy them verbatim from the task. These were carefully written during planning — don't paraphrase.
- **The SITUATION section is the most important.** An agent that understands why it's doing something will make better judgment calls than one that just follows steps mechanically.
- **Include the project directory.** The agent needs to know where the code lives.
- **Omit sections that add nothing.** If there are no scope boundaries relevant to this task, don't include an empty section. If there are no notes from risk reviews, don't mention it.

---

## Phase 4: Determine dispatch parameters

The task's `complexity` and `mode` fields determine how the executing agent is configured. The principle: **always assume the agent needs one level of thinking depth higher than what seems sufficient.** Quality beats speed. A task done well once is cheaper than a task done poorly twice.

### Model selection

| Complexity | Default model | Bumped model (what you use) |
|------------|---------------|----------------------------|
| `super-low` | haiku | **sonnet** |
| `low` | sonnet | **sonnet** |
| `medium` | sonnet | **opus** |
| `high` | opus | **opus** |
| `super-high` | opus | **opus** |
| `uncertain` | — | **opus** (always conservative) |

### Thinking depth (embedded in the prompt preamble)

| Complexity | Thinking instruction |
|------------|---------------------|
| `super-low` | "Think through each step before acting." |
| `low` | "Think carefully about edge cases and how your changes integrate with existing code." |
| `medium` | "Think deeply about this task. Consider architectural implications, edge cases, and how your work fits into the larger system." |
| `high` | "This is a high-complexity task. Take your time. Analyze the problem thoroughly before writing any code. Consider every edge case, every integration point, every way this could go wrong. Quality is paramount." |
| `super-high` | "This is the most complex type of task in the system. Use maximum analytical depth. Consider the problem from multiple angles before committing to an approach. Verify your reasoning at each step. Leave nothing to chance." |
| `uncertain` | Same as `high` — treat uncertainty as high complexity until proven otherwise. |

### Mode-specific framing

Prepend to the execution brief based on `mode`:

- **`plan`**: "You are PLANNING, not implementing. Your output should be a design document, architecture decision, or implementation plan — not code. Think, analyze, decide, document."
- **`execute`**: "You are IMPLEMENTING. Write the code. Follow the steps. Make it work. Verify with the integration tests."
- **`review`**: "You are REVIEWING existing work. Read the code critically. Check it against the acceptance criteria and integration tests. Flag issues. Do not rewrite unless explicitly broken."
- **`rework`**: "You are FIXING issues identified during review. Focus on the specific problems. Do not refactor beyond what's needed to resolve the issues."

### Skill dispatch

The task's `skills` field lists the specialist skill(s) to use. The executing agent should be told to follow these skills.

When skills contain a team coordinator (e.g., `rust:team-coordinator`), dispatch to the coordinator and let it triage internally. When skills are direct specialists (e.g., `rust:commenting`), the agent works as that specialist directly.

---

## Phase 5: Dispatch the executing agent

### Update task status and mode

Before dispatching, mark the task in progress. If the task's current `mode` is `plan`, flip it to `execute` so the agent receives implementation framing:

```bash
harnessx planning-tasks update [TASK-ID] --status in_progress --mode execute
```

If the task's mode is already `rework` or `review`, preserve it — only update status:

```bash
harnessx planning-tasks update [TASK-ID] --status in_progress
```

### Launch the agent

Launch one agent with the synthesized brief. This is the agent that does the actual work.

```
[THINKING DEPTH INSTRUCTION from Phase 4]

[MODE FRAMING from Phase 4]

You have access to the following specialist skills: [SKILLS LIST]
Load and follow the instructions in these skills for your work.

---

[EXECUTION BRIEF from Phase 3]

---

When you are done:
1. Verify your work against the integration tests listed above
2. Summarize what you did in 3-5 bullet points
3. Note any issues, concerns, or decisions you made that the next task should know about
4. If you could not complete the task, explain exactly what blocked you and what's needed
```

**Model:** determined by Phase 4's complexity mapping.

**Mode:** Use `mode: "bypassPermissions"` to let the agent work autonomously. The task was planned and approved during planning — the agent should execute without asking for permission at every file write.

Wait for the agent to complete and capture its result.

---

## Phase 6: Post-execution bookkeeping

After the executing agent returns, update the project state. This is critical — without it, the next invocation of this skill won't know what happened.

### 6a: Update task status

If the agent succeeded:

```bash
harnessx planning-tasks update [TASK-ID] --status completed --note "[AGENT-SUMMARY — first 1-2 bullet points from the agent's summary]"
```

If the agent failed or was blocked:

```bash
harnessx planning-tasks update [TASK-ID] --status rework --mode rework --note "[WHAT-WENT-WRONG]"
```

### 6b: Append to history.md

Write a concise entry to `harnessx/[PROJECT-ID]/history.md`:

```markdown
## [TASK-ID]: [task title]
**Date:** [today's date]
**Status:** completed | rework
**Skills used:** [skills]
**Summary:** [2-3 sentences — what was done, key decisions, any concerns]
**Files changed:** [list from git diff or agent report]
```

Read the existing file first and append — don't overwrite.

### 6c: Check story completion

```bash
harnessx planning-stories children [STORY-ID]
```

If ALL tasks under this story now have status `completed`:

```bash
harnessx planning-stories mark-completed [STORY-ID]
harnessx planning-stories update [STORY-ID] --status completed --note "All tasks completed via hx:execution-next-task."
```

Then check epic completion:

```bash
harnessx planning-epics children [EPIC-ID]
```

If all stories under this epic are completed:

```bash
harnessx planning-epics mark-completed [EPIC-ID]
harnessx planning-epics update [EPIC-ID] --status completed
```

Continue upward to milestones if applicable. This cascading completion keeps the hierarchy in sync.

### 6d: Report to user

Tell the user what happened:

```
Task [TASK-ID] "[task title]" — [completed | needs rework]

[2-3 bullet summary from the executing agent]

Next: [next task title, or "story complete" if this was the last task]
```

Then stop. The user or operator will invoke this skill again for the next task.

---

## Edge cases

### Task has multiple skills

If `skills` contains more than one skill (e.g., `"rust:developing, rust:unit-testing"`), the executing agent should be told about all of them. If one of them is a team coordinator, dispatch to the coordinator — it will handle the multi-skill orchestration internally.

If there's no coordinator and multiple direct specialists are listed, dispatch a single agent that follows each skill in sequence (implement first, then test, etc.).

### Task has no skills assigned

This shouldn't happen (planning should assign skills), but if it does: look at the task's steps and title to infer the right skill family. Default to the team coordinator for that domain if one exists.

### The executing agent fails

Don't retry automatically. Mark the task as `rework`, record what went wrong in history.md, and report to the user. The next invocation will pick up the same task (since it's not completed) and the rework mode framing will tell the agent to fix what's broken.

### History.md doesn't exist

Create it with a header:

```markdown
# Project History

Execution log for this project. Each entry records a task execution with its outcome.
```

Then append the first entry.

### No next task but story isn't marked complete

This can happen if tasks were manually completed outside this skill. Run the completion cascade (Phase 6c) to sync the hierarchy.

---

## What this skill does NOT do

- **Write planning artifacts** — milestones, epics, stories, and tasks must exist before execution
- **Make architectural decisions** — that's the executing agent's job (via its assigned skill)
- **Interact with the user during execution** — the executing agent works autonomously; if it needs user input, it fails and this skill records the failure
- **Execute multiple tasks** — one task per invocation, always
- **Skip the context gathering** — even for `super-low` complexity tasks, the brief matters; a two-minute context investment prevents a twenty-minute drift
