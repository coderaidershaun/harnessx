//! Intake team progress tracking for projects.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::errors::{ParserError, ParserResult};
use crate::models::intake_onboarding::IntakeItem;
use crate::models::project::ProjectRegistry;

const INTAKE_TEAM_FILE: &str = "intake_team.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IntakeTeamProgress {
    pub team_define: IntakeItem,
    pub team_build: IntakeItem,
    pub team_interview: IntakeItem,
}

impl Default for IntakeTeamProgress {
    fn default() -> Self {
        Self {
            team_define: IntakeItem {
                skills: vec![String::from("hx:intake-team")],
                ..Default::default()
            },
            team_build: IntakeItem {
                skills: vec![String::from("hx:intake-team")],
                ..Default::default()
            },
            team_interview: IntakeItem {
                skills: vec![String::from("hx:intake-team-interviewing")],
                ..Default::default()
            },
        }
    }
}

pub const INTAKE_TEAM_SECTIONS: [&str; 3] = ["team_define", "team_build", "team_interview"];

impl IntakeTeamProgress {
    pub fn items(&self) -> [(&str, &IntakeItem); 3] {
        [
            ("team_define", &self.team_define),
            ("team_build", &self.team_build),
            ("team_interview", &self.team_interview),
        ]
    }

    pub fn item_mut(&mut self, name: &str) -> Option<&mut IntakeItem> {
        match name {
            "team_define" => Some(&mut self.team_define),
            "team_build" => Some(&mut self.team_build),
            "team_interview" => Some(&mut self.team_interview),
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
        format!("harnessx/{project_id}/intake/{INTAKE_TEAM_FILE}")
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
            return Err(ParserError::IntakeTeamAlreadyExists(id.to_string()));
        }

        let progress = Self::default();
        progress.save(id)?;
        Ok(progress)
    }

    pub fn for_active_project() -> ParserResult<Self> {
        let registry = ProjectRegistry::load_or_default()?;
        let id = registry.active_project_id()?;

        if !Path::new(&Self::file_path(id)).exists() {
            return Err(ParserError::IntakeTeamNotFound(id.to_string()));
        }

        Self::load(id)
    }

    pub fn save_for_active_project(&self) -> ParserResult<()> {
        let registry = ProjectRegistry::load_or_default()?;
        let id = registry.active_project_id()?;
        self.save(id)
    }
}
