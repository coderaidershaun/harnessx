---
name: hx:intake-team
description: Define and build the team of specialist agent skills required to complete a harnessx project. Reviews all intake documents and action items, catalogs available skills, recommends team composition (existing skills + new skills needed), discusses with the user, then builds missing skills using /find-skills and /skill-creator. Handles both the team_define and team_build sections of intake-team. Use this skill when the pipeline reaches intake_team, when the user says "define the team", "what skills do I need", "build my team", "assemble the agents", or anything about determining which agent skills a project requires. Also trigger when the operator routes to the intake_team stage.
disable-model-invocation: false
user-invocable: true
---

# Intake Team: Define & Build

You analyze a project's intake documents and action items, determine what team of specialist agent skills is required to deliver it, and then build any missing skills. You handle two intake-team sections in sequence: `team_define` (analyze and recommend) and `team_build` (find or create missing skills).

The core insight driving this skill: **skills are not subagents**. Skills are markdown instruction sets that teach Claude how to do specific work. They get assigned to agents at execution time. Your job is to figure out which skill instruction sets the project needs, check what already exists, and create what's missing.

---

## Startup

### 1. Confirm active project

```bash
harnessx project active
```

If no active project exists, tell the user to run `/hx:operator` first and stop.

Capture the project ID — you'll need it for file paths throughout.

### 2. Initialize intake-team tracking

Check if intake-team is already initialized:

```bash
harnessx intake-team status
```

If it returns an error (not found), initialize it:

```bash
harnessx intake-team init
```

### 3. Check current position

```bash
harnessx intake-team next
```

This tells you which section to work on. If `team_define` is next, proceed to Phase 1. If `team_build` is next (team_define already complete), skip to Phase 2. If all sections are complete, tell the user and stop.

---

## Phase 1: Team Define

The goal here is to understand what the project needs, map those needs to skills, identify gaps, and agree on a team composition with the user.

### Step 1: Gather all project context

Read every intake document and the action items. This gives you the full picture of what's been captured about the project.

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

### Step 2: Catalog all available skills

List every skill currently installed:

```bash
ls .claude/skills/
```

For each skill directory, read its `SKILL.md` frontmatter (the `name` and `description` fields). Build a catalog of what's already available, organized by domain:

- **hx:*** — project orchestration skills (intake, operator, troubleshooting)
- **rust:*** — Rust development team (exploration, architecture, developing, testing, refactoring, commenting, errors, coordination)
- **Any other prefixes** — other domain teams that may already exist

Understanding the existing skill landscape is critical before recommending new ones.

### Step 3: Analyze project needs and map to skills

Now synthesize what you learned from the intake documents with what skills exist. Think about this in layers:

**Layer 1: What existing skills directly apply?**

Look at the project's technology stack and required work types. If the project involves Rust development, the entire `rust:*` team is immediately relevant. If it involves Python, check whether `python:*` skills exist. Map each major work area to existing skills.

**Layer 2: What gaps exist?**

For each domain the project touches that doesn't have a matching skill team, note the gap. Be specific about what's missing — not "we need a research skill" but "we need a skill that can analyze on-chain liquidity pool data across multiple DEXes" or "we need a skill that builds SolidJS components with Tailwind CSS following atomic design patterns."

**Layer 3: Does each gap warrant a full team or a single skill?**

This is the key judgment call. Use these guidelines:

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

Not every team needs all 9 roles. A simpler domain might only need exploration + developing + testing + coordinator. But for any serious coding work, the full set is recommended — it's what makes the difference between "code that works" and "code that's excellent."

The team coordinator is **always** the last skill created and **always** required when there's more than one skill in a domain. It's the entry point that knows how to deploy the specialists.

### Step 4: Present recommendations to the user

Structure your recommendation clearly. Present it in these sections:

**1. Existing skills that apply to this project**

List each existing skill that's relevant, with a one-line explanation of how it maps to the project's needs. Group by team/prefix.

**2. New skills recommended**

For each new skill or skill team needed:
- The proposed skill name (with prefix)
- What it would do
- Why it's needed (tied to specific intake findings)
- Whether it's part of a team or standalone

**3. Skills NOT needed (and why)**

Briefly explain what you considered but ruled out. This shows the user you've thought comprehensively and helps them catch if you've misjudged something.

**4. Complexity assessment**

Give your overall assessment: is this a project that needs a large multi-team operation, or can it be handled with existing skills plus one or two additions?

### Step 5: Discuss and confirm with the user

This is a conversation, not a report. After presenting your recommendations:

- Ask if the team composition looks right
- Ask if there are domains or work types you missed
- Ask if any of the proposed new skills seem unnecessary or over-engineered
- Listen to the user's feedback and adjust

The user knows their project better than you do. They may have strong opinions about team size, skill granularity, or whether certain work even needs a dedicated skill. Respect those opinions.

Common adjustments the user might request:
- "I don't need a full team for X, just a single development skill is fine"
- "Actually, we also need Y which I didn't mention in intake"
- "Don't bother with a commenting skill for this, it's a prototype"
- "The research here needs to be very specific — it should be a `<domain>-<specialty>-researcher` not a generic researcher"

Iterate until the user confirms the team composition.

### Step 6: Complete team_define

Once the user has confirmed the team composition:

```bash
harnessx intake-team complete team_define
```

Summarize what was agreed: which existing skills will be used, which new skills need to be created, and what teams they form.

---

## Phase 2: Team Build

Now you create the skills that don't exist yet. The team composition was agreed in Phase 1 — this phase is about execution.

### Step 1: Prioritize what to build

Sort the skills to be created into two categories:

1. **Skills that might exist externally** — generic or well-known domains (React, Python, testing frameworks, deployment tools)
2. **Skills that definitely need custom creation** — project-specific or niche domains (your user's particular research area, a custom data pipeline pattern, a specialized trading strategy)

### Step 2: Search for existing external skills first

For each skill in category 1, use `/find-skills` to check if a suitable skill already exists in the ecosystem:

```
/find-skills <domain> <specific-need>
```

For example:
- `/find-skills solidjs components` for a SolidJS UI team
- `/find-skills python testing pytest` for Python testing skills
- `/find-skills deployment kubernetes` for infrastructure skills

If a high-quality external skill is found (good install count, reputable source), present it to the user. It may cover the need entirely, or it may serve as a foundation that can be customized.

If no suitable external skill exists, or the user prefers a custom skill, move it to the creation queue.

### Step 3: Create missing skills

This is where the heavy lifting happens. For skills that need to be created from scratch, use the `/skill-creator:skill-creator` skill.

**For coding team skills (language-specific teams):**

When building a full development team for a language or framework, the most efficient approach is to use the existing `rust:*` skills as templates. They represent a battle-tested team structure. The process:

1. Read each relevant `rust:*` skill's SKILL.md
2. Adapt it for the target language/framework — change Rust-specific commands, conventions, tooling, and patterns to the target's equivalents
3. Maintain the same structural discipline — the phases, the coordinator's triage logic, the context-passing between agents

For example, creating a `python:*` team:
- `python:developing` would reference `pytest` instead of `cargo test`, `ruff` instead of `clippy`, Python module patterns instead of Rust module patterns
- `python:team-coordinator` would have the same tier system but with Python-appropriate complexity signals
- `python:unit-testing` would follow `pytest` conventions instead of `#[cfg(test)]` modules

**Use multi-agents to create skills in parallel when possible.** Independent skills (like `python:commenting` and `python:unit-testing`) have no dependencies on each other and can be created simultaneously. But the **team-coordinator must always be created last**, because it needs to reference all the specialist skills it will orchestrate.

**For standalone skills:**

Single-purpose skills are simpler to create. Give `/skill-creator:skill-creator` a clear description of:
- What the skill should enable Claude to do
- When it should trigger
- What the expected workflow looks like
- What domain expertise it needs to embody

**For research or analysis skills:**

Be maximally specific in the skill name and scope. The name should tell you exactly what kind of specialist this is:

| Too generic | Specific and useful |
|-------------|-------------------|
| `researcher` | `defi-liquidity-analysis-researcher` |
| `data-analyst` | `time-series-anomaly-detection-analyst` |
| `ml-engineer` | `transformer-fine-tuning-engineer` |
| `ui-developer` | `solidjs-reactive-component-developer` |

The specificity in the name carries into the skill's instructions — a `defi-liquidity-analysis-researcher` skill will contain domain knowledge about AMMs, impermanent loss, pool mechanics, and on-chain data sources that a generic `researcher` skill never would.

### Step 4: Write the team coordinator (if applicable)

If new skills form a team (share a prefix), the team coordinator is the capstone. It must be written after all specialist skills exist because it needs to:

1. **Know every specialist by name** — list them with their roles and when to deploy each
2. **Define the triage logic** — what task types map to which specialist, and when to use the full pipeline vs. direct dispatch
3. **Define the pipeline phases** — the sequence of specialist deployment for complex tasks, including which phases to skip for simpler work
4. **Specify model assignments** — which model (opus, sonnet, haiku) each specialist should run on
5. **Define context passing** — what information flows between phases

Use the `rust:team-coordinator` SKILL.md as your template. Read it thoroughly and adapt its structure for the new team's domain. The tier system (direct dispatch, lightweight pipeline, full pipeline) is universally applicable — adapt the specific signals and examples for the new domain.

### Step 5: Verify all skills were created

After creation is complete, verify each skill exists:

```bash
ls .claude/skills/<skill-name>/SKILL.md
```

Do this for every skill that was supposed to be created. If anything is missing, flag it and create it.

### Step 6: Complete team_build

Once all skills are created and verified:

```bash
harnessx intake-team complete team_build
```

Give the user a summary:
- How many skills were created
- How many were found externally vs. built from scratch
- The team(s) that are now available with their coordinators
- Any skills that were attempted but need further refinement

---

## After Both Phases Complete

Once both `team_define` and `team_build` are marked complete, **stop and confirm what comes next**.

Check what the next intake-team section is:

```bash
harnessx intake-team next
```

This should return `team_interview`. Tell the user:

> Team definition and building are complete. The next step is **team interviews** — where each specialist agent gets briefed on the project through a focused Q&A session. This is handled by the `hx:agent-interview` skill, which lets you interview each agent from the team before they start work.
>
> When you're ready to continue, the operator will route you to the interview phase.

Then stop. Do not proceed to interviews — that's a separate skill's responsibility and will be handled when the user continues.

---

## Judgment Calls

### When NOT to recommend a full team

Not every project needs a battalion. A full skill team (9 specialists + coordinator) is warranted when:

- The project involves significant original development in a language/framework
- Quality matters — tests, error handling, and code review are expected
- The codebase will be maintained long-term
- Multiple people (or agents) will work in it

A single skill or small cluster is better when:

- The work is a one-off script or prototype
- The domain is narrow and well-defined
- The user explicitly wants to keep things simple
- The project is time-boxed and disposable

When in doubt, ask the user. Present both options with honest tradeoffs.

### Naming conventions

All skill names follow these patterns:

- **Team skills:** `<domain>:<role>` — e.g., `python:developing`, `ui:component-builder`, `infra:terraform-deployer`
- **Standalone skills:** `<domain>-<specialty>` — e.g., `defi-liquidity-researcher`, `timeseries-anomaly-detector`
- **Team coordinators:** `<domain>:team-coordinator` — e.g., `python:team-coordinator`, `ui:team-coordinator`

Skill directory names replace `:` with `-` — `python:developing` lives at `.claude/skills/python-developing/SKILL.md`.

### Handling uncertainty

If the intake documents don't provide enough information to determine whether a skill is needed, don't guess. Ask the user specific questions:

- "The intake mentions data processing but doesn't specify the scale. Are we talking about hundreds of records or millions? This affects whether we need a dedicated data pipeline skill or if it's just part of the backend work."
- "I see references to a frontend but no mention of the framework. What UI technology are you using? This determines whether we need a generic frontend skill or a framework-specific team."

---

## Edge Cases

**No new skills needed:** Sometimes the existing skill set is sufficient. If the project only needs Rust development and all the `rust:*` skills exist, say so — don't invent unnecessary skills just to have something to build. Mark both phases complete with a note that the existing team covers all needs.

**User wants to defer team building:** If the user agrees on the team definition but wants to build the skills later, mark `team_define` as complete but leave `team_build` as `not_started`. The pipeline will pick it up next time.

**Skill creation fails:** If `/skill-creator:skill-creator` encounters issues creating a skill, document what went wrong, create what you can, and flag the failures to the user. Don't block the entire team build on one problematic skill — create the others and come back to the difficult one.

**Mixed external and custom skills:** A team can include both installed external skills and custom-created ones. The team coordinator just needs to know about all of them regardless of origin.

**Re-running this skill:** If `team_define` is already complete and the user runs this skill again, check `harnessx intake-team next` and pick up where you left off. Don't redo completed work.
