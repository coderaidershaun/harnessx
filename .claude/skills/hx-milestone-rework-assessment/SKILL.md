---
name: hx:milestone-rework-assessment
description: >
  Autonomous milestone review that runs all tests, dispatches 4 specialist review agents to assess
  completed work against success measures, and creates rework tasks via the harnessx CLI when issues
  are found. Assigned to the initial review task in each rework milestone. Runs on full autopilot —
  no user gates. Use this skill when a rework milestone's review task is dispatched during execution,
  or when the user says "review milestone", "assess milestone work", "run milestone review".
disable-model-invocation: false
user-invocable: false
---

# Milestone Rework Assessment

This is the skill assigned to the initial review task in each rework milestone. It runs autonomously — no user gates, no confirmation prompts. Its job is to verify the completed main milestone's work by running the full test suite and dispatching four specialist review agents, then creating targeted rework tasks via the harnessx CLI when issues are found.

The rework milestone depends on its corresponding main milestone. When the main milestone completes and this review task becomes ready, the execution engine dispatches it here. This skill takes over from that point: run tests, assess the work from four angles, synthesize findings, create rework tasks for anything that needs fixing, and report results.

---

## Phase 1: Setup

### 1. Get Active Project

```bash
harnessx project active
```

Capture the project ID, directory, and metadata.

### 2. Trace Parent Chain to Find the Main Milestone

Walk up the hierarchy from this task to the rework milestone, then follow `depends_on` to the main milestone:

```bash
harnessx planning-tasks parent [THIS-TASK-ID]
```

Capture the parent story ID.

```bash
harnessx planning-stories parent [STORY-ID]
```

Capture the parent epic ID.

```bash
harnessx planning-epics parent [EPIC-ID]
```

Capture the parent milestone ID — this is the **rework milestone**.

Also capture the **epic ID** from the step above — you will need it when creating rework tasks (the `--epic` flag).

Read the rework milestone's `depends_on` field to find the **main milestone ID**.

```bash
harnessx planning-milestones get [MAIN-MILESTONE-ID]
```

Capture the main milestone — its title, success_measures, and all metadata.

```bash
harnessx planning-milestones children [MAIN-MILESTONE-ID]
```

Capture the full hierarchy of completed work: all epics, stories, and tasks under the main milestone, including their notes (which contain execution summaries from the agents that completed them).

### 3. Load Intake Documents

Read from `harnessx/[PROJECT-ID]/intake/`:

- `success_measures.md`
- `user_acceptance_testing.md`
- `scope.md`

### 4. Capture Key References

Before proceeding, confirm you have:

- **Main milestone's success measures** — from the milestone object and intake docs
- **UAT criteria** — from the intake docs
- **All tasks with their notes** — execution summaries from the agents that did the work
- **Rework story ID** — the story under the rework milestone where new tasks will be created
- **Rework epic ID** — the epic under the rework milestone (captured during the parent chain walk)
- **This review task's ID** — rework tasks will use this as their `depends_on`

---

## Phase 2: Run All Tests (sequentially)

Launch a single agent to run the test suites. Tests are the hard gate — test failures are always Critical severity.

**Agent prompt:**

> Run the full test suite for this project. Execute sequentially — do not run suites in parallel.
>
> ```bash
> cd [PROJECT-DIRECTORY]
> cargo test -- --test-threads=1 2>&1
> cargo test -- --ignored --test-threads=1 2>&1
> ```
>
> Capture the FULL output from both commands. Report:
>
> 1. **Total tests run, passed, failed** for each suite (regular and ignored)
> 2. **For each failure**: test name, assertion message, relevant stack trace
> 3. **Any compilation errors or warnings**
> 4. **Do NOT fix anything** — only report what you find
>
> Return the complete test output followed by your structured summary.

**Model:** sonnet (running and reporting shell output — no deep reasoning needed)

---

## Phase 3: Dispatch 4 Review Agents (parallel)

All agents receive the same base context:
- The main milestone and its full hierarchy (all epics, stories, tasks with their notes/execution summaries)
- The test results from Phase 2
- The relevant intake documents (success_measures.md, user_acceptance_testing.md, scope.md)

Launch all 4 agents in parallel.

**IMPORTANT: Do NOT set `run_in_background: true`.** All agents must run in foreground — their results are needed before the next phase can proceed.

### Agent 1: Test Analyst

**Prompt:**

> You are a test analyst reviewing completed milestone work. You have the test results and the list of integration tests defined in each task under this milestone.
>
> Analyse:
> 1. **TEST COVERAGE** — Are all integration tests defined in the tasks actually present in the codebase? List any defined-but-missing tests.
> 2. **PASS/FAIL ANALYSIS** — For each test failure, determine: is this a code bug, a test environment issue, or a test definition problem?
> 3. **SUCCESS MEASURE VERIFICATION** — For each milestone success measure, is there at least one passing test that provides evidence it is satisfied?
> 4. **UAT CRITERIA COVERAGE** — For each UAT criterion, can the passing tests collectively demonstrate it?
> 5. **GAPS** — Are there success measures or UAT criteria with NO test coverage at all?
>
> Return structured findings as:
> ## Test Review
> ### Critical Issues (must fix)
> ### Warnings (should fix)
> ### Observations
> ### Score: X/10

**Model:** opus (needs to cross-reference test results with task definitions and success measures)

### Agent 2: Code Quality & Completeness

**Prompt:**

> You are a code quality and completeness analyst. You have all tasks under this milestone with their steps, output_sources, and notes (which contain execution summaries).
>
> For each task in this milestone, verify:
> 1. **OUTPUT FILES EXIST** — Do the files listed in traces.output_sources actually exist? Read them.
> 2. **STEPS EXECUTED** — Based on task notes and git history, were all steps actually completed?
> 3. **CODE QUALITY** — Read the key files produced. Look for: unwrap() calls that should be handled, TODO/FIXME comments left behind, placeholder implementations, dead code, missing error handling.
> 4. **STORY ACCEPTANCE CRITERIA** — For each story, read its acceptance_criteria. Based on the code delivered, are they satisfied?
>
> Focus on SUBSTANCE not style. Do not flag formatting preferences.
>
> Return structured findings as:
> ## Code Quality & Completeness Review
> ### Critical Issues (must fix)
> ### Warnings (should fix)
> ### Observations
> ### Score: X/10

**Model:** opus (needs to read actual code files and reason about quality)

### Agent 3: Cross-Component Integration

**Prompt:**

> You are a cross-component integration analyst. Agents executed tasks independently. Your job is to find where their work does not integrate properly.
>
> Analyse:
> 1. **INTERFACE COMPATIBILITY** — Do components built by different tasks use compatible types, function signatures, and data formats?
> 2. **SHARED STATE** — Are there shared resources that multiple tasks touch? Are they handled consistently?
> 3. **ERROR PROPAGATION** — Do errors from one component propagate correctly through others?
> 4. **DATA FLOW** — Trace the main data flows through this milestone's components. Does data flow end-to-end?
> 5. **COMPILATION** — Run `cargo check` to verify everything compiles together.
>
> Return structured findings as:
> ## Cross-Component Integration Review
> ### Critical Issues (must fix)
> ### Warnings (should fix)
> ### Observations
> ### Score: X/10

**Model:** opus (needs holistic systems thinking across component boundaries)

### Agent 4: Success Measure Verifier

**Prompt:**

> You are a milestone success measure verifier. This is the FINAL CHECK: does this milestone actually achieve what it was supposed to?
>
> For each success measure referenced by this milestone:
> 1. **STATE THE MEASURE** — Quote it from the intake document
> 2. **EVIDENCE** — What specific code, tests, or artifacts demonstrate it is met?
> 3. **VERDICT** — Met / Partially Met / Not Met
> 4. **GAP** — If not fully met, what specifically is missing?
>
> For each UAT criterion referenced by this milestone:
> 1. **STATE THE CRITERION**
> 2. **DEMO-ABILITY** — Could this criterion be demonstrated right now?
> 3. **VERDICT** — Demonstrable / Partially Demonstrable / Not Demonstrable
> 4. **GAP** — What would need to happen to make it demonstrable?
>
> Return structured findings as:
> ## Success Measure Verification
> ### Critical Issues (must fix)
> ### Warnings (should fix)
> ### Observations
> ### Score: X/10

**Model:** opus (needs to make judgment calls about whether evidence satisfies abstract criteria)

---

## Phase 4: Synthesize & Create Rework Tasks

### 4a: Synthesize Findings

Collect all 4 agent reports. Deduplicate findings — the same issue flagged from different angles (e.g., Agent 1 finds a failing test and Agent 3 finds the integration bug that causes it) becomes a single finding with the most severe rating.

Rank all findings by severity:

- **Critical** — Test failures, missing components, broken integration, unmet success measures. Must fix.
- **Warning** — Partial coverage, minor quality issues, edge cases not handled. Should fix.
- **Observation** — Style, optimization, nice-to-haves. Informational only.

### 4b: Create Rework Tasks (for Critical + Warning issues)

For each Critical or Warning issue that needs fixing, create a rework task via the CLI:

```bash
harnessx planning-tasks create \
  --title "REWORK: [specific fix description]" \
  --steps "[step 1 | step 2 | ...]" \
  --story "#[REWORK-STORY-ID]" \
  --epic "#[REWORK-EPIC-ID]" \
  --depends-on "#[THIS-REVIEW-TASK-ID]" \
  --complexity [appropriate level] \
  --mode rework \
  --skills "[appropriate specialist skill]" \
  --integration-tests "[specific tests that must pass after this fix]" \
  --note "Created by milestone review. Issue: [description]. Severity: [Critical/Warning]"
```

Each rework task must be specific enough for an agent to fix independently:
- Title clearly states what to fix
- Steps are concrete (not "investigate" — say what to do)
- Integration tests specify which tests must pass
- Dependencies are correct (depends on this review task)

### 4c: Create Final Verification Task

After all rework tasks are created, create a verification task that depends on ALL of them:

```bash
harnessx planning-tasks create \
  --title "VERIFY: Re-run all tests after rework" \
  --steps "Run cargo test -- --test-threads=1 | Run cargo test -- --ignored --test-threads=1 | Verify all tests pass | Report results" \
  --story "#[REWORK-STORY-ID]" \
  --epic "#[REWORK-EPIC-ID]" \
  --depends-on "[comma-separated list of ALL rework task IDs]" \
  --complexity low \
  --mode review \
  --skills "hx:milestone-rework-verification" \
  --note "Final verification after rework. Must pass before milestone can complete."
```

### 4d: If NO Issues Found

If all tests pass and all review agents report no Critical or Warning issues:
- Report: "Clean pass — no rework needed for [milestone title]. All tests pass, all success measures verified."
- Do NOT create any rework tasks
- The rework milestone will complete naturally with just this review task

---

## Phase 5: Write Report

Append to `harnessx/[PROJECT-ID]/history.md`:

```markdown
## Milestone Review: [main-milestone-id] — "[title]"
**Date**: [today]
**Test Results**: X unit tests passed, Y integration tests passed, Z failures
**Review Agents**: Test X/10, Quality X/10, Integration X/10, Success X/10
**Verdict**: Clean pass / N rework tasks created
**Rework Tasks**: [list task IDs and titles, or "None"]
**Files Affected**: [key files from review]
```

Read the existing file first and append — do not overwrite.

---

## Important Notes

### CLI-Only Task Creation

All rework tasks MUST be created via `harnessx planning-tasks create`. Never edit JSON files directly. The CLI ensures consistent ID generation, proper JSON structure, and active project resolution.

### Skill Assignment for Rework Tasks

Choose the appropriate specialist skill based on the issue type:

| Issue Type | Skill |
|---|---|
| Code bugs | `rust:developing` or `rust:team-coordinator` |
| Test issues | `rust:unit-testing` or `rust:integration-testing` |
| Error handling | `rust:errors-management` |
| Integration issues | `rust:team-coordinator` |
| Architecture issues | `rust:planning-and-architecture` |

### Rework Task Quality

Each rework task must be self-contained and actionable:

- **Title** clearly states what to fix (not "investigate issue" — say "fix missing error propagation in X")
- **Steps** are concrete implementation instructions
- **Integration tests** specify which tests must pass after the fix
- **Dependencies** are correct — every rework task depends on this review task
- **Note** references the review finding for traceability

### No User Gates

This skill runs on full autopilot. Do not ask the user for confirmation at any point. The review was triggered by the execution engine dispatching a review task — proceed through all phases without stopping.

### Test Failures Are Always Critical

Any test failure from Phase 2 is automatically a Critical severity finding. Tests are the objective truth — if they fail, something is broken and must be fixed before the milestone can pass.
