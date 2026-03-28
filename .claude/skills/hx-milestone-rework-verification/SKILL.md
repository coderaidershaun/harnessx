---
name: hx:milestone-rework-verification
description: >
  Lightweight final verification after milestone fix tasks complete. Runs all unit and integration
  tests sequentially to confirm fixes resolved the identified issues. If tests pass, sets review_status
  to passed and the milestone can be marked completed by the execution engine. If failures remain,
  creates one focused fix task for the specific failure. Use this skill when a verification task is
  dispatched during the milestone review cycle.
disable-model-invocation: false
user-invocable: false
---

# Milestone Rework Verification

This is the final checkpoint in a milestone's review cycle. All fix tasks have been executed. This skill runs all tests one more time to confirm everything passes. If they do, `review_status` is set to `"passed"` and the execution engine will mark the milestone completed on the next invocation. If not, a focused fix-and-reverify loop is created to converge on a clean state.

## Step 1: Setup

```bash
harnessx project active
```

Get the project directory from the response.

## Step 2: Run All Tests (sequentially)

```bash
cd [PROJECT-DIRECTORY]
cargo test -- --test-threads=1 2>&1
cargo test -- --ignored --test-threads=1 2>&1
```

Capture full output from both runs.

## Step 3: Evaluate Results

### ALL TESTS PASS

Find the milestone this task belongs to:

```bash
harnessx planning-tasks parent [THIS-TASK-ID]
```

Set the milestone review status to passed:

```bash
harnessx planning-milestones review [MILESTONE-ID] --status passed
```

Report: "All tests pass after rework. Milestone review verification complete."

The task will be marked completed by the execution engine. On the next invocation, the execution engine will see `review_status = "passed"` and mark the milestone completed.

### TESTS FAIL

For each failing test:
1. Read the test code
2. Read the code being tested
3. Identify the root cause

Create ONE focused fix task for the most critical remaining failure:

```bash
harnessx planning-tasks create \
  --milestone "#[MILESTONE-ID]" \
  --title "REWORK: Fix [specific test failure]" \
  --steps "[concrete fix steps]" \
  --depends-on "#[THIS-VERIFICATION-TASK-ID]" \
  --complexity [level] \
  --execution-order [high-number] \
  --mode rework \
  --skills "[appropriate skill]" \
  --integration-tests "[the failing test that must pass]" \
  --note "Created by verification. Test failure: [test name]. Error: [brief error]"
```

Then create ANOTHER verification task that depends on the fix:

```bash
harnessx planning-tasks create \
  --milestone "#[MILESTONE-ID]" \
  --title "VERIFY: Re-run tests after fix" \
  --steps "Run cargo test -- --test-threads=1 | Run cargo test -- --ignored --test-threads=1 | Verify all pass" \
  --depends-on "#[FIX-TASK-ID]" \
  --complexity low \
  --execution-order [high-number] \
  --mode review \
  --skills "hx:milestone-rework-verification" \
  --note "Re-verification after focused fix"
```

This creates a natural convergence loop — each cycle fixes one specific issue and re-verifies. The loop naturally terminates as issues are resolved.

### REWORK DEPTH LIMIT

Check this milestone's tasks. Count how many verification tasks already exist (tasks with title starting "VERIFY:"). If there are already 3 or more:

Report: "This milestone has been through 3+ rework cycles. The remaining failures may indicate a deeper issue. Creating the fix task but flagging for attention."

Still create the fix task, but add to the note: "WARNING: 3+ rework cycles. May need architectural review."

## Important Notes

- Run tests sequentially (`--test-threads=1`) to avoid flaky results from concurrent state
- The milestone ID can be found by tracing this task's parent: `harnessx planning-tasks parent [THIS-TASK-ID]` — this returns the milestone directly for v2 tasks
- Always create rework tasks via CLI, never edit JSON directly
- Keep fix tasks focused — one specific failure per task
