---
name: hx:intake-completion
description: Guide the intake completion process through its three sections — exploration, ideation, and project risk management — by looping through each section, loading the appropriate specialist skill, and marking progress. Use this skill when the pipeline reaches the intake_completion stage, when the operator routes to intake_completion, or when the user says "start intake completion", "run exploration and ideation", "continue intake completion". Also trigger after intake_team is complete and the pipeline advances to the next stage.
---

# Intake Completion

You orchestrate the three intake completion sections — exploration, ideation, and project risk management — in sequence. Each section has its own specialist skill that does the actual work. Your job is to loop through them, load the right skill, and track progress.

This is the same pattern as `hx:intake-onboarding` and `hx:intake-team` — a loop that calls `next`, loads skills, executes, marks complete, and advances.

## Startup

Confirm there is an active project:

```bash
harnessx project active
```

If no active project exists, tell the user to run `/hx:operator` first and stop.

Then mark the intake completion stage as in-progress:

```bash
harnessx progress update intake_completion in_progress
```

Initialize the intake completion tracking if it hasn't been done yet:

```bash
harnessx intake-completion init
```

(If it already exists, this will return an error — that's fine, move on.)

## Step 1: Get the current section

```bash
harnessx intake-completion next
```

This returns JSON like:

```json
{
  "success": true,
  "data": {
    "section": "exploration",
    "skills": ["hx:intake-completion-exploration"]
  }
}
```

If `success` is `false` or there's no next section, all intake completion sections are done — skip to the Final step.

Mark the section as in-progress:

```bash
harnessx intake-completion update <section> in_progress
```

## Step 2: Load required skills

Check the `skills` array in the response. Read each skill's instructions from `.claude/skills/<skill-name>/SKILL.md` (where `<skill-name>` uses the colon-to-hyphen convention — e.g., `hx:intake-completion-exploration` lives at `.claude/skills/hx-intake-completion-exploration/SKILL.md`).

Also read the CLI reference at `docs/intake-completion.md` for the exact commands available.

## Step 3: Load the actions writing skill

Read `.claude/skills/hx-intake-actions-writing/SKILL.md`. Every intake completion section creates action items — this skill is the authority on how to structure them, tag them, and link them bidirectionally.

Also read `docs/intake-actions.md` for the exact CLI flags and field types.

## Step 4: Execute the section

Follow the specialist skill's instructions. Each section operates differently:

### Section: exploration

The `hx:intake-completion-exploration` skill dispatches multi-agents to deeply explore all resources collected during intake onboarding. It writes comprehensive notes to `harnessx/<project-id>/intake/exploration/` and creates action items from findings.

This section runs autonomously — the specialist skill handles agent dispatch, note collection, summary writing, and action item creation. Let it do its work.

### Section: ideation

The `hx:intake-completion-ideation` skill dispatches creative multi-agents with different thinking lenses (Architect, User Advocate, Pragmatist, Domain Expert, Risk Spotter) to generate ideas that elevate the project within scope. It produces an ideation report and creates action items for the best ideas.

This section also runs autonomously via multi-agent dispatch.

### Section: project_risk_manager

The `hx:intake-completion-project-risk` skill dispatches risk auditor agents to review all intake documents and existing action items, identifying gaps that would lead to poor quality output. It catches the things LLMs typically miss — concurrency plans, error handling strategies, integration assumptions, etc.

This section runs autonomously via multi-agent dispatch.

## Step 5: Complete the section

After the specialist skill finishes its work (notes written, action items created, tags placed), mark the section done:

```bash
harnessx intake-completion complete <section>
```

Give the user a brief summary of what was accomplished in this section.

## Step 6: Loop — advance to the next section

After completing a section, immediately run `harnessx intake-completion next` again. If there is another section:

1. Mark it as in-progress: `harnessx intake-completion update <section> in_progress`
2. Load any skills listed in the response's `skills` array (same as Step 2)
3. Execute the section (Step 4)
4. Mark it complete and loop back here

Continue until `harnessx intake-completion next` returns no remaining sections.

## Final: Mark the pipeline stage complete

When all three sections are done, the CLI automatically marks the `intake_completion` pipeline stage as complete. Verify this happened:

```bash
harnessx progress status
```

Tell the user the full intake completion is done — exploration notes are written, ideas have been captured, and risks have been audited. The project is ready for planning.

Then tell the user to re-enter via `/hx:operator` to continue to the next pipeline stage.

**STOP — do not continue to the next stage yourself.** The operator skill handles stage routing.
