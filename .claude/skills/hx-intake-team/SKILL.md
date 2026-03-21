---
name: hx:intake-team
description: Define and build the team of specialist agent skills required to complete a harnessx project, then interview each agent. Reviews all intake documents and action items, catalogs available skills, recommends team composition (existing skills + new skills needed), discusses with the user, builds missing skills using /find-skills and /skill-creator, and conducts agent interviews. Handles all three intake-team sections (team_define, team_build, team_interview) in a loop. Use this skill when the pipeline reaches intake_team, when the user says "define the team", "what skills do I need", "build my team", "assemble the agents", or anything about determining which agent skills a project requires. Also trigger when the operator routes to the intake_team stage.
disable-model-invocation: false
user-invocable: false
---

# Intake Team

You guide the project through team definition, team building, and agent interviews — one section at a time. Your job is to figure out what specialist skills the project needs, build any that are missing, and then interview each agent before execution begins.

The core insight driving this skill: **skills are not subagents**. Skills are markdown instruction sets that teach Claude how to do specific work. They get assigned to agents at execution time. Your job is to figure out which skill instruction sets the project needs, check what already exists, and create what's missing.

This skill runs directly in the main conversation — you interact with the user naturally, ask questions, and wait for responses. Present one section at a time and wait for user input before advancing.

---

## Startup

Before beginning, confirm there is an active project:

```bash
harnessx project active
```

If no active project exists, tell the user to run `/hx:operator` first and stop.

Then mark the intake_team stage as in-progress:

```bash
harnessx progress update intake_team in_progress
```

## Step 1: Get the current intake team section

Run this immediately — it tells you what section to focus on:

```bash
harnessx intake-team next
```

This returns JSON like:

```json
{
  "success": true,
  "data": {
    "section": "team_define",
    "skills": ["hx:intake-team"]
  }
}
```

If `success` is `false` or there's no next section, tell the user all intake team sections are complete.

Mark the section as in-progress so the status reflects reality:

```bash
harnessx intake-team update <section> in_progress
```

## Step 2: Load required skills

Check the `skills` array in the response. If it contains skill names, read each skill's instructions from `.claude/skills/<skill-name>/SKILL.md` (where `<skill-name>` replaces `:` with `-` — e.g., `hx:intake-team` lives at `.claude/skills/hx-intake-team/SKILL.md`). Follow those instructions alongside these ones.

For `team_define` and `team_build`, the skill points back to this file — follow the section-specific instructions below.

For `team_interview`, the skill points to `hx:intake-team-interviewing` — read that skill's SKILL.md and follow its interview protocol.

## Step 3: Execute the current section

Based on the section name from Step 1, follow the corresponding instructions:

---

### Section: team_define

The goal is to understand what the project needs, map those needs to skills, identify gaps, and agree on a team composition with the user.

#### 3a. Gather all project context

Read every intake document and the action items. This gives you the full picture.

**Read all markdown files in the intake folder:**

```bash
ls harnessx/<project-id>/intake/
```

Read each `.md` file found — goal, scope, user_knowledge, resources, success_measures, user_acceptance_testing, and any interview documents.

**Read the action items:**

```bash
harnessx intake-actions list
```

**Read the project metadata:**

```bash
harnessx project active
```

Pay attention to the project description, directory, and any context about the technology stack, domain, and complexity.

As you read, build a mental model of:

- **What domains the project touches** — backend, frontend, data pipeline, infrastructure, ML/AI, blockchain, mobile, etc.
- **What technologies are specified** — languages (Rust, Python, TypeScript, Go...), frameworks (SolidJS, React, Actix, FastAPI...), platforms (AWS, Solana, Ethereum...)
- **What types of work are needed** — new development, refactoring, research, data analysis, UI/UX, testing, deployment
- **What complexity level** — a single script, a multi-module system, a distributed architecture
- **What the user explicitly mentioned** — any stated preferences about tools, approaches, or skill requirements

#### 3b. Catalog all available skills

List every skill currently installed:

```bash
ls .claude/skills/
```

For each skill directory, read its `SKILL.md` frontmatter (the `name` and `description` fields). Build a catalog of what's already available, organized by domain:

- **hx:*** — project orchestration skills (intake, operator, troubleshooting)
- **rust:*** — Rust development team (exploration, architecture, developing, testing, refactoring, commenting, errors, coordination)
- **Any other prefixes** — other domain teams that may already exist

#### 3c. Analyze project needs and map to skills

Synthesize what you learned from the intake documents with what skills exist. Think in layers:

**Layer 1: What existing skills directly apply?**

Look at the project's technology stack and required work types. If the project involves Rust development, the entire `rust:*` team is immediately relevant. Map each major work area to existing skills.

**Layer 2: What gaps exist?**

For each domain the project touches that doesn't have a matching skill team, note the gap. Be specific — not "we need a research skill" but "we need a skill that can analyze on-chain liquidity pool data across multiple DEXes" or "we need a skill that builds SolidJS components with Tailwind CSS following atomic design patterns."

**Layer 3: Does each gap warrant a full team or a single skill?**

| Situation | Recommendation |
|-----------|---------------|
| Project requires significant development in a language | Full skill team (like rust:*) with coordinator |
| Project needs a specific type of research or analysis | Single specialized skill |
| Project needs UI work in a framework | Full UI skill team with coordinator |
| Project needs simple scripting or glue code | Single development skill may suffice |
| Project needs infrastructure/deployment work | 1-3 focused skills depending on complexity |
| Project needs data pipeline work | Depends on complexity — could be single skill or team |

**Layer 4: How should new skill teams be structured?**

When a full team is needed, model it after the `rust:*` pattern. This pattern exists because software development has distinct phases that benefit from specialist focus:

| Skill Role | Purpose | Example for Python |
|------------|---------|-------------------|
| `<prefix>:exploration-and-planning` | Read-only codebase exploration, produce implementation plans | `python:exploration-and-planning` |
| `<prefix>:planning-and-architecture` | Performance-critical design decisions, data structure selection | `python:planning-and-architecture` |
| `<prefix>:developing` | Execute implementation from a plan | `python:developing` |
| `<prefix>:ergonomic-refactoring` | Make code idiomatic and readable | `python:ergonomic-refactoring` |
| `<prefix>:unit-testing` | Write minimal unit tests, then clean up | `python:unit-testing` |
| `<prefix>:integration-testing` | Write production-grade integration tests | `python:integration-testing` |
| `<prefix>:commenting` | Add minimal, consistent comments | `python:commenting` |
| `<prefix>:errors-management` | Architect robust error handling | `python:errors-management` |
| `<prefix>:team-coordinator` | Triage and orchestrate the team | `python:team-coordinator` |

Not every team needs all 9 roles. A simpler domain might only need exploration + developing + testing + coordinator. But for any serious coding work, the full set is recommended.

The team coordinator is **always** the last skill created and **always** required when there's more than one skill in a domain.

#### 3d. Present recommendations to the user

Structure your recommendation clearly:

**1. Existing skills that apply to this project** — list each with a one-line explanation of how it maps to the project's needs. Group by team/prefix.

**2. New skills recommended** — for each: proposed name (with prefix), what it would do, why it's needed (tied to intake findings), whether it's part of a team or standalone.

**3. Skills NOT needed (and why)** — briefly explain what you considered but ruled out.

**4. Complexity assessment** — overall assessment of team size needed.

#### 3e. Discuss and confirm with the user

This is a conversation, not a report. After presenting:

- Ask if the team composition looks right
- Ask if there are domains or work types you missed
- Ask if any proposed skills seem unnecessary or over-engineered
- Listen to feedback and adjust

Common adjustments:
- "I don't need a full team for X, just a single development skill is fine"
- "Actually, we also need Y which I didn't mention in intake"
- "Don't bother with a commenting skill for this, it's a prototype"
- "The research here needs to be very specific — it should be a `<domain>-<specialty>-researcher`"

Iterate until the user confirms.

---

### Section: team_build

Create the skills that don't exist yet. The team composition was agreed in team_define.

#### 3a. Search for existing external skills first

For skills that might exist in the ecosystem, use `/find-skills`:

```
/find-skills <domain> <specific-need>
```

For example:
- `/find-skills solidjs components` for a SolidJS UI team
- `/find-skills python testing pytest` for Python testing skills
- `/find-skills deployment kubernetes` for infrastructure skills

If a high-quality external skill is found, present it to the user. If not, move it to the creation queue.

#### 3b. Create missing skills

For skills that need custom creation, use `/skill-creator:skill-creator`.

**For coding team skills (language-specific teams):**

Use the existing `rust:*` skills as templates — they're battle-tested:

1. Read each relevant `rust:*` skill's SKILL.md
2. Adapt for the target language/framework — change commands, conventions, tooling, patterns
3. Maintain the same structural discipline — phases, triage logic, context-passing

**Use multi-agents to create skills in parallel when possible.** Independent skills (like `python:commenting` and `python:unit-testing`) have no dependencies. But the **team-coordinator must always be created last** — it references all specialists.

**For standalone skills:**

Give `/skill-creator:skill-creator` a clear description of what the skill enables, when it triggers, the expected workflow, and what domain expertise it embodies.

**For research or analysis skills:**

Be maximally specific in the name:

| Too generic | Specific and useful |
|-------------|-------------------|
| `researcher` | `defi-liquidity-analysis-researcher` |
| `data-analyst` | `time-series-anomaly-detection-analyst` |
| `ml-engineer` | `transformer-fine-tuning-engineer` |
| `ui-developer` | `solidjs-reactive-component-developer` |

#### 3c. Write the team coordinator (if applicable)

If new skills form a team (share a prefix), the team coordinator is the capstone. Written last because it needs to:

1. Know every specialist by name
2. Define the triage logic (direct dispatch vs. pipeline)
3. Define pipeline phases and skip conditions
4. Specify model assignments (opus, sonnet, haiku)
5. Define context passing between phases

Use `rust:team-coordinator` SKILL.md as your template.

#### 3d. Verify all skills were created

```bash
ls .claude/skills/<skill-name>/SKILL.md
```

Do this for every skill that was supposed to be created. Flag and create anything missing.

#### 3e. Summarize to the user

Report: how many skills created, how many found externally vs. built from scratch, team(s) now available with coordinators, any skills needing refinement.

---

### Section: team_interview

Load the `hx:intake-team-interviewing` skill from `.claude/skills/hx-intake-team-interviewing/SKILL.md` and follow its full instructions. That skill handles the interview process — adopting the specialist's perspective, asking targeted questions, capturing action items, and writing interview documents.

---

## Step 4: Complete the section

When the section has been thoroughly covered:

```bash
harnessx intake-team complete <section>
```

Give the user a brief wrap-up of what was accomplished.

## Step 5: Loop — advance to the next section

After completing a section, immediately run `harnessx intake-team next` again to get the next incomplete section. If there is one:

1. Mark it as in-progress: `harnessx intake-team update <section> in_progress`
2. Load any skills listed in the new response's `skills` array (same as Step 2)
3. Execute the section (Step 3)
4. Mark it complete and loop back here

Continue this cycle until `harnessx intake-team next` returns no remaining sections. When that happens:

1. Mark the pipeline stage complete:
   ```bash
   harnessx progress complete intake_team
   ```
2. Tell the user the full intake team process is complete — team has been defined, built, and interviewed.
3. **Stop.** Confirm what the next pipeline stage will be and let the user know they can continue via `/hx:operator`.

---

## Judgment Calls

### When NOT to recommend a full team

A full skill team (9 specialists + coordinator) is warranted when:
- Significant original development in a language/framework
- Quality matters — tests, error handling, code review expected
- Codebase will be maintained long-term
- Multiple agents will work in it

A single skill or small cluster is better when:
- One-off script or prototype
- Narrow, well-defined domain
- User explicitly wants simplicity
- Time-boxed and disposable

When in doubt, ask the user.

### Naming conventions

- **Team skills:** `<domain>:<role>` — e.g., `python:developing`, `ui:component-builder`
- **Standalone skills:** `<domain>-<specialty>` — e.g., `defi-liquidity-researcher`
- **Team coordinators:** `<domain>:team-coordinator` — e.g., `python:team-coordinator`

Skill directory names replace `:` with `-`.

### Handling uncertainty

If intake docs don't provide enough info, ask the user specific questions rather than guessing.

---

## Edge Cases

**No new skills needed:** If existing skills cover everything, say so — don't invent unnecessary skills. Mark sections complete with a note that the existing team covers all needs.

**User wants to defer:** Mark the current section complete and leave remaining sections as `not_started`. The pipeline picks them up next time.

**Skill creation fails:** Document failures, create what you can, flag to the user. Don't block the entire build on one problematic skill.

**Re-running this skill:** Check `harnessx intake-team next` and pick up where you left off. Don't redo completed work.
