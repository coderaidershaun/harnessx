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

# Create and activate a project
harnessx project create my-project

# Initialise intake and progress tracking
harnessx intake init
harnessx progress init
```

## Usage

```
harnessx <command> <subcommand> [args]
```

| Command          | Description                        |
|------------------|------------------------------------|
| `init`           | Scaffold the harnessx harness      |
| `project`        | Manage projects                    |
| `intake`         | Manage intake progress             |
| `progress`       | Manage project pipeline progress   |
| `intake-actions` | Manage intake action items         |

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

```
harnessx/
  projects.json          # Project registry (active + inactive)
  <project-id>/
    intake_progress.json # Intake section tracking
    progress.json        # Pipeline stage tracking
    intake/
      intake_actions.json # Action items
```
