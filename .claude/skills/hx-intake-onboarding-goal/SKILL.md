---
name: hx:intake-onboarding-goal
description: Craft exceptionally well-written project goals from user input, then populate all project metadata fields (title, subtitle, description, user_name, takeaway_line, directory). Use this skill when the intake process reaches the "goal" section, when the user wants to define or refine a project goal, or when project metadata fields need to be filled in. Also trigger when the user says "set my goal", "what's my project about", "help me define this project", or asks to update project details like title, subtitle, or description.
disable-model-invocation: false
user-invocable: false
---

# Intake Goal

You help users transform rough ideas into sharp, well-structured project goals — then capture that clarity as project metadata.

This is a two-phase skill: **goal crafting** first, then **metadata population**.

---

## Phase 1: Craft the Goal

A great goal is the difference between a project that drifts and one that ships. Your job is to help the user articulate what they're building and why it matters — in a way that's specific enough to act on but concise enough to remember.

### What makes a goal well-written

A strong goal has five qualities:

1. **Specific outcome** — It names what will exist when the work is done. Not "improve the app" but "users can reset their password via email link without contacting support."

2. **Clear beneficiary** — It says who benefits and how. A goal without a beneficiary is a task list, not a goal. Even if the beneficiary is yourself ("I want to..."), make it explicit.

3. **Motivation** — It captures *why this matters now*. What's the pain, opportunity, or trigger? This is what keeps the project alive when things get hard. "Because users are churning at the password reset step" is stronger than "because it would be nice."

4. **Bounded scope** — It draws a line around what's in and what's out. A goal that could mean anything means nothing. If the user says "build a trading platform," help them narrow: which instruments? which users? MVP or full product?

5. **Testable completion** — You should be able to look at the finished work and say "yes, this goal is met" or "no, it isn't." Vague goals ("make it better") can never be completed. Concrete ones ("sub-200ms response time on the dashboard") can.

### How to draw the goal out

Don't interrogate. Have a conversation. Most people know what they want — they just haven't said it in a structured way yet.

Start by asking the user to describe their project in whatever way feels natural. Then listen for the five qualities above and gently probe for whatever's missing:

- If the outcome is vague: "When this is done, what will someone be able to do that they can't do today?"
- If the beneficiary is unclear: "Who's going to use this? What's their situation?"
- If motivation is missing: "What's driving this? Is there a deadline, a pain point, or an opportunity?"
- If scope is unbounded: "If you had to ship the smallest useful version of this, what would it include?"
- If completion is untestable: "How will you know when this is done? What does success look like concretely?"

Don't force all five into every goal — a personal weekend project doesn't need the same rigor as an enterprise platform. Match the depth to the complexity. But even simple goals benefit from being specific about what "done" looks like.

### Converge on the goal statement

Once you've gathered enough context, draft a goal statement. Keep it to 1-3 sentences. It should be something the user can read back in a month and immediately recall what the project is about and why they started it.

Present it to the user and ask: "Does this capture it?" Iterate until they confirm.

**Example of a weak goal:**
> Build a better dashboard.

**Example of a strong goal:**
> Give the ops team a real-time dashboard that surfaces the 5 metrics they actually check daily (error rate, p95 latency, active users, queue depth, deploy status), replacing the current workflow of SSH-ing into three different boxes. Ship an MVP they can use by end of sprint.

---

## Phase 2: Populate Project Metadata

Once the goal is confirmed, you have the context to fill in the project's metadata fields. These fields serve different audiences and purposes — don't just rephrase the goal six times.

Before filling these in, confirm the user's name and where the project will live on disk.

### The fields

| Field | Purpose | Guidance |
|---|---|---|
| **title** | The project's proper name. Shows up in listings and headers. | Short, memorable, noun-phrase. 2-5 words. Not a sentence. e.g., "Ops Dashboard", "Token Rotation Service" |
| **subtitle** | A one-line elevator pitch. Appears below the title. | Complete the sentence "This project..." without starting with "This project." One line, under 80 chars. e.g., "Real-time ops metrics at a glance" |
| **description** | The full context. Read by agents and future-you. | 2-4 sentences covering what it does, who it's for, and why it exists. This is where the goal statement lives, possibly expanded with technical context. |
| **user_name** | Who's driving the project. | The user's name or handle. Just ask them. |
| **takeaway_line** | The one thing someone should remember. | If someone reads nothing else, what should they walk away knowing? One punchy sentence. e.g., "Replaces manual SSH-based monitoring with a single-page dashboard." |
| **directory** | Where the project code lives on disk. | Absolute path. Ask the user — don't guess. If they're unsure, suggest a sensible default based on the current working directory. |

### How to fill them

Work through each field with the user. You already have most of the raw material from the goal conversation — now you're shaping it into the right format for each slot.

For **title** and **subtitle**, propose options and let the user pick or riff. For **description**, draft it from the confirmed goal and present it. For **user_name** and **directory**, just ask directly. For **takeaway_line**, distill the single most important thing about the project.

Present all six fields together for final confirmation before writing them.

### Writing the metadata

Once confirmed, update each field on the active project using the harnessx CLI:

```bash
harnessx project update-title "Project Title"
harnessx project update-subtitle "Short elevator pitch"
harnessx project update-description "Full description of the project."
harnessx project update-takeaway "The one thing to remember."
harnessx project update-directory "/absolute/path/to/project"
harnessx project update-username "their-name"
```

Run all six commands. Verify the final state:

```bash
harnessx project active
```

Show the user the completed project card and confirm everything looks right. If they want to adjust anything, update the specific field.

---

## Document the Discussion

Before marking this section complete, write a comprehensive markdown file that captures the full substance of the goal discussion. Get the active project ID:

```bash
harnessx project active
```

Then save the document to `harnessx/<project-id>/intake/goal.md`.

The document should include:

- **Date** of the discussion
- **The user's initial description** — how they first described the project in their own words
- **Questions asked and responses** — paraphrased, not verbatim transcripts, covering what was explored to refine the goal
- **Goal evolution** — how the goal statement developed from rough idea to final form, including any drafts that were revised
- **The confirmed goal statement** — the final 1-3 sentence goal the user approved
- **All project metadata fields** — the confirmed values for title, subtitle, description, user_name, takeaway_line, and directory
- **Key decisions and reasoning** — any choices made during the conversation (e.g., why a particular framing was chosen, what was deliberately excluded from the goal)
- **Action items created** during this section (titles and brief descriptions)

Write this as a readable narrative document, not a raw chat log. The goal is that any agent or person reading this file later gets the full picture of what was discussed and decided, without needing access to the original conversation.
