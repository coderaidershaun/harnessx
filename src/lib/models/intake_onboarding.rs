//! Intake onboarding progress tracking for projects.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::errors::{ParserError, ParserResult};
use crate::models::project::ProjectRegistry;
use crate::models::status::Status;

const INTAKE_ONBOARDING_FILE: &str = "intake_progress.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IntakeItem {
    pub status: Status,
    pub agent: String,
    pub skills: Vec<String>,
}

impl Default for IntakeItem {
    fn default() -> Self {
        Self {
            status: Status::NotStarted,
            agent: String::from("opus"),
            skills: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IntakeOnboardingProgress {
    pub goal: IntakeItem,
    pub scope: IntakeItem,
    pub user_knowledge: IntakeItem,
    pub resources: IntakeItem,
    pub success_measures: IntakeItem,
    pub user_acceptance_testing: IntakeItem,
}

impl Default for IntakeOnboardingProgress {
    fn default() -> Self {
        Self {
            goal: IntakeItem {
                skills: vec![String::from("hx:intake-onboarding-goal")],
                ..Default::default()
            },
            scope: IntakeItem {
                skills: vec![String::from("hx:intake-onboarding-scope")],
                ..Default::default()
            },
            user_knowledge: IntakeItem {
                skills: vec![String::from("hx:intake-onboarding-user-knowledge")],
                ..Default::default()
            },
            resources: IntakeItem {
                skills: vec![String::from("hx:intake-onboarding-resources")],
                ..Default::default()
            },
            success_measures: IntakeItem {
                skills: vec![String::from("hx:intake-onboarding-success-measures")],
                ..Default::default()
            },
            user_acceptance_testing: IntakeItem {
                skills: vec![String::from("hx:intake-onboarding-uat")],
                ..Default::default()
            },
        }
    }
}

/// Canonical section order; do not reorder without user approval.
pub const INTAKE_ONBOARDING_SECTIONS: [&str; 6] = [
    "goal",
    "scope",
    "user_knowledge",
    "resources",
    "success_measures",
    "user_acceptance_testing",
];

impl IntakeOnboardingProgress {
    /// Returns an ordered snapshot of all sections and their items.
    pub fn items(&self) -> [(&str, &IntakeItem); 6] {
        [
            ("goal", &self.goal),
            ("scope", &self.scope),
            ("user_knowledge", &self.user_knowledge),
            ("resources", &self.resources),
            ("success_measures", &self.success_measures),
            ("user_acceptance_testing", &self.user_acceptance_testing),
        ]
    }

    pub fn item_mut(&mut self, name: &str) -> Option<&mut IntakeItem> {
        match name {
            "goal" => Some(&mut self.goal),
            "scope" => Some(&mut self.scope),
            "user_knowledge" => Some(&mut self.user_knowledge),
            "resources" => Some(&mut self.resources),
            "success_measures" => Some(&mut self.success_measures),
            "user_acceptance_testing" => Some(&mut self.user_acceptance_testing),
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
        format!("harnessx/{project_id}/intake/{INTAKE_ONBOARDING_FILE}")
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

    /// Creates a default intake onboarding file for the active project. Errors if one already exists.
    pub fn init_for_active_project() -> ParserResult<Self> {
        let registry = ProjectRegistry::load_or_default()?;
        let id = registry.active_project_id()?;

        if Path::new(&Self::file_path(id)).exists() {
            return Err(ParserError::IntakeOnboardingAlreadyExists(id.to_string()));
        }

        let progress = Self::default();
        progress.save(id)?;
        Ok(progress)
    }

    pub fn for_active_project() -> ParserResult<Self> {
        let registry = ProjectRegistry::load_or_default()?;
        let id = registry.active_project_id()?;

        if !Path::new(&Self::file_path(id)).exists() {
            return Err(ParserError::IntakeOnboardingNotFound(id.to_string()));
        }

        Self::load(id)
    }

    pub fn save_for_active_project(&self) -> ParserResult<()> {
        let registry = ProjectRegistry::load_or_default()?;
        let id = registry.active_project_id()?;
        self.save(id)
    }
}

/// Return value for the `next` CLI response.
#[derive(Serialize)]
pub struct NextItemResponse {
    pub section: String,
    pub agent: String,
    pub skills: Vec<String>,
}
