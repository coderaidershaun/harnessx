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

    #[error("intake already initialised for project '{0}'")]
    IntakeAlreadyExists(String),

    #[error("no intake progress found for project '{0}' — run `intake init` first")]
    IntakeNotFound(String),

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

    #[error("invalid enum value: {0}")]
    InvalidEnumValue(String),
}

pub type ParserResult<T> = Result<T, ParserError>;
