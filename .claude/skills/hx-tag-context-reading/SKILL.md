---
name: hx:tag-context-reading
description: Pull full project context for a planning item (task, story, epic, or milestone) by tracing its tags up through the entire harnessx hierarchy — from task through story, epic, milestone, action items, and intake documents — then synthesize everything into a structured narrative summary that tells the requesting agent exactly what they're building, why, and what comes next. Use this skill whenever an agent needs project context before starting work on a task, when you need to understand the "why" behind a planning item, when loading context for execution, or when any skill needs the full traceability chain. Also trigger when agents say "get context", "load context", "pull context", "what's the background", "why are we doing this", "brief me on this task", or when beginning execution of any planning task. This is the reading counterpart to hx:tag-context-writing — writing puts tags in, this skill pulls meaning out.
disable-model-invocation: false
user-invocable: false
---

# Tag Context Reading

You trace a planning item's tags through the full harnessx hierarchy and synthesize everything into a narrative summary that gives the requesting agent complete situational awareness — what they're building, why, for whom, and what comes before and after their work.

This is the inverse of `hx:tag-context-writing`. That skill writes tags into documents for searchability. This skill reads those tags back out, follows every thread, and assembles the full picture.

---

## Why this skill exists

Agents executing tasks operate with limited context. A task says *what* to do but not *why*. Without the chain of reasoning that produced the task — the user's goal, the scope boundaries, the milestone checkpoint, the epic capability, the story's acceptance criteria — agents make locally reasonable decisions that drift from the project's intent.

This skill prevents that drift by giving every agent the same grounding: here's the full story behind your work.

---

## When to use this skill

- **Before task execution** — any agent about to start implementation should pull context first
- **When an agent asks "why"** — if a specialist agent needs to understand the reasoning behind a task
- **When context feels thin** — if a task's title and steps don't provide enough direction
- **When starting a new session** — to re-establish context that was lost between conversations

---

## Inputs

This skill accepts one of:
- A **task ID** (e.g., `task-3`) — most common, traces from the bottom up
- A **story ID** (e.g., `story-2`) — traces from story level up
- An **epic ID** (e.g., `epic-1`) — traces from epic level up
- **No input** — auto-discovers via `harnessx planning-tasks next`

If no input is provided, default to the next ready task.

---

## Step 1: Resolve the starting point

```bash
# If a task ID was given
harnessx planning-tasks list
# Parse the JSON to find the specific task

# If no ID was given, get the next ready task
harnessx planning-tasks next
```

Capture the full task object. You need: `id`, `title`, `steps`, `story`, `skills`, `complexity`, `traces` (tags, intake_sources, output_sources), and `integration_tests`.

If starting from a story or epic, skip to the appropriate level in Step 2.

---

## Step 2: Walk up the hierarchy

From the task, follow the parent chain all the way to the milestone. Run these commands to collect each level:

```bash
# Get the parent story (task.story gives you the reference, e.g., "#story-1")
harnessx planning-tasks parent <task-id>

# Get the parent epic (story.epic gives you the reference)
harnessx planning-stories parent <story-id>

# Get the parent milestone (epic.milestone gives you the reference)
harnessx planning-epics parent <epic-id>
```

At each level, capture:
- **Title and description** — the "what" at that level of abstraction
- **Acceptance criteria** (stories) — the testable definition of done
- **Success measures** (milestones) — the measurable outcomes
- **Status** — what's already complete vs. in progress
- **Traces** — the `tags` and `intake_sources` arrays

Build a hierarchy map:
```
Milestone: [milestone title]
  Epic: [epic title]
    Story: [story title]
      Task: [task title] (YOUR TASK)
```

---

## Step 3: Collect all traced action items

Gather every unique `#action-N` tag referenced in `traces.tags` across all four levels (task, story, epic, milestone). These action items are the bridge between planning and intake.

```bash
# List all action items
harnessx intake-actions list
```

From the JSON output, extract each referenced action item's:
- `title` — what needs to happen
- `detail` — the *why* (this is often the most valuable field)
- `origin` — which intake section produced this action (e.g., `intake:goal`, `intake:scope`)
- `category` — the domain area
- `notes` — any agent observations with additional context

---

## Step 4: Pull intake document context

From the hierarchy's `traces.intake_sources` and the action items' `origin` fields, identify which intake documents matter. Use `harnessx context search-context` to pull the relevant paragraphs.

For each unique intake source reference (e.g., `#intake-goal`, `#intake-scope`):

```bash
# Pull paragraphs tagged with specific action items
harnessx context search-context --query "#action-1"
harnessx context search-context --query "#action-5"

# Pull broader intake section context
harnessx context search-context --query "#intake-goal"
harnessx context search-context --query "#intake-scope"
```

Also read the key intake documents directly for richer context — particularly the ones referenced by the action items' `origin` fields. The intake folder lives at `harnessx/<project-id>/intake/`:
- `goal.md` — the project's purpose and user intent
- `scope.md` — what's in and out of bounds
- `success_measures.md` — how success is measured
- `user_acceptance_testing.md` — what the user will verify

Only read the documents that are actually referenced in the traces. Don't read everything — read what's relevant.

---

## Step 5: Get the next task

To give the agent forward context (what comes after their work), get the task that follows:

```bash
harnessx planning-tasks list
```

From the task list, find:
1. The task immediately after the current one (by order) within the same story
2. If no more tasks in this story, note that story completion comes next

Also check what sibling tasks exist (other tasks under the same story) to understand how this task fits into the story's overall implementation.

---

## Step 6: Synthesize the narrative summary

Combine everything into a structured narrative. The summary has two parts: a **context brief** and a **task brief**.

### Context Brief

Write this as a flowing narrative that grounds the agent in the project's full context:

```
PROJECT CONTEXT
===============

We are building [WHAT — from milestone/epic description] for [WHO — from goal.md
user context] so that [OUTCOME — from success measures and goal].

This work is part of:
- Milestone: "[milestone title]" — [milestone description, 1-2 sentences]
- Epic: "[epic title]" — [epic description, 1-2 sentences]
- Story: "[story title]" — [story description, 1-2 sentences]

Key context from intake:
[2-4 sentences synthesized from the most relevant intake paragraphs — the user's
original words about what they need and why, scope boundaries that affect this work,
and any domain insights from user_knowledge that matter]

Relevant action items:
- #action-N: [title] — [1-line summary of detail/why]
- #action-M: [title] — [1-line summary of detail/why]
```

### Task Brief

Write this as a focused briefing for the executing agent:

```
YOUR TASK
=========

Task: [task-id] — "[task title]"
Complexity: [complexity]
Skills: [assigned skills]

Steps:
1. [step 1]
2. [step 2]
...

Story acceptance criteria (your task contributes to these):
- [criterion 1]
- [criterion 2]
...

Integration tests expected:
- [test 1]
- [test 2]
...

Output files:
- [output_sources entries, if any]

WHAT COMES NEXT
===============

After your task, the next task is: [next-task-id] — "[next task title]" ([brief
description of what it does]). Your work should leave the codebase in a state where
that task can begin cleanly.

[OR if this is the last task in the story:]
Your task is the final task in this story. When complete, the story's acceptance
criteria should all pass.
```

---

## Important guidelines

### Be concise but complete
The summary should be thorough enough that an agent can start working without asking questions, but not so long that it overwhelms context. Aim for 40-80 lines total. Trim intake quotes to the most relevant sentences — don't dump entire documents.

### Prioritize the "why"
The most valuable thing you provide is *why* this task exists. The agent can read the task's steps — what it can't see is the chain of reasoning from user goal → milestone → epic → story → this task. That chain is your main deliverable.

### Preserve the user's voice
When quoting intake documents, keep the user's original phrasing where it adds clarity. "My boss wants to see profit margins as percentages" is more useful than "stakeholder requires percentage-based margin display."

### Flag scope boundaries
If `scope.md` contains any boundaries relevant to this task (things explicitly excluded, deferred, or constrained), call them out. Agents that don't know the boundaries will build beyond them.

### Surface risks from action item notes
If any traced action item has notes from risk reviews or agent interviews that relate to this task, include them. These are often blindspot warnings that the task writer couldn't encode in the steps alone.

---

## Output

Return the complete synthesized summary (Context Brief + Task Brief) as text to the requesting agent. This is not written to a file — it's returned directly in the conversation for the agent to use as grounding context.
