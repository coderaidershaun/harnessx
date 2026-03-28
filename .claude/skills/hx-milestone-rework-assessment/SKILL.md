---
name: hx:milestone-rework-assessment
description: >
  Autonomous milestone review that runs all tests, dispatches 2 review agents to assess completed
  work against success measures, and creates fix tasks via the harnessx CLI when issues are found.
  Assigned to the review task appended to a milestone after all its implementation tasks complete.
  Runs on full autopilot — no user gates. Use this skill when a milestone review task is dispatched
  during execution, or when the user says "review milestone", "assess milestone work", "run
  milestone review".
disable-model-invocation: false
user-invocable: false
---

# Milestone Rework Assessment

This is the skill assigned to the review task that the execution engine creates on a milestone after all its implementation tasks complete. It runs autonomously — no user gates, no confirmation prompts. Its job is to verify the milestone's completed work by running the full test suite and dispatching four specialist review agents, then creating targeted fix tasks via the harnessx CLI when issues are found.

The review task lives on the **same milestone** as the implementation tasks it reviews. Fix tasks and a verification task are also appended to the same milestone with higher `execution_order` values. The `review_status` field on the milestone tracks the review lifecycle.

---

## Phase 1: Setup

### 1. Get Active Project

```bash
harnessx project active
```

Capture the project ID, directory, and metadata.

### 2. Find the Milestone Being Reviewed

Get the parent milestone directly from this task:

```bash
harnessx planning-tasks parent [THIS-TASK-ID]
```

This returns the milestone this review task belongs to — the **same milestone** whose implementation tasks are being reviewed.

Capture the milestone's title, success_measures, uat_criteria, and all metadata.

```bash
harnessx planning-milestones children [MILESTONE-ID]
```

This returns all tasks under this milestone. Categorize them:
- **Completed implementation tasks** — tasks with `completed` status whose group is NOT `"review"`. These are the work being reviewed.
- **This review task** — the task you're running as. Skip it in your analysis.
- **Any prior rework tasks** — if this is a re-review after a rework cycle, note what was already fixed (tasks with `"REWORK:"` or `"VERIFY:"` title prefixes).

### 3. Load Intake Documents

Read from `harnessx/[PROJECT-ID]/intake/`:

- `success_measures.md`
- `user_acceptance_testing.md`
- `scope.md`

### 4. Capture Key References

Before proceeding, confirm you have:

- **Milestone's success measures** — from the milestone object and intake docs
- **UAT criteria** — from the intake docs
- **All completed implementation tasks with their notes** — execution summaries from the agents that did the work
- **This milestone's ID** — fix tasks will be appended to the same milestone
- **This review task's ID** — fix tasks will use this as their `depends_on`
- **Current max execution_order** — from the milestone's children, needed for ordering fix tasks

---

## Phase 2: Run All Tests

Run the test suites directly — no agent needed for this.

```bash
cd [PROJECT-DIRECTORY]
cargo test -- --test-threads=1 2>&1
cargo test -- --ignored --test-threads=1 2>&1
```

Capture the full output from both commands. Note total tests run, passed, failed, and the details of any failures.

Test failures are always Critical severity.

---

## Phase 3: Dispatch 2 Review Agents (parallel)

Both agents receive the same base context:
- The milestone and its completed implementation tasks (with their notes/execution summaries)
- The test results from Phase 2
- The relevant intake documents (success_measures.md, user_acceptance_testing.md, scope.md)

Launch both agents in parallel.

**IMPORTANT: Do NOT set `run_in_background: true`.** Both agents must run in foreground — their results are needed before the next phase can proceed.

### Agent 1: Tests, Coverage & Success Measures

**Prompt:**

> You are reviewing completed milestone work against its success criteria. You have the test results, the task definitions (with their integration_tests fields), and the intake documents.
>
> Analyse:
> 1. **TEST COVERAGE** — Are all integration tests defined in the tasks actually present in the codebase? List any defined-but-missing tests.
> 2. **PASS/FAIL ANALYSIS** — For each test failure, determine: is this a code bug, a test environment issue, or a test definition problem?
> 3. **SUCCESS MEASURES** — For each milestone success measure, what evidence (passing tests, code, artifacts) demonstrates it is met? Verdict: Met / Partially Met / Not Met.
> 4. **UAT CRITERIA** — For each UAT criterion, could it be demonstrated right now? What's missing if not?
> 5. **GAPS** — Any success measures or UAT criteria with no test coverage or evidence at all?
>
> Return structured findings as:
> ## Tests & Success Measures Review
> ### Critical Issues (must fix)
> ### Warnings (should fix)
> ### Observations
> ### Score: X/10

**Model:** opus

### Agent 2: Code Quality & Integration

**Prompt:**

> You are reviewing code quality and cross-component integration for completed milestone work. Agents executed tasks independently — your job is to verify the code is complete, correct, and integrates properly.
>
> Analyse:
> 1. **OUTPUT FILES EXIST** — Do the files listed in traces.output_sources actually exist? Read them.
> 2. **CODE QUALITY** — Read the key files produced. Look for: unwrap() calls that should be handled, TODO/FIXME comments left behind, placeholder implementations, missing error handling. Focus on substance, not style.
> 3. **INTERFACE COMPATIBILITY** — Do components built by different tasks use compatible types, function signatures, and data formats?
> 4. **DATA FLOW** — Trace the main data flows through this milestone's components. Does data flow end-to-end?
> 5. **COMPILATION** — Run `cargo check` to verify everything compiles together.
>
> Return structured findings as:
> ## Code Quality & Integration Review
> ### Critical Issues (must fix)
> ### Warnings (should fix)
> ### Observations
> ### Score: X/10

**Model:** opus

---

## Phase 4: Synthesize & Create Rework Tasks

### 4a: Synthesize Findings

Collect both agent reports. Deduplicate findings — the same issue flagged from different angles becomes a single finding with the most severe rating.

Rank all findings by severity:

- **Critical** — Test failures, missing components, broken integration, unmet success measures. Must fix.
- **Warning** — Partial coverage, minor quality issues, edge cases not handled. Should fix.
- **Observation** — Style, optimization, nice-to-haves. Informational only.

### 4b: Create Fix Tasks (for Critical + Warning issues)

**Batch related issues.** If multiple issues affect the same file or module, combine them into a single fix task. Don't create one task per issue — group by what an agent would naturally fix together.

For each fix task, use `execution_order` values starting from the current max + 1. Within the same milestone, `execution_order` IS the dependency — no `--depends-on` needed.

```bash
harnessx planning-tasks create \
  --milestone "#[MILESTONE-ID]" \
  --title "REWORK: [specific fix description]" \
  --steps "[step 1 | step 2 | ...]" \
  --complexity [appropriate level] \
  --execution-order [current-max + 1] \
  --mode rework \
  --skills "[appropriate specialist skill]" \
  --integration-tests "[specific tests that must pass after this fix]" \
  --note "Created by milestone review. Issue: [description]. Severity: [Critical/Warning]"
```

Each fix task must be specific enough for an agent to fix independently:
- Title clearly states what to fix
- Steps are concrete (not "investigate" — say what to do)
- Integration tests specify which tests must pass

### 4c: Create Final Verification Task

After all fix tasks are created, create a verification task with the highest `execution_order`:

```bash
harnessx planning-tasks create \
  --milestone "#[MILESTONE-ID]" \
  --title "VERIFY: Re-run all tests after rework" \
  --steps "Run cargo test -- --test-threads=1 | Run cargo test -- --ignored --test-threads=1 | Verify all tests pass | Report results" \
  --complexity low \
  --execution-order [highest-number] \
  --mode review \
  --skills "hx:milestone-rework-verification" \
  --note "Final verification after rework. Must pass before milestone can complete."
```

### 4d: Set Review Status

After creating rework tasks, set the milestone review status to rework:

```bash
harnessx planning-milestones review [MILESTONE-ID] --status rework
```

### 4e: If NO Issues Found

If all tests pass and all review agents report no Critical or Warning issues:
- Report: "Clean pass — no rework needed for [milestone title]. All tests pass, all success measures verified."
- Do NOT create any rework tasks
- Set the milestone review status to passed:

```bash
harnessx planning-milestones review [MILESTONE-ID] --status passed
```

- The execution engine will see `review_status = "passed"` on the next invocation and mark the milestone completed

---

## Phase 5: Write Report

Append to `harnessx/[PROJECT-ID]/history.md`:

```markdown
## Milestone Review: [milestone-id] — "[title]"
**Date**: [today]
**Test Results**: X unit tests passed, Y integration tests passed, Z failures
**Review Agents**: Tests & Success X/10, Code & Integration X/10
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
