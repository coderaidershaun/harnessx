//! Shared status type for tracking progress across stages and sections.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    #[default]
    NotStarted,
    InProgress,
    Completed,
    Rework,
}

impl Status {
    pub fn is_completed(&self) -> bool {
        *self == Self::Completed
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotStarted => write!(f, "not_started"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Completed => write!(f, "completed"),
            Self::Rework => write!(f, "rework"),
        }
    }
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "not_started" => Ok(Self::NotStarted),
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "rework" => Ok(Self::Rework),
            other => Err(format!(
                "unknown status '{other}' — expected: not_started, in_progress, completed, rework"
            )),
        }
    }
}
