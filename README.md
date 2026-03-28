# harnessx CLI

[Crate](https://crates.io/crates/harnessx) | [GitHub](https://github.com/coderaidershaun/harnessx)

Command-line interface for harnessx project management. All output is JSON.

> **Note:** This README is embedded inside the `harnessx/` folder that is created when you run `harnessx init`.

## Installation

You need [Rust](https://www.rust-lang.org/tools/install) installed first, then run:

```bash
cargo install harnessx
```

## Quick Start

```bash
# Scaffold the harnessx harness
harnessx init

# Launch claude code or an agent
claude

# Run the process
/hx:operator
```

## Usage

See the [docs/](docs/) folder for detailed command reference.

## Output Format

All responses use a JSON envelope:

| Field     | Type    | Description                          |
|-----------|---------|--------------------------------------|
| `success` | bool    | `true` on success, `false` on error  |
| `data`    | object  | Present on success                   |
| `error`   | string  | Present on failure                   |

Exit code is `0` on success, `1` on error.

## Data Layout

### Repository Structure

```
harnessx/                                   # Repository root
├── CLAUDE.md                               # Agent instructions (AGENTS.md for Cursor)
├── README.md                               # Crate README (embedded into harnessx/ on init)
├── Cargo.toml                              # Rust crate manifest
├── Cargo.lock
├── CHANGELOG.md
├── LICENSE
├── skills-lock.json                        # Installed skill version tracking
├── .claude/                                # Claude Code config (.cursor/ for Cursor)
│   ├── settings.local.json
│   ├── hooks/
│   │   ├── commit-and-push.sh
│   │   └── session-start.sh
│   └── skills/                             # Agent skills (40+)
│       ├── hx-operator/                    # Pipeline orchestrator
│       ├── hx-intake-onboarding/           # Intake coordinator
│       ├── hx-intake-onboarding-goal/      # Goal section
│       ├── hx-intake-onboarding-scope/     # Scope section
│       ├── hx-intake-onboarding-user-knowledge/
│       ├── hx-intake-onboarding-resources/
│       ├── hx-intake-onboarding-success-measures/
│       ├── hx-intake-onboarding-uat/
│       ├── hx-intake-team/                 # Team define/build/interview coordinator
│       ├── hx-intake-team-interviewing/    # Agent interviews
│       ├── hx-intake-completion/           # Exploration/ideation/risk coordinator
│       ├── hx-intake-completion-exploration/
│       ├── hx-intake-completion-ideation/
│       ├── hx-intake-completion-project-risk/
│       ├── hx-intake-actions-writing/      # Action item creation
│       ├── hx-planning/                    # Planning coordinator
│       ├── hx-planning-milestones/
│       ├── hx-planning-epics/
│       ├── hx-planning-stories/
│       ├── hx-planning-tasks/
│       ├── hx-review/                      # Plan review gate
│       ├── hx-execution-next-task/         # Task execution engine
│       ├── hx-milestone-rework-assessment/ # Milestone review
│       ├── hx-milestone-rework-verification/
│       ├── hx-user-acceptance/             # UAT walkthrough
│       ├── hx-uat-rework/                  # UAT rework planning
│       ├── hx-user-troubleshooting/        # Failure loop handler
│       ├── hx-tag-context-reading/         # Tag traceability reader
│       ├── hx-tag-context-writing/         # Tag traceability writer
│       ├── rust-team-coordinator/          # Rust dev orchestrator
│       ├── rust-exploration-and-planning/
│       ├── rust-planning-and-architecture/
│       ├── rust-developing/
│       ├── rust-unit-testing/
│       ├── rust-integration-testing/
│       ├── rust-ergonomic-refactoring/
│       ├── rust-errors-management/
│       ├── rust-commenting/
│       └── ...                             # Additional domain skills
├── docs/                                   # CLI command reference
│   ├── README.md                           # End-to-end process walkthrough
│   ├── projects.md
│   ├── progress.md
│   ├── intake-onboarding.md
│   ├── intake-team.md
│   ├── intake-completion.md
│   ├── intake-actions.md
│   ├── context.md
│   ├── planning-milestones.md
│   ├── planning-epics.md
│   ├── planning-stories.md
│   └── planning-tasks.md
└── src/                                    # Rust source
    ├── bin/main.rs                         # CLI entry point
    └── lib/
        ├── lib.rs
        ├── errors.rs
        ├── output.rs
        ├── templates.rs                    # Embedded template manifest
        ├── commands/                       # CLI command implementations
        │   ├── init.rs
        │   ├── project.rs
        │   ├── progress.rs
        │   ├── intake_onboarding.rs
        │   ├── intake_team.rs
        │   ├── intake_completion.rs
        │   ├── intake_actions.rs
        │   ├── context.rs
        │   ├── planning.rs
        │   ├── planning_milestones.rs
        │   ├── planning_epics.rs
        │   ├── planning_stories.rs
        │   ├── planning_tasks.rs
        │   └── completion.rs
        └── models/                         # Data models (serde structs)
            ├── project.rs
            ├── progress.rs
            ├── status.rs
            ├── intake_onboarding.rs
            ├── intake_team.rs
            ├── intake_completion.rs
            ├── intake_actions.rs
            ├── planning.rs
            ├── planning_milestones.rs
            ├── planning_epics.rs
            ├── planning_stories.rs
            └── planning_tasks.rs
```

### Project Data Structure

When `harnessx init` runs, it scaffolds a `harnessx/` data directory in the target workspace. When a project is created, it gets its own subdirectory. All project state lives in JSON files managed exclusively by the CLI.

```
harnessx/                                   # Data root (in target workspace)
├── README.md                               # Embedded crate README
├── docs/                                   # Embedded CLI command reference
│   └── ...                                 # (same docs/ as repo, copied on init)
├── projects.json                           # Project registry (active + inactive)
└── <project-id>/                           # Per-project directory
    ├── progress.json                       # Pipeline stage tracking (10 stages)
    ├── history.md                          # Execution log (appended after each task)
    ├── uat_feedback.md                     # UAT feedback (written during user acceptance)
    ├── intake/
    │   ├── intake_onboarding.json          # Onboarding section progress (6 sections)
    │   ├── intake_team.json                # Team section progress (3 sections)
    │   ├── intake_completion.json          # Completion section progress (3 sections)
    │   ├── intake_actions.json             # Action items list
    │   ├── goal.md                         # Narrative: project goal
    │   ├── scope.md                        # Narrative: scope boundaries
    │   ├── user_knowledge.md               # Narrative: user expertise
    │   ├── resources.md                    # Narrative: collected resources
    │   ├── success_measures.md             # Narrative: success criteria
    │   ├── user_acceptance_testing.md      # Narrative: UAT plan
    │   └── interview-<skill-kebab>.md      # Narrative: agent interview (one per agent)
    ├── planning/
    │   ├── planning.json                   # Planning section progress (milestones + tasks)
    │   ├── planning_milestones.json        # Milestone definitions
    │   ├── planning_epics.json             # v1 legacy: epic definitions
    │   ├── planning_stories.json           # v1 legacy: story definitions
    │   └── tasks/                          # Task definitions (sharded)
    │       └── <milestone-id>/             # v2: sharded by milestone
    │           └── planning_tasks.json
    │       └── <epic-id>/                  # v1 legacy: sharded by epic/story
    │           └── <story-id>/
    │               └── planning_tasks.json
    └── integration-tests/
        └── failing.md                      # Failure reports (triggers troubleshooting loop)
```

#### Key relationships

- **progress.json** tracks 10 pipeline stages: `user_input_required` → `intake_onboarding` → `intake_team` → `intake_completion` → `planning` → `review` → `execution` → `user_acceptance` → `uat_rework` → `complete`
- **intake_onboarding.json** tracks 6 sections: `goal`, `scope`, `user_knowledge`, `resources`, `success_measures`, `user_acceptance_testing` — each produces a matching `.md` narrative
- **intake_team.json** tracks 3 sections: `team_define`, `team_build`, `team_interview`
- **intake_completion.json** tracks 3 sections: `exploration`, `ideation`, `project_risk_manager`
- **planning.json** tracks 2 active sections: `milestones` and `tasks` (v1 legacy also has `epics` and `stories`)
- **Planning hierarchy (v2)**: milestones → tasks (tasks reference parent milestone directly via `#milestone-N`, with optional `group` labels for organization)
- **Planning hierarchy (v1 legacy)**: milestones → epics → stories → tasks (each level references its parent via tag)
- **Traceability**: all planning artifacts carry `traces` linking back to action items (`#action-N`) and intake sections (`#intake-*`)
