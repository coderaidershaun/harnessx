//! Intake completion progress tracking for projects.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::errors::{ParserError, ParserResult};
use crate::models::intake_onboarding::IntakeItem;
use crate::models::project::ProjectRegistry;

const INTAKE_COMPLETION_FILE: &str = "intake_completion.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IntakeCompletionProgress {
    pub exploration: IntakeItem,
    pub ideation: IntakeItem,
    pub project_risk_manager: IntakeItem,
}

impl Default for IntakeCompletionProgress {
    fn default() -> Self {
        Self {
            exploration: IntakeItem {
                skills: vec!["hx:intake-completion-exploration".into()],
                ..Default::default()
            },
            ideation: IntakeItem {
                skills: vec!["hx:intake-completion-ideation".into()],
                ..Default::default()
            },
            project_risk_manager: IntakeItem::default(),
        }
    }
}

pub const INTAKE_COMPLETION_SECTIONS: [&str; 3] =
    ["exploration", "ideation", "project_risk_manager"];

impl IntakeCompletionProgress {
    pub fn items(&self) -> [(&str, &IntakeItem); 3] {
        [
            ("exploration", &self.exploration),
            ("ideation", &self.ideation),
            ("project_risk_manager", &self.project_risk_manager),
        ]
    }

    pub fn item_mut(&mut self, name: &str) -> Option<&mut IntakeItem> {
        match name {
            "exploration" => Some(&mut self.exploration),
            "ideation" => Some(&mut self.ideation),
            "project_risk_manager" => Some(&mut self.project_risk_manager),
            _ => None,
        }
    }

    pub fn next_item(&self) -> Option<&str> {
        self.items()
            .into_iter()
            .find(|(_, item)| !item.status.is_completed())
            .map(|(name, _)| name)
    }

    fn file_path(project_id: &str) -> String {
        format!("harnessx/{project_id}/intake/{INTAKE_COMPLETION_FILE}")
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

    pub fn init_for_active_project() -> ParserResult<Self> {
        let registry = ProjectRegistry::load_or_default()?;
        let id = registry.active_project_id()?;

        if Path::new(&Self::file_path(id)).exists() {
            return Err(ParserError::IntakeCompletionAlreadyExists(id.to_string()));
        }

        let progress = Self::default();
        progress.save(id)?;
        Ok(progress)
    }

    pub fn for_active_project() -> ParserResult<Self> {
        let registry = ProjectRegistry::load_or_default()?;
        let id = registry.active_project_id()?;

        if !Path::new(&Self::file_path(id)).exists() {
            return Err(ParserError::IntakeCompletionNotFound(id.to_string()));
        }

        Self::load(id)
    }

    pub fn save_for_active_project(&self) -> ParserResult<()> {
        let registry = ProjectRegistry::load_or_default()?;
        let id = registry.active_project_id()?;
        self.save(id)
    }
}
