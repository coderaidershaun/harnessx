//! Error types and result alias.

use std::io;

/// All CLI failure modes.
#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("filesystem operation failed: {0}")]
    Io(#[from] io::Error),

    #[error("JSON serialization failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("project '{0}' already exists")]
    DuplicateProject(String),

    #[error("no active project")]
    NoActiveProject,

    #[error("intake-onboarding already initialised for project '{0}'")]
    IntakeOnboardingAlreadyExists(String),

    #[error("no intake-onboarding progress found for project '{0}' — run `intake-onboarding init` first")]
    IntakeOnboardingNotFound(String),

    #[error("intake completion already initialised for project '{0}'")]
    IntakeCompletionAlreadyExists(String),

    #[error("no intake completion progress found for project '{0}' — run `intake-completion init` first")]
    IntakeCompletionNotFound(String),

    #[error("intake team already initialised for project '{0}'")]
    IntakeTeamAlreadyExists(String),

    #[error("no intake team progress found for project '{0}' — run `intake-team init` first")]
    IntakeTeamNotFound(String),

    #[error("progress already initialised for project '{0}'")]
    ProgressAlreadyExists(String),

    #[error("no progress found for project '{0}' — run `progress init` first")]
    ProgressNotFound(String),

    #[error("project '{0}' not found")]
    ProjectNotFound(String),

    #[error("no intake actions found for project '{0}' — run `intake-actions` after project creation")]
    ActionsNotFound(String),

    #[error("action item '{0}' not found")]
    ActionItemNotFound(String),

    #[error("no planning milestones found for project '{0}' — run `planning-milestones create` after project creation")]
    MilestonesNotFound(String),

    #[error("milestone '{0}' not found")]
    MilestoneNotFound(String),

    #[error("invalid enum value: {0}")]
    InvalidEnumValue(String),
}

impl ParserError {
    /// Returns `true` for errors that represent expected empty state rather
    /// than genuine failures (e.g. "no active project" is a normal condition
    /// that the calling skill uses to decide what to do next).
    pub fn is_expected_state(&self) -> bool {
        matches!(self, Self::NoActiveProject)
    }
}

pub type ParserResult<T> = Result<T, ParserError>;
