//! `harnessx init` — scaffold the harness in the current directory.

use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process;

use crate::templates::{self, Agent};

#[derive(clap::Args)]
pub struct InitArgs {
    /// Agent platform to initialise for: `claude` or `cursor`.
    /// Detected automatically from existing CLAUDE.md / AGENTS.md if omitted.
    agent: Option<String>,

    /// Overwrite existing files without prompting.
    #[arg(long)]
    force: bool,
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

        let manifest = templates::manifest(agent);

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
