---
name: hx:user-acceptance
description: >
  Guide the user through user acceptance testing against criteria defined during intake onboarding.
  Loads UAT scenarios, success measures, and sign-off criteria, then walks the user through each
  test scenario to collect structured feedback. If approved, advances the pipeline to completion.
  If rework is needed, captures structured feedback and routes to the uat_rework stage. Use this
  skill when the pipeline reaches user_acceptance, when the user says "test the project", "run UAT",
  "acceptance testing", "sign off", or when the operator routes to user_acceptance after execution
  completes.
disable-model-invocation: false
user-invocable: true
---

# User Acceptance Testing

This is the user's moment to evaluate what was built. Walk them through every UAT scenario defined during intake, collect structured verdicts, and route the pipeline based on their decision. This skill is interactive — talk to the user throughout, present results clearly, and probe for specifics when feedback is vague.

The pipeline state machine relies on two stages working together: `user_acceptance` (this skill) and `uat_rework`. This skill always marks `user_acceptance` complete when done. What varies is whether `uat_rework` gets marked complete (approved) or left as `not_started` (rework needed). This drives the pipeline routing.

---

## Phase 1: Load Context

### 1a. Get active project

```bash
harnessx project active
```

Capture the project ID, directory, and `user_name` from the response.

### 1b. Load UAT artifacts

Read these files in parallel from `harnessx/<project-id>/`:

- `intake/user_acceptance_testing.md` — the UAT plan from intake (test scenarios, sign-off criteria, deliverables)
- `intake/success_measures.md` — success measures the project must satisfy
- `history.md` — execution history showing what was built and when

```bash
harnessx planning-milestones list
```

Capture the milestone list to understand what was completed.

### 1c. Detect rework round

Check if `harnessx/<project-id>/uat_feedback.md` exists:

- **If it exists:** This is a re-test after a rework round. Read it to understand what was previously flagged and what round this is. The new round number = previous round + 1.
- **If it does not exist:** This is the first UAT pass. Round = 1.

### 1d. Mark stage in progress

```bash
harnessx progress update user_acceptance in_progress
```

---

## Phase 2: Present UAT Overview

Greet the user by name.

**If this is a re-test** (uat_feedback.md existed):

> "This is round N of UAT testing. In the previous round, you identified these issues: [summarize the key FAIL/PARTIAL items from the previous feedback]. The rework has been completed. Let's verify the fixes and re-test the full criteria."

**For all rounds**, present:

1. **What was built** — summarize completed milestones and key accomplishments from history.md. Keep it concise — the user knows what they asked for; focus on what was delivered.

2. **Test scenarios** — list every test scenario from `user_acceptance_testing.md`, organized by category:
   - Live demo scenarios
   - Hands-on testing scenarios
   - Evidence and artifact checks
   - Edge cases and failure modes

3. **Sign-off criteria** — what constitutes a pass, as defined during intake.

Tell the user: "We'll go through each scenario one at a time. For each one, I'll describe what to test, and you tell me whether it passes, fails, or partially works."

---

## Phase 3: Walk Through Test Scenarios

For each test scenario defined in `user_acceptance_testing.md`:

1. **Present the scenario:**
   - What to test (the specific action or verification)
   - Expected result (what success looks like)
   - Linked success measure (if referenced in the UAT plan)

2. **Ask the user to test it** — they may need to run the app, check output files, verify behavior, etc. Give them time. If they say they've already tested it, accept their assessment.

3. **Collect their verdict:** PASS / FAIL / PARTIAL

4. **If FAIL or PARTIAL**, collect specifics:
   - What did they expect to see?
   - What did they actually observe?
   - How severe is this? (critical / important / minor)
   - Any additional context about what went wrong?

5. **Track all results** internally as you go — you'll need them for the summary and feedback file.

**Pacing:** Don't rush. Group related scenarios if the user prefers ("I tested all three of these already, they all pass"), but make sure every scenario gets a verdict. If the user wants to skip a scenario, mark it as PARTIAL with a note explaining why it was skipped.

---

## Phase 4: Collect Deliverable Verification

Walk through each required deliverable mentioned in the UAT plan (documentation, test results, benchmarks, recordings, artifacts, etc.):

- Present the deliverable requirement
- Ask if it was provided and is satisfactory
- If missing or incomplete, note the gap with specific feedback

---

## Phase 5: Overall Verdict

Present a summary table:

```
Scenarios: X passed, Y failed, Z partial (out of N total)
Deliverables: A complete, B missing/incomplete
```

List any FAIL or PARTIAL items with their severity.

Ask the user for their overall verdict:

- **APPROVED** — the project meets acceptance criteria and is ready to ship
- **REWORK NEEDED** — specific items need fixing before sign-off

**If the user leans toward rework but their feedback is vague**, probe for specifics:

> "Before we enter a rework cycle, I need concrete feedback the agents can act on. Can you point to a specific scenario that didn't meet your expectations? What exactly did you observe vs. what you expected?"

If the user genuinely cannot articulate specific feedback, suggest they approve with a note rather than entering a rework cycle with nothing actionable. A rework cycle without clear feedback wastes execution time and comes back to UAT with nothing obviously different.

---

## Phase 6a: If APPROVED

Mark both stages complete:

```bash
harnessx progress complete user_acceptance
harnessx progress complete uat_rework
```

Read `harnessx/<project-id>/history.md` first, then append:

```markdown
## UAT Approved
**Date:** [today]
**Round:** [N]
**Scenarios:** X passed / Y total
**Verdict:** APPROVED
```

Tell the user: "The project has passed user acceptance testing. The pipeline will advance to the complete stage."

---

## Phase 6b: If REWORK NEEDED

### Write structured feedback

Write to `harnessx/<project-id>/uat_feedback.md` (overwrite — each round gets a fresh file, but the round number increments):

```markdown
# UAT Feedback — Round N

**Date:** [today's date]
**Round:** N
**Scenarios Passed:** X / Y total

---

## Failed Scenarios

### Scenario: [scenario title from UAT plan]
**Linked Success Measure:** [reference if available]
**Verdict:** FAIL
**Expected:** [what the user expected to see]
**Actual:** [what the user actually observed]
**User Feedback:** [their specific description of what's wrong]
**Severity:** critical | important | minor

---

## Partial Scenarios

### Scenario: [scenario title]
**Linked Success Measure:** [reference if available]
**Verdict:** PARTIAL
**What Works:** [what passed]
**What Doesn't:** [what's missing or broken]
**User Feedback:** [specific description]
**Severity:** critical | important | minor

---

## Passed Scenarios (for regression awareness)

- [scenario title] — PASS

---

## User's Priority Notes

[Any additional context, priority ordering, or notes the user provided]
```

### Update pipeline state

```bash
harnessx progress update uat_rework not_started
harnessx progress complete user_acceptance
```

The `uat_rework` reset is idempotent — it works correctly whether this is the first round (already not_started) or a subsequent round (was completed from a prior rework cycle).

### Append to history

Read `harnessx/<project-id>/history.md` first, then append:

```markdown
## UAT Review — Round N
**Date:** [today]
**Scenarios:** X passed, Y failed, Z partial / N total
**Verdict:** REWORK NEEDED
**Key Issues:** [1-2 sentence summary of the most important issues]
```

### Report to user

Tell the user: "I've captured your feedback. The pipeline will now create a rework plan and re-execute the fixes. Run `/hx:operator` in a new session to continue."

---

## Important Notes

### CLI-only state changes

All pipeline state changes go through the harnessx CLI. Never edit progress.json or other JSON files directly.

### History file is append-only

Always read `history.md` before appending. Never overwrite existing content.

### Feedback file is overwritten each round

`uat_feedback.md` is overwritten — not appended — each round. This is intentional: the uat_rework skill needs a clean, current view of what's wrong. Historical feedback is preserved in `history.md`.

### Require actionable feedback for rework

Do not proceed with rework if the user has no specific feedback. A rework cycle with vague or empty feedback wastes execution resources and produces no meaningful improvement. Probe for specifics, and if none emerge, recommend approval with notes.
