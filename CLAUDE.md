# How harnessx Works

harnessx is a Rust CLI + Claude Code skill system that orchestrates software projects from intake through delivery. The CLI manages state (projects, progress, actions) via JSON files on disk, while a fleet of specialized skills handle the actual conversations, decisions, and code work.

---

## Architecture at a Glance

```
User
  ↓
/hx:operator (skill) ─── reads progress ───→ invokes /skill-name directly
                                                       ↓
                                              skill runs in main conversation
                                                       ↓
                                              runs harnessx CLI for state
                                                       ↓
                                              creates action items, marks progress
```

Two layers:
1. **CLI** (`harnessx` binary) — stateless Rust tool that reads/writes JSON files
2. **Skills** (`.claude/skills/`) — prompt instructions that teach Claude how to do specific work

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
| `harnessx init` | Scaffold the full harnessx directory structure (skills, hooks, docs, Obsidian vault) |
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

**IntakeItem** — status + skills array (per intake section)

**Stage** — status + skill name (per pipeline stage)

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
│   ├── skills/                            # 16+ skills with SKILL.md files
│   ├── hooks/                             # session-start.sh, commit-and-push.sh
│   └── settings.local.json               # Permission whitelist
├── .obsidian/                             # Obsidian vault config
├── harness/                               # CLI reference docs
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

Each stage has an assigned skill. `harnessx progress next` returns the first incomplete stage + its skill, which the operator skill uses to route work.

Stages with `"hx:TODO-WARN-USER"` as skill are not yet implemented.

---

## Intake Onboarding Flow

6 sections, each with a dedicated skill:

| # | Section | Skill |
|---|---------|-------|
| 1 | goal | hx:intake-onboarding-goal |
| 2 | scope | hx:intake-onboarding-scope |
| 3 | user_knowledge | hx:intake-onboarding-user-knowledge |
| 4 | resources | hx:intake-onboarding-resources |
| 5 | success_measures | hx:intake-onboarding-success-measures |
| 6 | user_acceptance_testing | hx:intake-onboarding-uat |

### How Each Section Works

The `hx:intake-onboarding` skill runs the loop directly in the main conversation:

1. `harnessx intake-onboarding next` → gets section name and required skills
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

3 sections tracked separately from onboarding (skills not yet implemented):

| # | Section | Skill |
|---|---------|-------|
| 1 | exploration | *(not yet implemented)* |
| 2 | ideation | *(not yet implemented)* |
| 3 | project_risk_manager | *(not yet implemented)* |

Same CLI pattern as onboarding: `harnessx intake-completion init|status|list|next|complete <section>`.

Stored at `harnessx/<id>/intake/intake_completion.json`.

---

## Intake Team Flow

3 sections for team intake (skills not yet implemented):

| # | Section | Skill |
|---|---------|-------|
| 1 | team_define | *(not yet implemented)* |
| 2 | team_build | *(not yet implemented)* |
| 3 | team_interview | *(not yet implemented)* |

Same CLI pattern: `harnessx intake-team init|status|list|next|complete <section>`.

Stored at `harnessx/<id>/intake/intake_team.json`.

---

## Action Item Capture

Actions are created throughout intake with:
- **origin**: `intake:<section>` for traceability
- **detail**: includes *why*, not just *what*
- **notes**: skill observations and user context
- Captured in **real-time**, never batched

---

## The Operator — Entry Point

The `hx:operator` skill is how everything starts:

```
/hx:operator
    │
    ├─ No active project?
    │   → Ask user for project description
    │   → Create project (kebab-case ID)
    │   → /compact
    │   → Invoke /hx:intake-onboarding skill
    │
    └─ Active project exists?
        → harnessx progress next
        │
        ├─ skill starts with "hx:" → Invoke skill directly (interactive)
        ├─ skill starts with "rust-" → Invoke directly or delegate to subagent
        └─ skill field empty → Project pipeline complete
```

---

## The Skill Fleet

### HX Skills (Project Orchestration)

| Skill | Purpose |
|-------|---------|
| **hx:operator** | Entry point — routes to the right skill based on pipeline state |
| **hx:intake-onboarding** | Guide intake conversations section by section |
| **hx:user-troubleshooting** | Diagnose and resolve pipeline blockages |

### Rust Skills (Development)

| Skill | Purpose |
|-------|---------|
| **rust-exploration-and-planning** | Explore codebase, produce implementation plans (read-only) |
| **rust-planning-and-architecture** | Performance-critical architecture decisions |
| **rust-developing** | Execute implementation from a plan |
| **rust-ergonomic-refactoring** | Make code idiomatic and readable |
| **rust-unit-testing** | Write minimal unit tests, then clean up |
| **rust-integration-testing** | Write production-grade integration tests |
| **rust-commenting** | Add minimal, consistent comments |

### The Failure → Troubleshooting Loop

When integration tests fail and need user input:

```
/rust-integration-testing skill
    → writes failure report to harnessx/<id>/integration-tests/failing.md
    → runs: harnessx progress update user_input_required not_started
    → pipeline reroutes to /hx:user-troubleshooting skill
    → user provides input/decision
    → runs: harnessx progress complete user_input_required
    → pipeline continues
```

---

## Hooks

| Hook | Trigger | Behavior |
|------|---------|----------|
| `session-start.sh` | Session starts | Outputs project.json, runs project-specific init.sh |
| `commit-and-push.sh` | Session ends | Stages all changes, auto-commits with timestamp, pushes |

---

## Obsidian Integration

The CLI scaffolds a `.obsidian/` vault with preconfigured plugins (file explorer, search, graph, backlinks, tags, outline, etc.). Combined with the Obsidian CLI (`obsidian`), skills can:

- Search by tags/wikilinks: `obsidian search query="tag:#your-tag" format=json`
- Search with context: `obsidian search:context query="/\[\[some_wikilink\]\]/" format=json`
- Set properties: `obsidian property:set file="about" name="status" value="analyzed"`
- Query by properties: `obsidian search query="[status:analyzed]" format=json`

Purpose: reduce token usage by allowing local indexing and structured vault-based documentation.

---

## Init Flow

`harnessx init` bootstraps everything:

1. Detect platform (Claude or Cursor) from CLAUDE.md/AGENTS.md
2. Detect Obsidian CLI on PATH
3. Embed template files (hooks, skills, docs, Obsidian config) compiled into the binary via `include_dir!`
4. Write files to disk, respecting `--force` and `--no-obsidian` flags
5. Set executable permissions on shell scripts
6. Create root markdown file (CLAUDE.md or AGENTS.md)

---

## Key Design Principles

1. **JSON files as database** — no external dependencies, everything is files on disk
2. **Stateless CLI** — the binary reads/writes JSON and exits; skills drive the workflow
3. **Skills as prompts** — skills are markdown instructions, not code; they teach Claude procedures
4. **Skills run in the main conversation** — interactive skills (hx:*) talk directly to the user, no relay
5. **Real-time action capture** — action items are created as they emerge, not batched
6. **Failure routing** — integration test failures automatically route to the troubleshooting skill
7. **Read-only exploration** — the `rust-exploration-and-planning` skill never writes code, only produces plans
