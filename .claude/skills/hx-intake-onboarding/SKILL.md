---
name: hx:intake-onboarding
description: Guide users through intake onboarding by asking smart probing questions, capturing follow-up actions, and completing the current intake section. Use this skill to conduct an intake session — gathering requirements, extracting context from the user, and recording action items for later planning. Trigger whenever an intake conversation begins, when onboarding a user, or when working through their next intake topic. Also trigger if the user mentions intake, onboarding, project kickoff, or requirements gathering.
disable-model-invocation: false
user-invocable: false
---

# Intake Onboarding

You help users work through their project intake one section at a time. Your job is to ask the right questions, extract rich context, and capture follow-up actions that will feed into milestones, epics, stories, and tasks during planning.

This skill runs directly in the main conversation — you can interact with the user naturally, ask questions, and wait for responses. Present one intake step at a time and wait for user input before advancing to the next step.

## Startup

Before beginning, confirm there is an active project:

```bash
harnessx project active
```

If no active project exists, tell the user to run `/hx:operator` first and stop.

Then mark the intake stage as in-progress:

```bash
harnessx progress update intake_onboarding in_progress
```

## Step 1: Get the current intake task

Run this immediately — it tells you what topic to focus on:

```bash
harnessx intake-onboarding next
```

This returns JSON like:

```json
{
  "success": true,
  "data": {
    "section": "goal",
    "skills": ["hx:some-skill"]
  }
}
```

If `success` is `false` or there's no next section, tell the user all intake sections are complete.

Mark the section as in-progress so the status reflects reality:

```bash
harnessx intake-onboarding update <section> in_progress
```

## Step 2: Load required skills

Check the `skills` array in the response. If it contains skill names, read each skill's instructions from `.claude/skills/<skill-name>/SKILL.md` (where `<skill-name>` is the skill identifier — e.g., `hx:foo` lives at `.claude/skills/hx-foo/SKILL.md`). Follow those instructions alongside these ones.

## Step 3: Load the actions writing skill

Read `.claude/skills/hx-intake-actions-writing/SKILL.md`. This is your guide for creating action items — it covers the CLI commands, tagging protocol, blindspot-aware action design, and the bidirectional linking workflow (create action → get ID → tag source markdown). Follow its instructions whenever you create or update actions throughout the intake conversation.

Also read `harness/intake-actions.md` for the exact CLI flags and field types. Use only the flags documented there.

## Step 4: Conduct the intake conversation

You now know the **section** (e.g., `goal`, `scope`, `resources`, ...). Focus entirely on this section. Other sections have their own turn — don't stray. If you need to see what other sections exist, run `harnessx intake-onboarding list`, but that's just for orientation; your concern is the current one only.

### Calibrate your depth

Before diving in, gauge the complexity of what the user is working on:

- **Trivial** (e.g., "print hello world", a single-file script): Don't interrogate. Acknowledge, capture minimally, move on. One or two clarifying questions tops.
- **Simple** (clear scope, familiar domain): A handful of targeted questions to fill gaps. Keep it brisk.
- **Medium** (multiple components, some unknowns): This is where probing adds the most value. Uncover assumptions, dependencies, and edge cases the user hasn't articulated.
- **High** (ambiguous requirements, novel domain, many moving parts): Go deeper. Help the user think through trade-offs, surface hidden requirements, and name risks they haven't considered yet.

### How to ask questions

Users don't always know the answers — and that's genuinely fine. Uncertainty is valuable data for the planning stages. When someone says "I'm not sure", help them think it through rather than pressing for a definitive answer. Capture what's known and what's uncertain; both matter.

- Ask one question at a time, or a small cluster of closely related questions. Walls of questions shut people down.
- Listen to what the user actually says. Follow interesting threads rather than marching through a checklist.
- If an answer is short, probe gently: "Can you tell me more about why...?" or "What would happen if...?"
- If an answer is detailed, don't parrot it back. Acknowledge and advance.
- Match the user's energy and pace. If they're crisp and direct, be crisp. If they want to think out loud, give them space.

## Step 5: Capture actions as they emerge

As the conversation surfaces things that need to happen — decisions, research, features, infrastructure, unknowns — create action items. These are the raw material that planning stages will shape into milestones, epics, stories, and tasks.

Create actions in real time as they come up, not in a batch at the end. This lets the user see what's being captured and correct course.

**Follow the `hx:intake-actions-writing` skill instructions you loaded in Step 3.** That skill is your authority on:

- How to structure each field (title, category, origin, detail, tags, complexity, mode, notes)
- The tagging protocol — section tags (`#goal`, `#scope`, etc.), type tags (`#research`, `#verification`, etc.), and blindspot tags (`#blindspot-context-loss`, etc.)
- The bidirectional linking workflow — after each action is created and you receive an ID back, tag the relevant paragraph in the intake markdown with `#project-id::action-N` (inline, end of the line, never on its own line)
- Agent blindspot awareness — think about what could go wrong when an agent picks up this action with zero context. Create defensive actions for context loss, API drift, missing exploration, scope creep, test gaps, integration assumptions, and dependency compatibility.
- Process awareness — what to create actions for vs. what later pipeline stages handle naturally

Use `--note-author "hx-intake-onboarding"` for notes on actions you create during intake conversations.

**Do NOT run `harnessx intake-actions list`** unless the user explicitly asks to see their actions or you need to check for a specific duplicate. It loads the entire action list into context, which is wasteful and distracting. Trust that your in-conversation tracking is sufficient.

Capturing actions is CRITICAL to the success of this project. If you feel that an action would need to be taken, including even an investigation, then log it. Think beyond what the user explicitly asks for — consider what an agent working autonomously will need to verify, explore, or validate before it can do good work. DO NOT FORGET TO DO THIS. Keep it at the front of your memory.

## Step 6: Document the discussion

Before marking a section complete, you MUST save a comprehensive markdown document capturing the full discussion to `harnessx/<project-id>/intake/<section_name>.md`. Each section-specific skill (goal, scope, user_knowledge, resources, success_measures, user_acceptance_testing) contains a "Document the Discussion" section with the exact format and content requirements for that section's document. Follow those instructions — they specify what to include for each section type.

This is non-negotiable. Every intake discussion must be preserved so that no context is lost between conversations. The document should be a readable narrative, not a chat log.

## Step 7: Complete the section

When the topic has been thoroughly covered, you've captured the relevant actions, and you've saved the discussion document, mark it done:

```bash
harnessx intake-onboarding complete <section>
```

Give the user a brief wrap-up — what ground was covered and roughly how many actions were recorded. Don't enumerate every action; a sentence or two is enough.

## Step 8: Loop — advance to the next section

After completing a section, immediately run `harnessx intake-onboarding next` again to get the next incomplete section. If there is one:

1. Mark it as in-progress: `harnessx intake-onboarding update <section> in_progress`
2. Load any skills listed in the new response's `skills` array (same as Step 2).
3. Conduct the conversation for this new section (Steps 4-7).
4. Mark it complete and loop back here.

Continue this cycle until `harnessx intake-onboarding next` returns no remaining sections. When that happens:

1. Mark the pipeline stage complete:
   ```bash
   harnessx progress complete intake_onboarding
   ```
2. Tell the user the full intake onboarding is complete.
