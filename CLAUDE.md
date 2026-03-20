# How harnessx Works

harnessx is a Rust CLI + Claude Code agent system that orchestrates software projects from intake through delivery. The CLI manages state (projects, progress, actions) via JSON files on disk, while a fleet of specialized agents and skills handle the actual conversations, decisions, and code work.

---

## Architecture at a Glance

```
User
  ↓
/hx:operator (skill) ─── routes to ───→ @specialist-agent
                                              ↓
                                     loads skill(s) from SKILL.md
                                              ↓
                                     runs harnessx CLI for state
                                              ↓
                                     creates action items, marks progress
```

Three layers:
1. **CLI** (`harnessx` binary) — stateless Rust tool that reads/writes JSON files
2. **Skills** (`.claude/skills/`) — prompt instructions that teach agents how to do specific work
3. **Agents** (`.claude/agents/`) — specialist personas with scoped tool access that execute skills

---

## The Rust CLI

Built with clap, serde, smol_str, thiserror, and include_dir. All commands output JSON in a standard envelope:

```json
{ "success": true, "data": { ... } }
{ "success": false, "error": "..." }
```

### Commands

| Command | Purpose |
|---------|---------|
| `harnessx init` | Scaffold the full harnessx directory structure (agents, skills, hooks, docs, Obsidian vault) |
| `harnessx project <sub>` | Create, list, activate, remove, and update projects |
| `harnessx intake-onboarding <sub>` | Track progress through 6 intake onboarding sections |
| `harnessx intake-completion <sub>` | Track progress through 3 intake completion sections |
| `harnessx intake-team <sub>` | Track team intake sections (define, build, interview) |
| `harnessx intake-actions <sub>` | CRUD for action items captured during intake |
| `harnessx progress <sub>` | Track progress through 9 pipeline stages |

### Data Models

**Project** — id, title, subtitle, description, takeaway_line, directory, user_name

**ProjectRegistry** — one `active` project + array of `inactive` projects → `harnessx/projects.json`

**Status** (shared) — `not_started | in_progress | completed | rework`

**IntakeItem** — status + agent tier + skills array (per intake section)

**Stage** — status + agent name (per pipeline stage)

**ActionItem** — id, title, category, origin, detail, tags, input_docs, complexity, mode, notes[]

**Complexity** — `super-low | low | medium | high | super-high | uncertain`

**ActionMode** — `plan | execute | review | rework`

---

## Directory Structure on Disk

```
harnessx/
├── projects.json                          # Project registry
├── <project-id>/
│   ├── progress.json                      # Pipeline stage tracking
│   └── intake/
│       ├── intake_progress.json           # Intake onboarding tracking
│       ├── intake_completion.json         # Intake completion tracking
│       ├── intake_actions.json            # Action items
│       └── intake_team.json              # Team intake tracking
│   └── integration-tests/
│       └── failing.md                     # Failure reports for user input
├── .claude/
│   ├── agents/                            # 8 specialist agents
│   ├── skills/                            # 16+ skills with SKILL.md files
│   ├── hooks/                             # session-start.sh, commit-and-push.sh
│   ├── agent-memory/                      # Persistent agent knowledge
│   └── settings.local.json               # Permission whitelist
├── .obsidian/                             # Obsidian vault config
├── docs/                                  # CLI reference docs
└── src/                                   # Rust source code
    ├── bin/main.rs                         # CLI entry point
    └── lib/
        ├── lib.rs                          # Library root
        ├── commands/                       # Command implementations
        │   ├── init.rs
        │   ├── project.rs
        │   ├── intake_onboarding.rs
        │   ├── intake_completion.rs
        │   ├── intake_team.rs
        │   ├── intake_actions.rs
        │   └── progress.rs
        └── models/                         # Data structures
            ├── project.rs
            ├── intake_onboarding.rs
            ├── intake_completion.rs
            ├── intake_team.rs
            ├── intake_actions.rs
            ├── progress.rs
            └── status.rs                   # Status enum, Response, errors
```

---

## The Project Pipeline

Every project moves through 9 stages tracked in `progress.json`:

```
user_input_required → intake_onboarding → intake_team → intake_exploration
        ↓                    ↓                 ↓              ↓
   (troubleshoot)     (6 sections)        (define,        (technical
                                           build,          discovery)
                                          interview)
        → planning → review → execution → user_acceptance → complete
```

Each stage has an assigned agent. `harnessx progress next` returns the first incomplete stage + its agent, which the operator skill uses to route work.

Stages with `"hx-TODO-WARN-USER"` as agent are not yet implemented.

---

## Intake Onboarding Flow

6 sections, each with a dedicated skill:

| # | Section | Skill | Agent Tier |
|---|---------|-------|------------|
| 1 | goal | hx:intake-onboarding-goal | opus |
| 2 | scope | hx:intake-onboarding-scope | opus |
| 3 | user_knowledge | hx:intake-onboarding-user-knowledge | opus |
| 4 | resources | hx:intake-onboarding-resources | opus |
| 5 | success_measures | hx:intake-onboarding-success-measures | opus |
| 6 | user_acceptance_testing | hx:intake-onboarding-uat | opus |

### How Each Section Works

The `@hx-intake-onboarding-specialist` agent runs the loop:

1. `harnessx intake-onboarding next` → gets section name, agent tier, required skills
2. Reads the skill's SKILL.md to learn how to conduct that section
3. Has a conversation with the user, asking probing questions
4. Creates action items in real-time as they emerge (`harnessx intake-actions create`)
5. Marks section complete (`harnessx intake-onboarding complete <section>`)
6. Loops back to step 1

### What Each Section Captures

- **goal** — Crafts a 1-3 sentence goal statement, then populates all project metadata fields (title, subtitle, description, user_name, takeaway_line, directory)
- **scope** — Defines boundaries across 5 dimensions: feature, user, technical, quality, timeline
- **user_knowledge** — Extracts the user's professional background, domain expertise, and working preferences
- **resources** — Collects repos, docs, APIs, designs with concrete follow-up actions for each
- **success_measures** — Defines must-have (blocking) and nice-to-have criteria organized as observable, measurable, agreed
- **user_acceptance_testing** — Defines demo scenarios, hands-on testing, evidence/artifacts, edge cases, handover process

---

## Intake Completion Flow

3 sections tracked separately from onboarding (skills/agents not yet implemented):

| # | Section | Skill | Agent Tier |
|---|---------|-------|------------|
| 1 | exploration | *(not yet implemented)* | opus |
| 2 | ideation | *(not yet implemented)* | opus |
| 3 | project_risk_manager | *(not yet implemented)* | opus |

Same CLI pattern as onboarding: `harnessx intake-completion init|status|list|next|complete <section>`.

Stored at `harnessx/<id>/intake/intake_completion.json`.

---

## Intake Team Flow

3 sections for team intake (skills/agents not yet implemented):

| # | Section | Skill | Agent Tier |
|---|---------|-------|------------|
| 1 | team_define | *(not yet implemented)* | opus |
| 2 | team_build | *(not yet implemented)* | opus |
| 3 | team_interview | *(not yet implemented)* | opus |

Same CLI pattern: `harnessx intake-team init|status|list|next|complete <section>`.

Stored at `harnessx/<id>/intake/intake_team.json`.

---

## Action Item Capture

Actions are created throughout intake with:
- **origin**: `intake:<section>` for traceability
- **detail**: includes *why*, not just *what*
- **notes**: agent observations and user context
- Captured in **real-time**, never batched

---

## The Operator — Entry Point

The `hx-operator` skill is how everything starts:

```
/hx:operator
    │
    ├─ No active project?
    │   → Ask user for project description
    │   → Create project (kebab-case ID)
    │   → /compact
    │   → Launch @hx-intake-onboarding-specialist
    │
    └─ Active project exists?
        → harnessx progress next
        │
        ├─ Agent field present → Launch @{agent}
        └─ Agent field empty → Project pipeline complete
```

---

## The Agent Fleet

### HX Agents (Project Orchestration)

| Agent | Purpose | Model | Key Tools |
|-------|---------|-------|-----------|
| **hx-intake-onboarding-specialist** | Guide intake conversations section by section | opus | Read, Bash(harnessx:*) |
| **hx-user-troubleshooting-specialist** | Diagnose and resolve pipeline blockages | opus | Read, Edit, Write, Skill, Bash(harnessx:*, git, cargo) |

### Rust Agents (Development)

| Agent | Purpose | Model | Read-Only? |
|-------|---------|-------|-----------|
| **rust-navigator** | Explore codebase, produce implementation plans | opus | Yes |
| **rust-senior-architect** | Performance-critical architecture decisions | opus | No |
| **rust-ergonomics-specialist** | Make code idiomatic and readable | opus | No |
| **rust-unit-testing-specialist** | Write minimal unit tests, then clean up | sonnet | No |
| **rust-integration-testing-specialist** | Write production-grade integration tests | opus | No |
| **rust-commenting-specialist** | Add minimal, consistent comments | sonnet | No |

### Agent → Skill Mapping

Each agent loads specific skills:
- `rust-navigator` → `rust-exploration-and-planning`
- `rust-senior-architect` → `rust-planning-and-architecture`
- `rust-ergonomics-specialist` → `rust-ergonomic-refactoring`
- `rust-unit-testing-specialist` → `rust-unit-testing`
- `rust-integration-testing-specialist` → `rust-integration-testing`
- `rust-commenting-specialist` → `rust-commenting`

### The Failure → Troubleshooting Loop

When integration tests fail and need user input:

```
@rust-integration-testing-specialist
    → writes failure report to harnessx/<id>/integration-tests/failing.md
    → runs: harnessx progress update user_input_required not_started
    → pipeline reroutes to @hx-user-troubleshooting-specialist
    → user provides input/decision
    → runs: harnessx progress complete user_input_required
    → pipeline continues
```

---

## Hooks

| Hook | Trigger | Behavior |
|------|---------|----------|
| `session-start.sh` | Agent session starts | Outputs project.json, runs project-specific init.sh |
| `commit-and-push.sh` | Agent session ends | Stages all changes, auto-commits with timestamp, pushes |

---

## Obsidian Integration

The CLI scaffolds a `.obsidian/` vault with preconfigured plugins (file explorer, search, graph, backlinks, tags, outline, etc.). Combined with the Obsidian CLI (`obsidian`), agents can:

- Search by tags/wikilinks: `obsidian search query="tag:#your-tag" format=json`
- Search with context: `obsidian search:context query="/\[\[some_wikilink\]\]/" format=json`
- Set properties: `obsidian property:set file="about" name="agent-status" value="analyzed"`
- Query by properties: `obsidian search query="[agent-status:analyzed]" format=json`

Purpose: reduce token usage by allowing local indexing and structured vault-based documentation.

---

## Init Flow

`harnessx init` bootstraps everything:

1. Detect agent platform (Claude or Cursor) from CLAUDE.md/AGENTS.md
2. Detect Obsidian CLI on PATH
3. Embed template files (agents, hooks, skills, docs, Obsidian config) compiled into the binary via `include_dir!`
4. Write files to disk, respecting `--force` and `--no-obsidian` flags
5. Set executable permissions on shell scripts
6. Create root markdown file (CLAUDE.md or AGENTS.md)

---

## Agent Memory

Agents accumulate institutional knowledge in `.claude/agent-memory/<agent-name>/`. Currently the `rust-ergonomics-specialist` has stored patterns about the CLI's command architecture (JSON commands use `exit_with()`, interactive commands use internal `execute()`).

---

## Key Design Principles

1. **JSON files as database** — no external dependencies, everything is files on disk
2. **Stateless CLI** — the binary reads/writes JSON and exits; agents drive the workflow
3. **Skills as prompts** — skills are markdown instructions, not code; they teach agents procedures
4. **Real-time action capture** — action items are created as they emerge, not batched
5. **Failure routing** — integration test failures automatically route to the troubleshooting agent
6. **Read-only exploration** — the `rust-navigator` never writes code, only produces plans
7. **Agent tier selection** — some sections use `sonnet` (cheaper/faster) for simpler tasks like commenting
