//! `harnessx init` — scaffold the harness in a directory.

use std::env;
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
}

/// How to handle files that already exist on disk.
enum ConflictPolicy {
    SkipExisting,
    Overwrite,
}

/// The type of Cargo project to create when not using the current directory.
enum ProjectKind {
    Workspace,
    Library,
    Binary,
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
        self.resolve_directory()?;

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

    /// Ask the user whether to initialise in the current directory or create a
    /// new Cargo project. If a new project is created, the working directory is
    /// changed into it before returning so the rest of `execute` operates there.
    fn resolve_directory(&self) -> io::Result<()> {
        let cwd = env::current_dir()?;
        println!("Use current directory? ({})", cwd.display());
        println!();
        print!("[Y/n]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().to_lowercase().as_str() {
            "" | "y" | "yes" => return Ok(()),
            "n" | "no" => {}
            other => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("unknown selection '{other}' — expected y or n"),
                ));
            }
        }

        // --- Create a new project ------------------------------------------------

        let kind = Self::prompt_project_kind()?;
        let name = Self::prompt_project_name()?;

        match kind {
            ProjectKind::Workspace => {
                fs::create_dir_all(&name)?;
                let cargo_toml = format!(
                    "[workspace]\nresolver = \"2\"\nmembers = []\n"
                );
                fs::write(format!("{name}/Cargo.toml"), cargo_toml)?;
                println!("  created workspace {name}/");
            }
            ProjectKind::Library => {
                let status = Command::new("cargo")
                    .args(["new", "--lib", &name])
                    .status()?;
                if !status.success() {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("cargo new --lib {name} failed"),
                    ));
                }
            }
            ProjectKind::Binary => {
                let status = Command::new("cargo")
                    .args(["new", "--bin", &name])
                    .status()?;
                if !status.success() {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("cargo new --bin {name} failed"),
                    ));
                }
            }
        }

        env::set_current_dir(&name)?;
        println!("  changed directory to {name}/");
        println!();

        Ok(())
    }

    fn prompt_project_kind() -> io::Result<ProjectKind> {
        println!();
        println!("What kind of project?");
        println!();
        println!("  [1] Workspace");
        println!("  [2] Library");
        println!("  [3] Binary");
        println!();
        print!("Select [1/2/3]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" | "workspace" => Ok(ProjectKind::Workspace),
            "2" | "library" | "lib" => Ok(ProjectKind::Library),
            "3" | "binary" | "bin" => Ok(ProjectKind::Binary),
            other => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("unknown selection '{other}' — expected 1, 2, or 3"),
            )),
        }
    }

    fn prompt_project_name() -> io::Result<String> {
        print!("Project name: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let name = input.trim().to_string();
        if name.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "project name cannot be empty",
            ));
        }

        Ok(name)
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
