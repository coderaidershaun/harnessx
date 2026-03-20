//! Project subcommands: create, list, active, activate, remove.

use clap::Subcommand;
use smol_str::SmolStr;

use crate::errors::{ParserError, ParserResult};
use crate::models::project::{Project, ProjectRegistry};
use crate::output::exit_with;

#[derive(Subcommand)]
pub enum ProjectCommand {
    /// Create a new project and set it as active.
    Create { id: String },
    /// List all projects.
    List,
    /// Show the active project.
    Active,
    /// Activate an inactive project.
    Activate { id: String },
    /// Remove a project and its metadata folder (harnessx/<id>/).
    Remove { id: String },
}

impl ProjectCommand {
    pub fn run(self) -> ! {
        match self {
            Self::Create { id } => {
                exit_with(Project::create_and_activate(SmolStr::new(&id)))
            }
            Self::List => {
                exit_with(ProjectRegistry::load_or_default())
            }
            Self::Active => exit_with(active_project()),
            Self::Activate { id } => exit_with(activate_project(&id)),
            Self::Remove { id } => exit_with(Project::remove(&id)),
        }
    }
}

fn active_project() -> ParserResult<Project> {
    let registry = ProjectRegistry::load_or_default()?;
    registry.active.ok_or(ParserError::NoActiveProject)
}

fn activate_project(id: &str) -> ParserResult<Project> {
    let mut registry = ProjectRegistry::load_or_default()?;

    let position = registry
        .inactive
        .iter()
        .position(|p| p.id == id)
        .ok_or_else(|| ParserError::ProjectNotFound(id.to_string()))?;

    let project = registry.inactive.remove(position);
    registry.set_active(project);
    registry.save()?;

    // safe: `set_active` just assigned `Some`
    Ok(registry.active.unwrap())
}
