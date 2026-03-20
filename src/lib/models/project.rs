//! Project data model and registry persistence.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::errors::{ParserError, ParserResult};
use crate::models::intake_onboarding::IntakeOnboardingProgress;
use crate::models::intake_actions;
use crate::models::progress::ProjectProgress;

const PROJECTS_FILE: &str = "harnessx/projects.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub id: SmolStr,
    pub title: SmolStr,
    pub subtitle: String,
    pub description: String,
    pub takeaway_line: String,
    pub directory: String,
    pub user_name: String,
}

/// Holds at most one active project alongside any number of inactive ones.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ProjectRegistry {
    pub active: Option<Project>,
    pub inactive: Vec<Project>,
}

impl ProjectRegistry {
    pub fn load() -> ParserResult<Self> {
        let contents = fs::read_to_string(PROJECTS_FILE)?;
        Ok(serde_json::from_str(&contents)?)
    }

    /// Falls back to an empty default when the file is missing; propagates deserialization errors.
    pub fn load_or_default() -> ParserResult<Self> {
        match fs::read_to_string(PROJECTS_FILE) {
            Ok(contents) => Ok(serde_json::from_str(&contents)?),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn save(&self) -> ParserResult<()> {
        fs::write(PROJECTS_FILE, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    /// Returns `true` if a project with the given ID exists (active or inactive).
    pub fn has_project(&self, id: &str) -> bool {
        self.active.as_ref().is_some_and(|p| p.id == id)
            || self.inactive.iter().any(|p| p.id == id)
    }

    /// Returns the active project's ID, or [`ParserError::NoActiveProject`] if none is set.
    pub fn active_project_id(&self) -> ParserResult<&SmolStr> {
        self.active
            .as_ref()
            .map(|p| &p.id)
            .ok_or(ParserError::NoActiveProject)
    }

    /// Promotes `project` to active, moving any current active project into the inactive list.
    pub fn set_active(&mut self, project: Project) {
        if let Some(previous) = self.active.take() {
            self.inactive.push(previous);
        }
        self.active = Some(project);
    }

    /// Removes a project from the registry (active or inactive) and returns it.
    pub fn remove_project(&mut self, id: &str) -> ParserResult<Project> {
        if self.active.as_ref().is_some_and(|p| p.id == id) {
            return Ok(self.active.take().expect("just verified active is Some"));
        }

        let position = self
            .inactive
            .iter()
            .position(|p| p.id == id)
            .ok_or_else(|| ParserError::ProjectNotFound(id.to_string()))?;

        Ok(self.inactive.remove(position))
    }
}

impl Project {
    pub fn new(id: SmolStr) -> Self {
        Self {
            id,
            title: SmolStr::default(),
            subtitle: String::new(),
            description: String::new(),
            takeaway_line: String::new(),
            directory: String::new(),
            user_name: String::new(),
        }
    }

    /// Creates the project directory, seeds default files, and sets the project as active.
    pub fn create_and_activate(id: SmolStr) -> ParserResult<Self> {
        let mut registry = ProjectRegistry::load_or_default()?;

        if registry.has_project(&id) {
            return Err(ParserError::DuplicateProject(id.to_string()));
        }

        let project = Self::new(id.clone());
        fs::create_dir_all(Path::new("harnessx").join(id.as_str()))?;

        registry.set_active(project.clone());
        registry.save()?;

        IntakeOnboardingProgress::default().save(&id)?;
        ProjectProgress::default().save(&id)?;
        intake_actions::save(&[], &id)?;

        Ok(project)
    }

    fn update_active(mutate_fn: impl FnOnce(&mut Project)) -> ParserResult<Self> {
        let mut registry = ProjectRegistry::load_or_default()?;
        let project = registry.active.as_mut().ok_or(ParserError::NoActiveProject)?;
        mutate_fn(project);
        registry.save()?;
        Ok(registry.active.unwrap())
    }

    pub fn update_title(value: SmolStr) -> ParserResult<Self> {
        Self::update_active(|p| p.title = value)
    }

    pub fn update_subtitle(value: String) -> ParserResult<Self> {
        Self::update_active(|p| p.subtitle = value)
    }

    pub fn update_description(value: String) -> ParserResult<Self> {
        Self::update_active(|p| p.description = value)
    }

    pub fn update_takeaway(value: String) -> ParserResult<Self> {
        Self::update_active(|p| p.takeaway_line = value)
    }

    pub fn update_directory(value: String) -> ParserResult<Self> {
        Self::update_active(|p| p.directory = value)
    }

    pub fn update_username(value: String) -> ParserResult<Self> {
        Self::update_active(|p| p.user_name = value)
    }

    /// Removes a project from the registry and deletes its `harnessx/<id>/` metadata folder only.
    pub fn remove(id: &str) -> ParserResult<Self> {
        let mut registry = ProjectRegistry::load_or_default()?;
        let project = registry.remove_project(id)?;

        let meta_dir = Path::new("harnessx").join(id);
        if meta_dir.exists() {
            fs::remove_dir_all(&meta_dir)?;
        }

        registry.save()?;
        Ok(project)
    }
}
