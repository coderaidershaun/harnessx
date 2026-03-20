//! `harnessx init` — scaffold the harness in the current directory.

use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process;

use crate::templates;

#[derive(clap::Args)]
pub struct InitArgs {
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
        let manifest = templates::manifest();

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

        println!();
        println!("harnessx initialized: {created} created, {skipped} skipped.");
        Ok(())
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
