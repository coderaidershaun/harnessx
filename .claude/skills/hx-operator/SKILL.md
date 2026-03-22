---
name: hx:operator
description: The harnessx project operator — the main entry point for all project workflows. Use this skill when the user runs /hx:operator, wants to start or resume a project, check what's next, or get routed to the right skill. Trigger this whenever the user says "start project", "what's next", "resume work", or anything related to project orchestration and workflow routing.
disable-model-invocation: true
user-invocable: false
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

The `data` field contains the project object. Before checking pipeline progress, inspect the project for missing metadata.

### Check for incomplete fields

Look at these fields in the project `data`: `title`, `subtitle`, `description`, `takeaway_line`, `directory`, `user_name`. Any field that is an empty string `""` is incomplete.

If **all fields are populated** (non-empty), skip ahead to "Check pipeline progress" below.

If **any fields are empty**, tell the user which fields are missing and work through them conversationally — the same way the intake goal skill would. The goal is to not let a half-filled project slip through into the pipeline.

For each missing field, ask the user for the value. Use these as guidance:

| Field | What to ask |
|---|---|
| `title` | "What should we call this project? A short name, 2-5 words." |
| `subtitle` | "Give me a one-line elevator pitch — what does this project do?" |
| `description` | "Can you describe the project in 2-4 sentences? What it does, who it's for, and why it exists." |
| `takeaway_line` | "If someone reads one sentence about this project, what should they remember?" |
| `directory` | "Where does the project code live on disk? Give me an absolute path." |
| `user_name` | "What's your name?" |

You can gather multiple fields in a single conversational turn if natural — don't force one question per message. Once you have the values, update each field:

```bash
harnessx project update-title "Project Title"
harnessx project update-subtitle "Short elevator pitch"
harnessx project update-description "Full description."
harnessx project update-takeaway "The one thing to remember."
harnessx project update-directory "/absolute/path"
harnessx project update-username "their-name"
```

Only run the update commands for fields that were actually missing. Verify with `harnessx project active` and show the user the completed project card before continuing.

### Check pipeline progress

Now find out what's next:

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
- If `skill` starts with `rust:`: These can optionally be delegated to a subagent for autonomous execution, or invoked directly.

Run `/compact` before invoking the skill to free up context, then invoke the skill and stop.
