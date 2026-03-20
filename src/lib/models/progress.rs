//! Project progress tracking across pipeline stages.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::errors::{ParserError, ParserResult};
use crate::models::project::ProjectRegistry;
use crate::models::status::Status;

const PROGRESS_FILE: &str = "progress.json";

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Stage {
    pub status: Status,
    pub command: String,
}

/// Ordered list of all pipeline stage names, defining the progression order.
pub const PROGRESS_STAGES: [&str; 7] = [
    "intake",
    "planning",
    "review",
    "execution",
    "user_acceptance",
    "complete",
    "user_input_required",
];

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ProjectProgress {
    pub intake: Stage,
    pub planning: Stage,
    pub review: Stage,
    pub execution: Stage,
    pub user_acceptance: Stage,
    pub complete: Stage,
    pub user_input_required: Stage,
}

impl ProjectProgress {
    /// Returns an ordered snapshot of all stages.
    pub fn items(&self) -> [(&str, &Stage); 7] {
        [
            ("intake", &self.intake),
            ("planning", &self.planning),
            ("review", &self.review),
            ("execution", &self.execution),
            ("user_acceptance", &self.user_acceptance),
            ("complete", &self.complete),
            ("user_input_required", &self.user_input_required),
        ]
    }

    pub fn item_mut(&mut self, name: &str) -> Option<&mut Stage> {
        match name {
            "intake" => Some(&mut self.intake),
            "planning" => Some(&mut self.planning),
            "review" => Some(&mut self.review),
            "execution" => Some(&mut self.execution),
            "user_acceptance" => Some(&mut self.user_acceptance),
            "complete" => Some(&mut self.complete),
            "user_input_required" => Some(&mut self.user_input_required),
            _ => None,
        }
    }

    /// Returns the name of the next incomplete stage, or `None` if all are done.
    pub fn next_stage(&self) -> Option<&str> {
        self.items()
            .into_iter()
            .find(|(_, stage)| !stage.status.is_completed())
            .map(|(name, _)| name)
    }

    fn file_path(project_id: &str) -> String {
        format!("projects/{project_id}/{PROGRESS_FILE}")
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

    /// Creates a default progress file for the active project. Errors if one already exists.
    pub fn init_for_active_project() -> ParserResult<Self> {
        let registry = ProjectRegistry::load_or_default()?;
        let id = registry.active_project_id()?;

        if Path::new(&Self::file_path(id)).exists() {
            return Err(ParserError::ProgressAlreadyExists(id.to_string()));
        }

        let progress = Self::default();
        progress.save(id)?;
        Ok(progress)
    }

    pub fn for_active_project() -> ParserResult<Self> {
        let registry = ProjectRegistry::load_or_default()?;
        let id = registry.active_project_id()?;

        if !Path::new(&Self::file_path(id)).exists() {
            return Err(ParserError::ProgressNotFound(id.to_string()));
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
pub struct NextStageResponse {
    pub stage: String,
    pub status: Status,
    pub command: String,
}
