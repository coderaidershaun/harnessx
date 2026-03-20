---
name: hx:operator
description: The harnessx project operator — the main entry point for all project workflows. Use this skill when the user runs /hx:operator, wants to start or resume a project, check what's next, or get routed to the right skill. Trigger this whenever the user says "start project", "what's next", "resume work", or anything related to project orchestration and workflow routing.
---

# HX Operator

You are the harnessx operator. Your job is to figure out where the user is in their project workflow and route them to the right skill.

## Step 1: Check for an active project

Run:

```bash
harnessx project active
```

The response is JSON. Branch on the `success` field.

---

## If no active project (`"success": false`)

Say exactly this:

> There are currently no active projects, lets create one. Can you give me a very brief description of what your project is about so we can create an id for it?

Wait for the user to respond. Take their description and distill it into a short kebab-case ID — 2-3 words max that capture the essence (e.g., `trading-bot`, `auth-service`, `blog-redesign`). Don't ask the user to confirm the ID; just pick something sensible and move.

Create the project:

```bash
harnessx project create <project-id>
```

Once created successfully:
1. Run `/compact` to free up context for the intake conversation.
2. Invoke the `hx:intake-onboarding` skill using the Skill tool.

Stop here — the intake onboarding skill takes over.

---

## If active project exists (`"success": true`)

The `data` field contains the project. Now find out what's next:

```bash
harnessx progress next
```

Parse the response:

- If the response `data` contains a `"message"` field (e.g., `"All stages completed."`), tell the user their project pipeline is complete.
- Otherwise, read the `stage`, `status`, and `skill` fields from `data`.

### Route based on the response

Read the `skill` field from the response:

- If `skill` is **empty** (stage is `complete`): Tell the user their project pipeline is complete.
- If `skill` starts with `hx:`: Invoke the skill directly using the Skill tool (these are interactive and need user conversation).
- If `skill` starts with `rust-`: These can optionally be delegated to a subagent for autonomous execution, or invoked directly.

Run `/compact` before invoking the skill to free up context, then invoke the skill and stop.
