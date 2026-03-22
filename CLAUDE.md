# How Harnessx Works

Read harnessx/docs/README.md

## Repo Structure

```
harnessx/          # This repo — the CLI crate
├── CLAUDE.md      # Agent instructions
├── README.md      # Crate README (also embedded into target projects on init)
├── Cargo.toml
├── .claude/
│   ├── hooks/     # Session hooks
│   └── skills/    # Agent skills (hx-*, rust-*, etc.)
├── docs/          # CLI command reference (embedded into target projects on init)
│   └── README.md  # End-to-end process walkthrough
└── src/           # Rust source
    ├── bin/        # CLI entry point
    └── lib/
        ├── commands/  # CLI command implementations
        ├── models/    # Data models (serde structs)
        ├── templates.rs  # Embedded template manifest
        ├── errors.rs
        └── output.rs
```

## Data Layout

See the [Project Data Structure](README.md#project-data-structure) section in README.md for the complete `harnessx/<project-id>/` file layout that the CLI creates and manages in target workspaces.

# User Interaction

The user gave their name when setting the active project. If you have it, dont be afraid to use it occasionally when talking to them.
