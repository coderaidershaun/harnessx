//! `harnessx init` — scaffold the harness in the current directory.

use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{self, Command};

use crate::templates::{self, Agent};

#[derive(clap::Args)]
pub struct InitArgs {
    /// Agent platform to initialise for: `claude` or `cursor`.
    /// Detected automatically from existing CLAUDE.md / AGENTS.md if omitted.
    agent: Option<String>,

    /// Overwrite existing files without prompting.
    #[arg(long)]
    force: bool,

    /// Skip Obsidian vault scaffolding.
    #[arg(long)]
    no_obsidian: bool,
}

/// How to handle files that already exist on disk.
enum ConflictPolicy {
    SkipExisting,
    Overwrite,
}

impl InitArgs {
    pub fn run(self) -> ! {
        if let Err(err) = self.execute() {
            eprintln!("harnessx init failed: {err}");
            process::exit(1);
        }
        process::exit(0);
    }

    fn execute(self) -> io::Result<()> {
        let agent = self.resolve_agent()?;

        let include_obsidian = if self.no_obsidian {
            false
        } else {
            self.resolve_obsidian()?
        };

        let manifest = templates::manifest(agent, include_obsidian);

        let conflicts: Vec<&str> = manifest
            .iter()
            .filter(|f| Path::new(&f.path).exists())
            .map(|f| f.path.as_str())
            .collect();

        let policy = self.resolve_conflict_policy(&conflicts)?;

        fs::create_dir_all("harnessx")?;

        let mut created = 0u32;
        let mut skipped = 0u32;

        for file in &manifest {
            let dest = Path::new(&file.path);

            if dest.exists() && matches!(policy, ConflictPolicy::SkipExisting) {
                println!("  skipped {} (already exists)", file.path);
                skipped += 1;
                continue;
            }

            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(dest, file.content)?;

            if file.path.ends_with(".sh") {
                fs::set_permissions(dest, fs::Permissions::from_mode(0o755))?;
            }

            println!("  created {}", file.path);
            created += 1;
        }

        // Create root md file (CLAUDE.md or AGENTS.md) — never overwrite.
        let root_md = agent.root_md();
        if Path::new(root_md).exists() {
            println!("  skipped {root_md} (already exists)");
            skipped += 1;
        } else {
            fs::write(root_md, templates::root_md_content())?;
            println!("  created {root_md}");
            created += 1;
        }

        println!();
        println!("harnessx initialized ({agent_name}): {created} created, {skipped} skipped.",
            agent_name = match agent {
                Agent::Claude => "claude",
                Agent::Cursor => "cursor",
            },
        );
        Ok(())
    }

    /// Resolve the agent platform.
    ///
    /// 1. Explicit CLI argument → use that.
    /// 2. `CLAUDE.md` exists → Claude.
    /// 3. `AGENTS.md` exists → Cursor.
    /// 4. Neither → prompt the user.
    fn resolve_agent(&self) -> io::Result<Agent> {
        if let Some(arg) = &self.agent {
            return match arg.to_lowercase().as_str() {
                "claude" => Ok(Agent::Claude),
                "cursor" => Ok(Agent::Cursor),
                other => Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("unknown agent '{other}' — expected 'claude' or 'cursor'"),
                )),
            };
        }

        if Path::new("CLAUDE.md").exists() {
            println!("  detected CLAUDE.md — using claude.");
            return Ok(Agent::Claude);
        }

        if Path::new("AGENTS.md").exists() {
            println!("  detected AGENTS.md — using cursor.");
            return Ok(Agent::Cursor);
        }

        println!("Which agent platform are you using?");
        println!();
        println!("  [1] Claude Code");
        println!("  [2] Cursor");
        println!();
        print!("Select [1/2]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" | "claude" => Ok(Agent::Claude),
            "2" | "cursor" => Ok(Agent::Cursor),
            other => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("unknown selection '{other}' — expected 1 or 2"),
            )),
        }
    }

    /// Decide whether to include `.obsidian/` in the scaffold.
    ///
    /// 1. If `.obsidian/` already exists on disk → skip (already set up).
    /// 2. If `obsidian` CLI is on PATH → include it.
    /// 3. Otherwise → prompt the user.
    fn resolve_obsidian(&self) -> io::Result<bool> {
        if Path::new(".obsidian").is_dir() {
            println!("  .obsidian/ already exists, skipping vault scaffolding.");
            return Ok(false);
        }

        // Reject the macOS desktop app (lives inside a .app bundle).
        let has_cli = Command::new("which")
            .arg("obsidian")
            .output()
            .map(|out| {
                out.status.success()
                    && !String::from_utf8_lossy(&out.stdout)
                        .trim()
                        .contains(".app/")
            })
            .unwrap_or(false);

        if has_cli {
            println!("  obsidian-cli detected — including .obsidian/ vault config.");
            return Ok(true);
        }

        println!("obsidian-cli was not detected on your PATH.");
        println!();
        println!("  [y] I want Obsidian CLI support and will install it");
        println!("  [n] Continue without Obsidian (may lead to higher token usage)");
        println!();
        print!("Include .obsidian/ config? [y/n]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "y" | "yes" => {
                println!();
                println!("  Including .obsidian/ config.");
                println!("  Install obsidian-cli: https://github.com/obsidianmd/obsidian-cli");
                Ok(true)
            }
            _ => {
                println!("  Skipping .obsidian/ vault scaffolding.");
                Ok(false)
            }
        }
    }

    fn resolve_conflict_policy(&self, conflicts: &[&str]) -> io::Result<ConflictPolicy> {
        if conflicts.is_empty() {
            return Ok(ConflictPolicy::SkipExisting);
        }
        if self.force {
            return Ok(ConflictPolicy::Overwrite);
        }

        println!("The following files already exist:");
        for path in conflicts {
            println!("  {path}");
        }
        println!();
        print!("[m]erge (skip existing) / [o]verwrite / [e]xit: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "m" | "merge" => Ok(ConflictPolicy::SkipExisting),
            "o" | "overwrite" => Ok(ConflictPolicy::Overwrite),
            _ => {
                println!("Aborted.");
                process::exit(0);
            }
        }
    }
}
