//! Planning stage progress tracking (section-level).

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::errors::{ParserError, ParserResult};
use crate::models::intake_onboarding::IntakeItem;
use crate::models::project::ProjectRegistry;

const PLANNING_FILE: &str = "planning.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanningProgress {
    pub milestones: IntakeItem,
    pub epics: IntakeItem,
    pub stories: IntakeItem,
    pub tasks: IntakeItem,
}

impl Default for PlanningProgress {
    fn default() -> Self {
        Self {
            milestones: IntakeItem {
                skills: vec![String::from("hx:planning-milestones")],
                ..Default::default()
            },
            epics: IntakeItem {
                skills: vec![String::from("hx:planning-epics")],
                ..Default::default()
            },
            stories: IntakeItem {
                skills: vec![String::from("hx:planning-stories")],
                ..Default::default()
            },
            tasks: IntakeItem {
                skills: vec![String::from("hx:planning-tasks")],
                ..Default::default()
            },
        }
    }
}

/// Canonical section order; do not reorder without user approval.
pub const PLANNING_SECTIONS: [&str; 4] = [
    "milestones",
    "epics",
    "stories",
    "tasks",
];

impl PlanningProgress {
    /// Returns an ordered snapshot of all sections and their items.
    pub fn items(&self) -> [(&str, &IntakeItem); 4] {
        [
            ("milestones", &self.milestones),
            ("epics", &self.epics),
            ("stories", &self.stories),
            ("tasks", &self.tasks),
        ]
    }

    pub fn item_mut(&mut self, name: &str) -> Option<&mut IntakeItem> {
        match name {
            "milestones" => Some(&mut self.milestones),
            "epics" => Some(&mut self.epics),
            "stories" => Some(&mut self.stories),
            "tasks" => Some(&mut self.tasks),
            _ => None,
        }
    }

    /// Returns the name of the next incomplete section, or `None` if all are done.
    pub fn next_item(&self) -> Option<&str> {
        self.items()
            .into_iter()
            .find(|(_, item)| !item.status.is_completed())
            .map(|(name, _)| name)
    }

    fn file_path(project_id: &str) -> String {
        format!("harnessx/{project_id}/planning/{PLANNING_FILE}")
    }

    pub fn load(project_id: &str) -> ParserResult<Self> {
        let contents = fs::read_to_string(Self::file_path(project_id))?;
        Ok(serde_json::from_str(&contents)?)
    }

    pub fn load_or_default(project_id: &str) -> Self {
        Self::load(project_id).unwrap_or_default()
    }

    pub fn save(&self, project_id: &str) -> ParserResult<()> {
        let path = Self::file_path(project_id);
        if let Some(parent) = Path::new(&path).parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    /// Creates a default planning file for the active project. Errors if one already exists.
    pub fn init_for_active_project() -> ParserResult<Self> {
        let registry = ProjectRegistry::load_or_default()?;
        let id = registry.active_project_id()?;

        if Path::new(&Self::file_path(id)).exists() {
            return Err(ParserError::PlanningAlreadyExists(id.to_string()));
        }

        let progress = Self::default();
        progress.save(id)?;
        Ok(progress)
    }

    pub fn for_active_project() -> ParserResult<Self> {
        let registry = ProjectRegistry::load_or_default()?;
        let id = registry.active_project_id()?;

        if !Path::new(&Self::file_path(id)).exists() {
            return Err(ParserError::PlanningNotFound(id.to_string()));
        }

        Self::load(id)
    }

    pub fn save_for_active_project(&self) -> ParserResult<()> {
        let registry = ProjectRegistry::load_or_default()?;
        let id = registry.active_project_id()?;
        self.save(id)
    }
}
