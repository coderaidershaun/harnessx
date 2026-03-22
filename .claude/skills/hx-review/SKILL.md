---
name: hx:review
description: >
  Full-spectrum project review that dispatches 5 specialist agents to audit the entire planning hierarchy
  (milestones, epics, stories, tasks) against intake documents, action items, and risk assessments.
  Reviews task ordering, task robustness, goal alignment, intake-actions traceability, and risk coverage —
  then launches per-milestone remediation agents to fix issues via the harnessx CLI.
  Use this skill when planning is complete and you need a quality gate before execution, when the user says
  "review the project", "audit the plan", "check the planning", "is this plan solid", "review before execution",
  "validate the plan", "sanity check", or anything about reviewing whether the planned work will actually
  achieve the project goal. Also trigger when the operator routes to the review stage, after all planning is
  complete, or when the user wants to verify that agents working independently will produce a coherent outcome.
user-invocable: true
disable-model-invocation: false
---

# Project Review

This skill is a quality gate between planning and execution. It answers one question: **will this plan actually work when agents execute it independently?**

Five specialist review agents examine the plan from different angles, then remediation agents fix what they find — all via the harnessx CLI.

## Startup

### 1. Verify Active Project

```bash
harnessx project active
```

If no active project, tell the user to run `/hx:operator` first and stop.

Capture the project metadata — you'll need `user_name`, `title`, and the project ID (from the response's `id` field).

### 2. Load Full Project State

Launch **6 parallel data-gathering commands** and capture all outputs:

```bash
harnessx planning-milestones list
harnessx planning-epics list
harnessx planning-stories list
harnessx planning-tasks list
harnessx intake-actions list
harnessx project active
```

If any planning list returns an error (e.g., no milestones found), the project hasn't been planned yet. Tell the user planning needs to happen first and stop.

### 3. Load Intake Documents

Read the intake markdown files from the active project directory. The project ID comes from `harnessx project active` — files live at `harnessx/<project-id>/intake/`:

- `goal.md`
- `scope.md`
- `user_knowledge.md`
- `resources.md`
- `success_measures.md`
- `user_acceptance_testing.md`

Read whichever exist. These are the source of truth for what the user actually wants.

## Phase 1: Dispatch 5 Review Agents

Launch all 5 agents **in parallel** using the Agent tool. Each agent receives the full project state (all lists + intake docs) and specialises in one review dimension.

Each agent must return a structured review in this format:

```
## [Review Dimension] Review

### Critical Issues (must fix before execution)
- [CRIT-1] Description... | Affects: task-3, task-7 | Fix: ...

### Warnings (should fix, execution may succeed without)
- [WARN-1] Description... | Affects: epic-2 | Fix: ...

### Observations (informational, no action needed)
- [OBS-1] Description...

### Score: X/10
Brief justification for the score.
```

### Agent 1: Task Ordering & Dependencies

**Prompt to agent:**

> You are a dependency and ordering analyst. Your job is to find structural problems in the task execution order that would cause agents to fail or waste time.
>
> Given all milestones, epics, stories, and tasks for this project, analyse:
>
> 1. **Circular dependencies** — Are there any cycles in `depends_on` chains? Trace every dependency chain to completion.
> 2. **Missing dependencies** — Are there tasks that reference code, structs, or APIs that would be created by another task, but don't list that task as a dependency?
> 3. **Ordering within stories** — Do task `order` values within each story reflect logical build sequence? Would an agent picking up task-3 before task-2 (if both were ready) produce a broken result?
> 4. **Cross-story dependencies** — Are there implicit dependencies between stories in different epics that aren't captured in `depends_on`?
> 5. **Parallelisation opportunities** — Are there tasks marked as sequential (via depends_on) that could safely run in parallel?
> 6. **Bottleneck detection** — Is there a single task that blocks many others? If it fails, does the entire plan stall?
> 7. **Dead-end tasks** — Are there tasks that nothing depends on and that aren't the final task in their story?
>
> For each issue, specify exactly which task/story/epic IDs are affected and what the fix should be.

**Model:** opus (needs deep reasoning about dependency graphs)

### Agent 2: Task Robustness

**Prompt to agent:**

> You are a task quality analyst. Your job is to determine whether each task is robust enough that a specialist agent — working alone with only the task definition and context tags — can execute it successfully without getting stuck, going off-track, or producing incomplete work.
>
> For every task, evaluate:
>
> 1. **Step completeness** — Are the steps detailed enough to guide implementation? Would an agent know what to do at each step, or are there gaps where it would have to guess?
> 2. **Skill assignment** — Is the assigned skill the right one for this work? Does the task need capabilities the assigned skill doesn't have?
> 3. **Integration test coverage** — Do the integration tests actually verify the task's purpose? Are there scenarios that would catch real bugs but aren't covered?
> 4. **Complexity calibration** — Is the complexity rating realistic? Would a "low" task actually take significant thought? Would a "high" task be better split?
> 5. **Self-containment** — Can this task be completed in a single focused agent session, or does it implicitly require multiple sessions or human intervention?
> 6. **Context sufficiency** — Do the trace tags and intake sources give an agent enough context to understand why this task exists and what constraints apply?
> 7. **Output clarity** — Does the task make clear what files/code/artifacts it should produce? Will the next task in the chain know where to find what this task created?
>
> Flag any task where an independent agent would likely produce poor results or get stuck.

**Model:** opus (needs to simulate agent execution mentally)

### Agent 3: Goal Alignment

**Prompt to agent:**

> You are a goal alignment analyst. Your job is to verify that executing this entire plan — every milestone, epic, story, and task — will actually achieve what the user set out to do. Plans can drift from the original intent during decomposition, and your job is to catch that drift.
>
> Given the intake documents (goal, scope, success measures, UAT criteria) and the full planning hierarchy, analyse:
>
> 1. **Success measure coverage** — For each success measure in `success_measures.md`, trace it to at least one milestone. Are any success measures not addressed by any milestone?
> 2. **UAT criteria coverage** — For each UAT criterion, is there a clear path from task completion → story completion → epic completion → milestone that satisfies it?
> 3. **Scope adherence** — Are there milestones, epics, or stories that address work explicitly listed as out-of-scope or deferred?
> 4. **Goal completeness** — After all milestones complete, would the user's goal (from `goal.md`) actually be achieved? What would still be missing?
> 5. **Milestone ordering** — Do the milestones build toward the goal in a logical progression? Would a demo at each milestone show meaningful progress?
> 6. **Feature gaps** — Is there anything the user clearly wants (from goal + scope) that has no corresponding planning artifact at any level?
> 7. **Over-engineering** — Are there tasks or epics that go beyond what the user asked for? Gold-plating that adds complexity without addressing a stated need?
>
> Be specific — cite the exact text from intake documents and the exact planning artifact IDs.

**Model:** opus (needs to cross-reference intake text with planning artifacts)

### Agent 4: Intake-Actions Alignment

**Prompt to agent:**

> You are a traceability analyst. Your job is to ensure every action item from the intake process is accounted for in the planning hierarchy, and every planning artifact traces back to a real need.
>
> Given the full list of intake action items and the full planning hierarchy, analyse:
>
> 1. **Orphaned action items** — For each action item in `intake_actions.json`, check whether its ID (e.g., `#action-1`) appears in any planning artifact's `traces.tags`. List any action items with zero trace references — these represent work the user discussed during intake that was dropped during planning.
> 2. **Orphaned planning artifacts** — Are there milestones, epics, stories, or tasks whose `traces.tags` are empty or only reference non-existent action items?
> 3. **Category alignment** — Do the action item categories (backend, frontend, research, etc.) align with the types of tasks that reference them? A "research" action traced only to implementation tasks may indicate skipped research.
> 4. **Input document coverage** — Action items with `input_docs` (URLs, paths) should trace to tasks that actually consume those resources. Are there input docs that no task references?
> 5. **Complexity consistency** — Are action items rated as "high" complexity traced to tasks rated as "low"? This suggests the planning underestimates the work.
> 6. **Mode alignment** — Action items in "review" mode should trace to review-type tasks. Action items in "execute" mode should trace to implementation tasks.
> 7. **Note preservation** — Do the blindspot notes on action items (often warnings from the intake-actions-writing skill about agent pitfalls) appear to be addressed by the task design?
>
> For each orphaned action item, recommend which planning level it should be addressed at (new task? new story? already covered implicitly?).

**Model:** opus (needs to cross-reference two large datasets)

### Agent 5: Risk & Coherence

**Prompt to agent:**

> You are a risk and coherence analyst. Your job is to determine whether agents working independently — each executing their assigned tasks using their assigned skills — will produce a system that works as a coherent whole. Independent execution is the fundamental challenge: no agent sees what other agents produce until its dependencies are marked complete.
>
> Analyse:
>
> 1. **Integration seams** — Where do independently-built components need to connect? Are there shared interfaces (structs, traits, APIs, data formats) that multiple tasks assume but no single task is responsible for defining? This is the #1 failure mode.
> 2. **Architectural decisions** — Are there decisions (data structures, error handling strategy, logging approach, configuration format) that should be made once and early, but are left implicit across multiple tasks?
> 3. **Risk from intake-completion** — Read the intake documents for any risk assessment notes (from exploration, ideation, or project_risk_manager phases). Are those risks addressed by specific tasks?
> 4. **Error handling coherence** — Do tasks across different stories/epics handle errors in compatible ways, or would each agent invent its own approach?
> 5. **Testing gaps** — Are there integration points between components that no single task's integration tests cover? End-to-end flows that cross task boundaries?
> 6. **Agent context loss** — Given that each agent only sees its task definition + context tags, are there critical pieces of context that would be lost? Things an agent needs to know but wouldn't discover from the traces alone?
> 7. **Sum-of-parts check** — If every task were completed perfectly to spec, would the result be a working system? Or are there assembly steps, configuration, deployment, or integration work that nobody owns?
>
> For each risk, rate severity (critical/warning/observation) and recommend a specific fix (new task, modified task, architectural decision task, etc.).

**Model:** opus (needs holistic systems thinking)

## Phase 2: Synthesise Review

Once all 5 agents return, synthesise their findings into a unified report.

### Deduplication

Multiple agents may flag the same issue from different angles (e.g., Agent 1 flags a missing dependency while Agent 5 flags the same integration seam). Merge these into a single finding with the most severe rating.

### Priority Ranking

Rank all findings by impact:

1. **Critical** — Execution would fail or produce a broken system. Must fix.
2. **Warning** — Execution might succeed but with quality issues. Should fix.
3. **Observation** — Worth noting but not blocking. Optional.

### Present to User

Show the user a clear summary:

```
## Project Review: [project title]

### Overall Score: X/10
[One paragraph synthesis — is this plan ready for execution?]

### Critical Issues (N found)
[Numbered list with affected artifacts and recommended fixes]

### Warnings (N found)
[Numbered list]

### Observations (N found)
[Numbered list]

### Review Agent Scores
- Task Ordering: X/10
- Task Robustness: X/10
- Goal Alignment: X/10
- Actions Alignment: X/10
- Risk & Coherence: X/10
```

Ask the user: **"Which issues should I fix? I can address all critical + warning issues, or you can tell me which specific ones to tackle."**

Wait for the user's response before proceeding to remediation.

## Phase 3: Remediation

Based on the user's selection, launch remediation agents — one per milestone that has affected children.

### Grouping Fixes by Milestone

For each finding the user wants fixed:
1. Identify which tasks/stories/epics are affected
2. Trace each affected artifact up to its milestone (task → story → epic → milestone)
3. Group all fixes by milestone

### Launching Remediation Agents

For each milestone that has fixes:

1. Run `harnessx planning-milestones children <milestone-id>` to get the full hierarchy
2. Launch an agent with:
   - The milestone and all its children (epics, stories, tasks)
   - The specific findings that affect this milestone's children
   - Clear instructions on what CLI commands to run

**Remediation agent prompt template:**

> You are a planning remediation agent. You have been given a milestone and its full hierarchy (epics, stories, tasks), along with specific review findings that need to be fixed.
>
> Your job is to fix each finding by running harnessx CLI commands. You MUST use CLI commands — never edit JSON files directly.
>
> **Available commands for fixes:**
>
> - `harnessx planning-tasks update <id> --depends-on "task-1,task-2"` — Fix dependencies
> - `harnessx planning-tasks update <id> --order N` — Fix ordering
> - `harnessx planning-tasks update <id> --steps "step 1 | step 2 | step 3"` — Improve steps
> - `harnessx planning-tasks update <id> --integration-tests "test 1 | test 2"` — Improve tests
> - `harnessx planning-tasks update <id> --complexity medium` — Fix complexity rating
> - `harnessx planning-tasks update <id> --skills "rust:developing"` — Fix skill assignment
> - `harnessx planning-tasks update <id> --trace-tags "#action-1,#action-2"` — Fix traceability
> - `harnessx planning-tasks update <id> --note "Review fix: ..."` — Document what was changed and why
> - `harnessx planning-tasks create ...` — Create new tasks to fill gaps
> - `harnessx planning-stories update <id> ...` — Update stories
> - `harnessx planning-stories create ...` — Create new stories to fill gaps
> - `harnessx planning-epics update <id> ...` — Update epics
> - `harnessx planning-milestones update <id> ...` — Update milestones
>
> Read the CLI docs if you need to confirm exact flag names:
> - `docs/planning-tasks.md`
> - `docs/planning-stories.md`
> - `docs/planning-epics.md`
> - `docs/planning-milestones.md`
>
> **For every fix, append a note** explaining what was changed and why (referencing the finding ID).
>
> **Findings to fix for milestone [ID]:**
> [insert findings here]
>
> **Current milestone hierarchy:**
> [insert children output here]
>
> After making all fixes, report a summary of what you changed.

**Model:** sonnet (remediation is mechanical — running CLI commands based on clear instructions)

### After Remediation

Once all remediation agents complete, report to the user:

```
## Remediation Complete

### Changes Made
- [milestone-1]: Updated N tasks, created M new tasks
  - task-3: Fixed dependencies (added #task-1)
  - task-7: Improved steps (was 2 steps, now 5)
  - task-12: Created new task for shared struct definitions
- [milestone-2]: Updated N tasks
  - ...

### Re-review Recommended: [yes/no]
[If changes were extensive, recommend running /hx:review again]
```

## Important Notes

### CLI-Only Updates

All modifications to planning artifacts MUST go through the harnessx CLI. Never read or write JSON files directly. The CLI ensures:
- Consistent ID generation for new items
- Proper JSON structure
- Active project resolution

### Note Discipline

Every update made during remediation must include a `--note` explaining what was changed and why. This creates an audit trail that future agents can reference.

### When to Re-review

If remediation created new tasks or significantly restructured dependencies, recommend the user run `/hx:review` again. The new tasks themselves need the same quality checks.

### Reading CLI Docs

Before running CLI commands, read the relevant doc file to confirm exact flag names and behaviour:
- `docs/planning-tasks.md`
- `docs/planning-stories.md`
- `docs/planning-epics.md`
- `docs/planning-milestones.md`
- `docs/intake-actions.md`
