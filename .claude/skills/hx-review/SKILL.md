---
name: hx:review
description: >
  Full-spectrum project review that dispatches 5 specialist agents to audit the entire planning hierarchy
  (milestones and tasks) against intake documents, action items, and risk assessments.
  Reviews task ordering, task robustness, goal alignment, intake-actions traceability, and risk coverage —
  then launches per-milestone remediation agents to fix issues via the harnessx CLI.
  Use this skill when planning is complete and you need a quality gate before execution, when the user says
  "review the project", "audit the plan", "check the planning", "is this plan solid", "review before execution",
  "validate the plan", "sanity check", or anything about reviewing whether the planned work will actually
  achieve the project goal. Also trigger when the operator routes to the review stage, after all planning is
  complete, or when the user wants to verify that agents working independently will produce a coherent outcome.
disable-model-invocation: false
user-invocable: false
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

Launch **4 parallel data-gathering commands** and capture all outputs:

```bash
harnessx planning-milestones list
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
- [WARN-1] Description... | Affects: task-5 | Fix: ...

### Observations (informational, no action needed)
- [OBS-1] Description...

### Score: X/10
Brief justification for the score.
```

### Agent 1: Task Ordering & Dependencies

**Prompt to agent:**

> You are a dependency and ordering analyst. Your job is to find structural problems in the task execution order that would cause agents to fail or waste time.
>
> Given all milestones and tasks for this project, analyse:
>
> 1. **Execution order validity** — Within each milestone, tasks have an `execution_order` field that defines strict sequential ordering. Does the execution order reflect logical build sequence? Would executing tasks in this order cause an agent to reference code, structs, or APIs not yet created?
> 2. **Cross-milestone dependencies** — Are there implicit dependencies between tasks in different milestones that aren't captured by milestone ordering?
> 3. **Group coherence** — Tasks have `group` labels that cluster related work. Are groups logically coherent? Are there tasks that seem misplaced in their group?
> 4. **Purpose clarity** — Each task has a `purpose` field explaining why it exists. Are there tasks whose purpose doesn't clearly connect to their milestone's goal?
> 5. **Parallelisation opportunities** — Within a milestone, execution_order is strictly sequential. Are there tasks that could safely run in parallel if the model supported it? (Flag as observations, not issues.)
> 6. **Bottleneck detection** — Is there a single task that, if it fails, would make all subsequent tasks in the milestone invalid?
> 7. **Dead-end tasks** — Are there tasks that produce no artifacts consumed by later tasks and aren't the final task in their milestone?
>
> For each issue, specify exactly which task and milestone IDs are affected and what the fix should be.

**Model:** opus (needs deep reasoning about dependency graphs)

### Agent 2: Task Robustness

**Prompt to agent:**

> You are a task quality analyst. Your job is to determine whether each task is robust enough that a specialist agent — working alone with only the task definition and context tags — can execute it successfully without getting stuck, going off-track, or producing incomplete work.
>
> For every task, evaluate:
>
> 1. **Step completeness** — Are the steps detailed enough to guide implementation? Would an agent know what to do at each step, or are there gaps where it would have to guess?
> 2. **Skill assignment** — Is the assigned skill the right one for this work? Does the task need capabilities the assigned skill doesn't have?
> 3. **Task purpose and integration tests** — Does the task's `purpose` field clearly explain why this work matters? Do the integration tests actually verify the task's purpose? Are there scenarios that would catch real bugs but aren't covered?
> 4. **Complexity calibration** — Is the complexity rating realistic? Would a "low" task actually take significant thought? Would a "high" task be better split?
> 5. **Self-containment** — Can this task be completed in a single focused agent session, or does it implicitly require multiple sessions or human intervention?
> 6. **Context sufficiency** — Do the trace tags and intake sources give an agent enough context to understand why this task exists and what constraints apply?
> 7. **Output clarity** — Does the task make clear what files/code/artifacts it should produce? Will the next task in execution order know where to find what this task created?
>
> Flag any task where an independent agent would likely produce poor results or get stuck.

**Model:** opus (needs to simulate agent execution mentally)

### Agent 3: Goal Alignment

**Prompt to agent:**

> You are a goal alignment analyst. Your job is to verify that executing this entire plan — every milestone and task — will actually achieve what the user set out to do. Plans can drift from the original intent during decomposition, and your job is to catch that drift.
>
> Given the intake documents (goal, scope, success measures, UAT criteria) and the full planning hierarchy, analyse:
>
> 1. **Success measure coverage** — For each success measure in `success_measures.md`, trace it to at least one milestone and its tasks. Are any success measures not addressed by any task?
> 2. **UAT criteria coverage** — For each UAT criterion, is there a clear path from task completion → milestone completion that satisfies it?
> 3. **Scope adherence** — Are there milestones or tasks that address work explicitly listed as out-of-scope or deferred?
> 4. **Goal completeness** — After all milestones complete, would the user's goal (from `goal.md`) actually be achieved? What would still be missing?
> 5. **Milestone ordering** — Do the milestones build toward the goal in a logical progression? Would a demo at each milestone show meaningful progress?
> 6. **Feature gaps** — Is there anything the user clearly wants (from goal + scope) that has no corresponding planning artifact at any level?
> 7. **Over-engineering** — Are there tasks that go beyond what the user asked for? Gold-plating that adds complexity without addressing a stated need?
>
> Be specific — cite the exact text from intake documents and the exact planning artifact IDs.

**Model:** opus (needs to cross-reference intake text with planning artifacts)

### Agent 4: Intake-Actions Alignment

**Prompt to agent:**

> You are a traceability analyst. Your job is to ensure every action item from the intake process is accounted for in the planning hierarchy, and every planning artifact traces back to a real need.
>
> Given the full list of intake action items and the full planning hierarchy (milestones and tasks), analyse:
>
> 1. **Orphaned action items** — For each action item in `intake_actions.json`, check whether its ID (e.g., `#action-1`) appears in any planning artifact's `traces.tags`. List any action items with zero trace references — these represent work the user discussed during intake that was dropped during planning.
> 2. **Orphaned planning artifacts** — Are there milestones or tasks whose `traces.tags` are empty or only reference non-existent action items?
> 3. **Category alignment** — Do the action item categories (backend, frontend, research, etc.) align with the types of tasks that reference them? A "research" action traced only to implementation tasks may indicate skipped research.
> 4. **Input document coverage** — Action items with `input_docs` (URLs, paths) should trace to tasks that actually consume those resources. Are there input docs that no task references?
> 5. **Complexity consistency** — Are action items rated as "high" complexity traced to tasks rated as "low"? This suggests the planning underestimates the work.
> 6. **Mode alignment** — Action items in "review" mode should trace to review-type tasks. Action items in "execute" mode should trace to implementation tasks.
> 7. **Note preservation** — Do the blindspot notes on action items (often warnings from the intake-actions-writing skill about agent pitfalls) appear to be addressed by the task design?
>
> For each orphaned action item, recommend whether it should be addressed by a new task or is already covered implicitly.

**Model:** opus (needs to cross-reference two large datasets)

### Agent 5: Risk & Coherence

**Prompt to agent:**

> You are a risk and coherence analyst. Your job is to determine whether agents working independently — each executing their assigned tasks using their assigned skills — will produce a system that works as a coherent whole. Independent execution is the fundamental challenge: no agent sees what other agents produce until its dependencies are marked complete.
>
> Analyse:
>
> 1. **Integration seams** — Where do independently-built tasks need to connect? Are there shared interfaces (structs, traits, APIs, data formats) that multiple tasks assume but no single task is responsible for defining? This is the #1 failure mode.
> 2. **Architectural decisions** — Are there decisions (data structures, error handling strategy, logging approach, configuration format) that should be made once and early, but are left implicit across multiple tasks?
> 3. **Risk from intake-completion** — Read the intake documents for any risk assessment notes (from exploration, ideation, or project_risk_manager phases). Are those risks addressed by specific tasks?
> 4. **Error handling coherence** — Do tasks across different groups and milestones handle errors in compatible ways, or would each agent invent its own approach?
> 5. **Testing gaps** — Are there integration points between tasks that no single task's integration tests cover? End-to-end flows that cross task boundaries?
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
1. Identify which tasks are affected
2. Trace each affected task up to its milestone
3. Group all fixes by milestone

### Launching Remediation Agents

For each milestone that has fixes:

1. Run `harnessx planning-milestones children <milestone-id>` to get the full hierarchy
2. Launch an agent with:
   - The milestone and all its tasks
   - The specific findings that affect this milestone's tasks
   - Clear instructions on what CLI commands to run

**Remediation agent prompt template:**

> You are a planning remediation agent. You have been given a milestone and its tasks, along with specific review findings that need to be fixed.
>
> Your job is to fix each finding by running harnessx CLI commands. You MUST use CLI commands — never edit JSON files directly.
>
> **Available commands for fixes:**
>
> - `harnessx planning-tasks update <id> --execution-order N` — Fix ordering
> - `harnessx planning-tasks update <id> --steps "step 1 | step 2 | step 3"` — Improve steps
> - `harnessx planning-tasks update <id> --integration-tests "test 1 | test 2"` — Improve tests
> - `harnessx planning-tasks update <id> --complexity medium` — Fix complexity rating
> - `harnessx planning-tasks update <id> --skills "rust:developing"` — Fix skill assignment
> - `harnessx planning-tasks update <id> --trace-tags "#action-1,#action-2"` — Fix traceability
> - `harnessx planning-tasks update <id> --group "group-name"` — Fix group assignment
> - `harnessx planning-tasks update <id> --purpose "..."` — Fix purpose description
> - `harnessx planning-tasks update <id> --note "Review fix: ..."` — Document what was changed and why
> - `harnessx planning-tasks create ... --milestone "#[milestone-id]" ...` — Create new tasks to fill gaps (always include `--milestone` with the parent milestone ID)
> - `harnessx planning-milestones update <id> ...` — Update milestones
>
> Read the CLI docs if you need to confirm exact flag names:
> - `docs/planning-tasks.md`
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
- `docs/planning-milestones.md`
- `docs/intake-actions.md`

---

## Phase 4: Complete Review

After remediation is complete (or if no critical issues were found), mark the review stage as done.

### After remediation

When remediation agents have finished and you've reported changes to the user, ask:

> "Are you satisfied with the remediation, or would you like to re-run the review to validate the changes?"

If the user is satisfied:

```bash
harnessx progress complete review
```

Tell the user: "Review is complete. The pipeline will advance to execution on next invocation. Start a new session and run `/hx:operator` to begin executing tasks."

If the user wants a re-review, tell them to run `/hx:operator` again in a new session. Do not mark complete.

### No critical issues found

If Phase 2's synthesis shows zero critical issues and zero warnings (only observations), and the overall score is 8/10 or higher, tell the user:

> "The plan looks solid — no critical issues or warnings. Ready to mark review complete and move to execution?"

If the user confirms:

```bash
harnessx progress complete review
```

If the user wants to address observations or run further checks, do not mark complete.

### Important

Never auto-complete review without user confirmation. The review is a quality gate — the user must explicitly approve moving to execution.
