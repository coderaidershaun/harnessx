---
name: hx:uat-rework
description: >
  Read UAT feedback from the user acceptance testing phase, create a uat_rework milestone with
  epics, stories, and tasks to address all feedback items, reset the pipeline for re-execution,
  and mark the rework planning complete. This skill runs when the pipeline reaches uat_rework
  after the user has requested changes during user acceptance testing. Use this skill when the
  operator routes to the uat_rework stage, when the user says "plan rework", "fix UAT issues",
  "create rework plan", "address UAT feedback", or when the pipeline naturally advances to
  uat_rework after user_acceptance completes with rework requested.
disable-model-invocation: false
user-invocable: false
---

# UAT Rework

This skill takes structured feedback from user acceptance testing and turns it into an actionable rework plan. It creates a milestone with the full planning hierarchy (epics, stories, tasks) so the execution engine can pick up the work immediately.

Unlike the automated milestone-level rework (which runs 4 review agents autonomously), UAT rework is driven by the user's direct feedback. The user tested the deliverables, found specific issues, and this skill translates those issues into work the agents can execute.

The rework plan does NOT get a companion rework milestone (unlike main milestones). The UAT cycle itself is the verification mechanism — when rework tasks complete, the user re-tests during the next user_acceptance round.

---

## Phase 1: Load Context

### 1a. Get active project

```bash
harnessx project active
```

Capture the project ID, directory, and user_name.

```bash
harnessx progress update uat_rework in_progress
```

### 1b. Read UAT feedback

Read `harnessx/<project-id>/uat_feedback.md`.

**If the file does not exist or is empty:** This is an error state — the uat_rework stage should only run after user_acceptance has collected rework feedback. Report to the user: "No UAT feedback found. This stage should only run after user acceptance testing has identified specific issues for rework." Mark complete and stop:

```bash
harnessx progress complete uat_rework
```

Extract:
- The round number (from `Round:` field)
- All FAIL scenarios with their expected/actual/severity
- All PARTIAL scenarios with their details/severity
- Passed scenarios (for regression awareness)
- User priority notes

### 1c. Read intake and planning context

Read these files in parallel from `harnessx/<project-id>/`:

- `intake/goal.md` — project goal
- `intake/scope.md` — scope boundaries
- `intake/success_measures.md` — success measures
- `intake/user_acceptance_testing.md` — original UAT criteria
- `history.md` — execution history

```bash
harnessx planning-milestones list
harnessx planning-tasks list
```

Capture the current state of milestones and tasks to understand what was already completed and what the rework builds on.

### 1d. Catalog available specialist skills

```bash
ls .claude/skills/
```

Identify which skill families exist and their team leads (e.g., `rust:team-coordinator` for the Rust family). Read the SKILL.md frontmatter of any skill you're uncertain about.

---

## Phase 2: Create UAT Rework Milestone

Determine the milestone title from the feedback. Summarize the top 2-3 issues.

```bash
harnessx planning-milestones create \
  --title "UAT Rework Round N: [summary of key issues]" \
  --description "Address user acceptance testing feedback from round N. Key issues: [list top 2-3 issues from feedback]." \
  --status not_started \
  --note "Created by hx:uat-rework from uat_feedback.md round N. Issues: X failed, Y partial scenarios."
```

Capture the auto-assigned milestone ID from the CLI response.

---

## Phase 3: Plan the Rework Hierarchy

This is the core of the skill. Use a dual-agent approach (proposer + reviewer) to create a robust rework plan, following the same methodology as `hx:planning-tasks`.

### Agent 1: Rework Proposer (opus)

Launch a subagent with all context. Its job is to propose the complete planning hierarchy for the rework milestone.

```
You are planning UAT rework for round N. The user tested the project and found these issues:

UAT FEEDBACK:
[paste the full content of uat_feedback.md]

PROJECT CONTEXT:
- Goal: [from goal.md]
- Scope: [from scope.md]
- Success measures: [from success_measures.md]
- Original UAT criteria: [from user_acceptance_testing.md]
- Completed milestones: [summary from milestone list]

AVAILABLE SPECIALIST SKILLS:
[list all non-hx skills with names and descriptions]

EXISTING TASKS (completed):
[summary of completed tasks, especially those related to the failing scenarios]

Your job: propose a complete rework plan — epics, stories, and tasks — that addresses every FAIL and PARTIAL scenario from the feedback. For each level:

EPICS: Group related feedback items into coherent capability areas (1-3 epics). Each epic should represent a distinct area of rework. Don't create an epic per feedback item unless they're truly unrelated.

STORIES: For each epic, define testable behavioral increments. Each story should have acceptance criteria derived directly from the UAT feedback — specifically, the "Expected" field from FAIL scenarios and the "What Doesn't" field from PARTIAL scenarios become acceptance criteria.

TASKS: For each story, define atomic implementation tasks. For each task provide:
1. Title — specific, actionable (verb + noun + context)
2. Steps — ordered implementation steps (3-7 per task), concrete enough for an agent to follow
3. Complexity — super-low, low, medium, high, or super-high
4. Skills — which specialist skill(s) should execute this. Use team leads (like rust:team-coordinator) for non-trivial work
5. Integration tests — how to verify this specific task works
6. Dependencies — which other proposed tasks must be done first

GUIDELINES:
- Every FAIL scenario must be addressed. Every PARTIAL scenario must be addressed.
- Passed scenarios should NOT be touched unless fixing a failed scenario requires it. Note regression risks.
- Severity from the feedback should influence task priority: critical items first.
- Steps should reference specific code paths when possible (check history for what was built).
- Keep tasks bite-sized — completable in one focused session, touching 1-3 files max.
- The user's priority notes (if any) should influence ordering.
```

**Model:** opus

### Agent 2: Rework Reviewer (opus)

After receiving the proposer's output, launch a second subagent to critique.

```
Review this proposed UAT rework plan for round N.

UAT FEEDBACK:
[paste uat_feedback.md content]

PROPOSED PLAN:
[paste Agent 1's output]

Validate with these checks:

1. **Feedback coverage** — Map each FAIL and PARTIAL scenario to the task(s) that address it. Flag any feedback item not covered by any task.

2. **Regression risk** — Do any proposed changes risk breaking passed scenarios? If so, flag which passed scenarios are at risk and recommend adding a regression check.

3. **Right-sized tasks** — Would any task require an agent to hold more than 2-3 files in context? If so, it needs splitting.

4. **Skill assignment** — Is each task assigned to the right skill? Use team leads for non-trivial work. Direct specialists only for trivially simple, single-concern tasks.

5. **Step quality** — Are steps concrete enough for an agent who hasn't seen this codebase to follow?

6. **Missing work** — Is there anything implied by the feedback that nobody planned for? Common gaps: error handling changes, UI updates that need backend changes, test updates for changed behavior.

7. **Acceptance criteria quality** — Do story acceptance criteria directly map to what the user expected (from the feedback's Expected/What Doesn't fields)?

Provide specific, actionable critique. For each issue, explain what's wrong and how to fix it.
```

**Model:** opus

### Synthesize and Finalize

After both agents return, synthesize. The reviewer's critiques should be addressed unless they're incorrect. Produce the final plan: epics, stories, and tasks.

---

## Phase 4: Write the Plan via CLI

Read `docs/planning-epics.md`, `docs/planning-stories.md`, and `docs/planning-tasks.md` if you haven't this session — flag names and pipe-separated fields matter.

### Create epics

For each epic in the finalized plan:

```bash
harnessx planning-epics create \
  --title "[epic title]" \
  --milestone "#[uat-rework-milestone-id]" \
  --description "[epic description]" \
  --note "UAT rework round N. Addresses: [list the feedback scenarios this epic covers]"
```

Capture each auto-assigned epic ID.

### Create stories

For each story under each epic:

```bash
harnessx planning-stories create \
  --title "[story title]" \
  --epic "#[epic-id]" \
  --description "[story description]" \
  --acceptance-criteria "[criterion 1 | criterion 2 | criterion 3]" \
  --note "Derived from UAT feedback: [scenario title]"
```

Acceptance criteria use **pipe separators** (`|`).

### Create tasks

For each task under each story:

```bash
harnessx planning-tasks create \
  --title "[task title]" \
  --steps "[step 1 | step 2 | step 3]" \
  --story "#[story-id]" \
  --complexity [level] \
  --mode plan \
  --skills "[skill-name]" \
  --integration-tests "[test 1 | test 2]" \
  --note "UAT rework: [brief description of what this fixes from feedback]"
```

Steps and integration tests use **pipe separators** (`|`). Dependencies use **comma separators**.

If a task depends on another rework task:

```bash
--depends-on "task-N"
```

### Mark all as written

After creating the full hierarchy:

```bash
harnessx planning-milestones mark-written [uat-rework-milestone-id]
harnessx planning-epics mark-written [epic-id]
# ... for each epic
harnessx planning-stories mark-written [story-id]
# ... for each story
```

This tells downstream planning skills to skip these items — they're fully planned.

---

## Phase 5: Reset Pipeline and Complete

Reset the execution and user_acceptance stages so the pipeline loops:

```bash
harnessx progress update execution not_started
harnessx progress update user_acceptance not_started
harnessx progress complete uat_rework
```

Now `progress next` will return `execution` (the first incomplete stage). The execution engine will pick up the new rework tasks via `planning-tasks next` — all previously completed tasks remain completed, so only the new rework tasks are returned.

---

## Phase 6: Report and Record

### Append to history

Read `harnessx/<project-id>/history.md` first, then append:

```markdown
## UAT Rework Planning: Round N
**Date:** [today]
**Feedback Items:** X failed, Y partial scenarios
**Milestone Created:** [milestone-id] — "[title]"
**Epics:** [count] | **Stories:** [count] | **Tasks:** [count]
**Pipeline Reset:** execution and user_acceptance reset to not_started
```

### Report to user

Tell the user what was created:

> "I've created a rework plan based on your UAT feedback:
>
> - **Milestone:** [title]
> - **[N] epics**, **[M] stories**, **[P] tasks**
> - Key areas: [list epic titles]
>
> The execution and user acceptance stages have been reset. Run `/hx:operator` to start executing the rework tasks. When they're complete, you'll go through UAT again."

---

## Important Notes

### CLI-only mutations

All state changes and plan creation go through the harnessx CLI. Never edit JSON files directly.

### No companion rework milestone

Unlike main milestones during initial planning, UAT rework milestones do NOT get a companion automated rework milestone. The user's re-testing during the next `user_acceptance` round serves as the verification mechanism. This prevents infinite nesting of rework milestones.

### Pipe-separated fields

`--steps`, `--integration-tests`, and `--acceptance-criteria` use pipe (`|`) separators. `--depends-on`, `--skills`, `--trace-tags`, and `--trace-intake-sources` use comma (`,`) separators.

### Team lead assignment

When a skill family has a team lead (like `rust:team-coordinator`), assign the team lead by default for non-trivial tasks. Direct specialist assignment only for trivially simple, single-concern work (commenting, simple refactoring).

### History is append-only

Read `history.md` before appending. Never overwrite.

### Feedback file persists for re-test context

Do NOT delete `uat_feedback.md` after creating the rework plan. The `hx:user-acceptance` skill reads it during re-test to frame the context: "In the previous round, you identified these issues..."
