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
    #[serde(alias = "agent")]
    pub skill: String,
}

/// Ordered list of all pipeline stage names, defining the progression order.
pub const PROGRESS_STAGES: [&str; 9] = [
    "user_input_required",
    "intake_onboarding",
    "intake_team",
    "intake_exploration",
    "planning",
    "review",
    "execution",
    "user_acceptance",
    "complete",
];

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectProgress {
    pub user_input_required: Stage,
    pub intake_onboarding: Stage,
    pub intake_team: Stage,
    pub intake_exploration: Stage,
    pub planning: Stage,
    pub review: Stage,
    pub execution: Stage,
    pub user_acceptance: Stage,
    pub complete: Stage,
}

impl ProjectProgress {
    /// Returns an ordered snapshot of all stages.
    pub fn items(&self) -> [(&str, &Stage); 9] {
        [
            ("user_input_required", &self.user_input_required),
            ("intake_onboarding", &self.intake_onboarding),
            ("intake_team", &self.intake_team),
            ("intake_exploration", &self.intake_exploration),
            ("planning", &self.planning),
            ("review", &self.review),
            ("execution", &self.execution),
            ("user_acceptance", &self.user_acceptance),
            ("complete", &self.complete),
        ]
    }

    pub fn item_mut(&mut self, name: &str) -> Option<&mut Stage> {
        match name {
            "user_input_required" => Some(&mut self.user_input_required),
            "intake_onboarding" => Some(&mut self.intake_onboarding),
            "intake_team" => Some(&mut self.intake_team),
            "intake_exploration" => Some(&mut self.intake_exploration),
            "planning" => Some(&mut self.planning),
            "review" => Some(&mut self.review),
            "execution" => Some(&mut self.execution),
            "user_acceptance" => Some(&mut self.user_acceptance),
            "complete" => Some(&mut self.complete),
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
        format!("harnessx/{project_id}/{PROGRESS_FILE}")
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

    /// Loads progress for the active project, marks the given stage complete, and saves.
    pub fn complete_stage_for_active_project(stage_name: &str) -> ParserResult<()> {
        let mut progress = Self::for_active_project()?;
        if let Some(stage) = progress.item_mut(stage_name) {
            stage.status = Status::Completed;
            progress.save_for_active_project()?;
        }
        Ok(())
    }
}

impl Default for ProjectProgress {
    fn default() -> Self {
        Self {
            user_input_required: Stage {
                status: Status::Completed,
                skill: "hx:user-troubleshooting".into(),
                ..Default::default()
            },
            intake_onboarding: Stage {
                skill: "hx:intake-onboarding".into(),
                ..Default::default()
            },
            intake_team: Stage {
                skill: "hx:intake-team".into(),
                ..Default::default()
            },
            intake_exploration: Stage {
                skill: "hx:intake-exploration".into(),
                ..Default::default()
            },
            planning: Stage {
                skill: "hx:TODO-WARN-USER".into(),
                ..Default::default()
            },
            review: Stage {
                skill: "hx:TODO-WARN-USER".into(),
                ..Default::default()
            },
            execution: Stage {
                skill: "hx:TODO-WARN-USER".into(),
                ..Default::default()
            },
            user_acceptance: Stage {
                skill: "hx:TODO-WARN-USER".into(),
                ..Default::default()
            },
            complete: Stage::default(),
        }
    }
}

/// Return value for the `next` CLI response.
#[derive(Serialize)]
pub struct NextStageResponse {
    pub stage: String,
    pub status: Status,
    pub skill: String,
}
