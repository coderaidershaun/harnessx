---
name: hx:execution-next-task
description: Pick up the next ready task, gather full project context via parallel lightweight agents, synthesize a precision-targeted execution brief, then dispatch the task to the right specialist agent with the right model and thinking depth. This is the execution engine — the skill that turns planning artifacts into working code by orchestrating context gathering and agent dispatch with surgical precision. Use this skill when the pipeline reaches the execution stage, when the user says "execute next task", "run next task", "do the next task", "start working", "pick up next task", "continue execution", or anything about executing planned work. Also trigger when the operator routes to execution, after all planning is complete and implementation should begin, or when the user just says "next" during the execution phase. This skill should be used repeatedly — each invocation picks up one task, executes it, records progress, and stops.
disable-model-invocation: false
user-invocable: false
---

# Execution: Next Task

You are the execution orchestrator. Your job is to pick up the next ready task from the planning hierarchy, gather exactly the right context from multiple sources in parallel, synthesize that context into a precision-targeted brief, and dispatch the task to the specialist agent who will do the work.

This is the most context-sensitive skill in the system. Every decision you make about what context to include — and what to leave out — directly affects whether the executing agent succeeds or drifts. Too much context and the agent drowns. Too little and it guesses. Your job is to find the sweet spot.

**One task per invocation.** Pick up one task. Execute it. Record the result. Stop. The user or operator invokes you again for the next task.

---

## Phase 1: Identify the next task

```bash
harnessx project active
```

Capture the project ID from `data.id`. You'll need this for file paths throughout.

```bash
harnessx planning-tasks next
```

Parse the JSON response. There are several possible shapes:

**Ready task found** — the response is the full task object. Capture everything: `id`, `title`, `steps`, `milestone`, `group`, `purpose`, `execution_order`, `depends_on`, `complexity`, `mode`, `skills`, `integration_tests`, `traces`, `notes`. Proceed to Phase 2.

**All blocked** — the response has `"message": "All remaining tasks are blocked..."`. Report this to the user with the specific blockers. Do not attempt to unblock — ask the user what to do. Stop.

**Milestone tasks done** — the response has `"message": "All tasks in current milestone completed. Milestone ready for review."` plus a `milestone` field. This means all tasks in the current milestone are done but the milestone isn't marked complete yet. Trigger the built-in review:

```bash
harnessx planning-milestones review <milestone-id> --status pending
```

Then mark the milestone as in_progress for review and dispatch the review task (see Phase 7 below). Stop.

**All completed** — the response has `"message": "All tasks completed."`. Mark the execution stage complete:

```bash
harnessx progress complete execution
```

Report to the user that all tasks are complete and the pipeline will advance past execution on next invocation. Stop.

---

## Phase 2: Gather intelligence (parallel agents)

Launch three agents in parallel. Each has a focused job and returns a compact result. These are cheap scouts, not deep thinkers — the synthesis happens in Phase 3.

**IMPORTANT: Do NOT set `run_in_background: true`.** All agents must run in foreground.

### Agent A: Project context via tag tracing

This is the heavyweight. Launch an agent that follows the `hx:tag-context-reading` skill's process for the task you just identified.

```
You are gathering project context for task [TASK-ID]: "[TASK-TITLE]"

Follow the hx:tag-context-reading skill process:

1. Walk up the hierarchy:
   harnessx planning-tasks parent [TASK-ID]
   (This returns the parent milestone directly for v2 tasks)

2. Collect traced action items:
   harnessx intake-actions list
   (extract only the action items referenced in traces.tags)

3. Pull relevant intake context:
   harnessx context search-context --query "#action-N"
   (for each traced action item)

4. Check sibling tasks in the same milestone:
   harnessx planning-milestones children [MILESTONE-ID]
   (find what comes before and after this task by execution_order)

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

You have:
- The full task object (from Phase 1)
- The project context narrative (from Agent A)
- The recent progress summary (from Agent B)
- The git activity summary (from Agent C)

Synthesize these into a **single, focused execution brief**. This brief is the ONLY context the executing agent will receive.

### The execution brief structure

```
EXECUTION BRIEF
===============

TASK: [task-id] — "[task title]"
MODE: [plan|execute|review|rework]
COMPLEXITY: [complexity]
SKILLS: [assigned skills]
GROUP: [group label]

PROJECT DIRECTORY: [directory from project metadata]

SITUATION
---------
[2-4 sentences from Agent A's Context Brief — the "we are building X for Y so that Z" framing.
Include the milestone purpose → task purpose chain as a single flowing sentence.
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
- [1-2 sentences from the most relevant traced action item details]

Scope boundaries:
- [Any constraints from scope.md that affect this task]

VERIFICATION
------------
Integration tests to pass:
- [test 1]
- [test 2]

Output files expected:
- [output_sources entries, if any]

WHAT COMES NEXT
---------------
[1-2 sentences about the task that follows this one (next execution_order in the milestone).
"Your work should leave the codebase in a state where [next task title] can begin cleanly."
OR if this is the last task: "This is the final task in the milestone. When complete,
all milestone success measures should be satisfied."]
```

### Brief guidelines

- **40-60 lines total.** If you're over 60 lines, cut the least actionable content.
- **No intake document dumps.** The agent needs the synthesized WHY.
- **Steps are sacred.** Copy them verbatim from the task.
- **The SITUATION section is the most important.**
- **Include the project directory.**
- **Omit sections that add nothing.**

---

## Phase 4: Determine dispatch parameters

### Model selection

| Complexity | Model Used |
|------------|-----------|
| `super-low`, `low` | sonnet |
| `medium`, `high`, `super-high`, `uncertain` | opus |

### Thinking depth

| Complexity | Thinking instruction |
|------------|---------------------|
| `super-low` | "Think through each step before acting." |
| `low` | "Think carefully about edge cases and how your changes integrate with existing code." |
| `medium` | "Think deeply about this task. Consider architectural implications, edge cases, and how your work fits into the larger system." |
| `high` | "This is a high-complexity task. Take your time. Analyze the problem thoroughly before writing any code. Quality is paramount." |
| `super-high` | "This is the most complex type of task. Use maximum analytical depth. Verify your reasoning at each step." |
| `uncertain` | Same as `high`. |

### Mode-specific framing

- **`plan`**: "You are PLANNING, not implementing. Output a design document or implementation plan — not code."
- **`execute`**: "You are IMPLEMENTING. Write the code. Follow the steps. Make it work. Verify with the integration tests."
- **`review`**: "You are REVIEWING existing work. Read critically. Check against criteria. Flag issues."
- **`rework`**: "You are FIXING issues identified during review. Focus on the specific problems."

---

## Phase 5: Dispatch the executing agent

### Update task status and mode

Before dispatching, mark the task in progress. If the task's current `mode` is `plan`, flip it to `execute`:

```bash
harnessx planning-tasks update [TASK-ID] --status in_progress --mode execute
```

If the task's mode is already `rework` or `review`, preserve it — only update status:

```bash
harnessx planning-tasks update [TASK-ID] --status in_progress
```

### Launch the agent

Launch one agent with the synthesized brief.

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
**Mode:** Use `mode: "bypassPermissions"` to let the agent work autonomously.

Wait for the agent to complete and capture its result.

---

## Phase 6: Post-execution bookkeeping

### 6a: Update task status

If the agent succeeded:

```bash
harnessx planning-tasks update [TASK-ID] --status completed --note "[AGENT-SUMMARY]"
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
**Summary:** [2-3 sentences]
**Files changed:** [list from git diff or agent report]
```

Read the existing file first and append — don't overwrite.

### 6c: Check milestone completion

After a task completes, check if all tasks in the milestone are now done:

```bash
harnessx planning-milestones children [MILESTONE-ID]
```

If ALL tasks under this milestone now have status `completed`:

```bash
harnessx planning-milestones update [MILESTONE-ID] --status completed
```

This triggers the built-in milestone review on the next invocation (Phase 1 will see "Milestone ready for review").

### 6d: Report to user

Tell the user what happened:

```
Task [TASK-ID] "[task title]" — [completed | needs rework]

[2-3 bullet summary from the executing agent]

Next: [next task title by execution_order, or "milestone complete — review pending"]
```

Then stop.

---

## Phase 7: Milestone review (built-in)

When Phase 1 detects "All tasks in current milestone completed", the milestone needs review before the next milestone's tasks can begin.

The review process is handled by the `hx:milestone-rework-assessment` skill. The execution engine's role is:

1. Set `review_status` to `pending`
2. Find or create the review task for this milestone
3. Dispatch it via the normal Phase 2-6 flow

If the milestone already has a review task (from planning), execute it. If not, the rework assessment skill runs its autonomous review process.

After review:
- If clean: `harnessx planning-milestones review [ID] --status passed` and mark milestone completed
- If issues found: rework tasks are appended to the milestone. The next `planning-tasks next` call will return the first rework task.

---

## Edge cases

### Task has multiple skills

If one is a team coordinator, dispatch to the coordinator. If multiple direct specialists, dispatch a single agent that follows each skill in sequence.

### Task has no skills assigned

Look at the task's steps and title to infer the right skill family. Default to the team coordinator.

### The executing agent fails

Don't retry automatically. Mark as `rework`, record what went wrong, report to the user. The next invocation picks up the same task with rework framing.

### History.md doesn't exist

Create it with a header, then append the first entry.

### Batch tasks

If the task has `batch_with` entries, gather context for all tasks in the batch and include all their steps in the execution brief. The agent executes all batched tasks in one session. Mark all as completed/rework after.

---

## What this skill does NOT do

- **Write planning artifacts** — milestones and tasks must exist before execution
- **Make architectural decisions** — that's the executing agent's job
- **Interact with the user during execution** — the executing agent works autonomously
- **Execute multiple tasks** — one task (or one batch) per invocation
- **Skip the context gathering** — even for `super-low` tasks, the brief matters
