use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a parsed user prior to enrichment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub age: u8,
    pub email: String,
}

/// Represents additional context derived from the raw user data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnrichedUser {
    pub user: User,
    pub age_group: AgeGroup,
    pub username: String,
}

/// Human friendly bucket describing a user's age segment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgeGroup {
    label: String,
}

impl AgeGroup {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

impl fmt::Display for AgeGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.label)
    }
}

/// Strategy used for deriving age groups.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ValueEnum, Default)]
#[serde(rename_all = "kebab-case")]
pub enum AgeGroupingMode {
    #[value(alias = "default")]
    #[default]
    Default,
    #[value(alias = "fine")]
    FineGrained,
    Wide,
}

/// Errors produced during pipeline processing.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PipelineError {
    #[error("failed to parse line: {reason}")]
    Parse { reason: String },
    #[error("name must not be empty")]
    EmptyName,
    #[error("age {age} is below configured minimum {min_age}")]
    InvalidAge { age: u8, min_age: u8 },
    #[error("age {age} exceeds supported upper bound")]
    AgeOutOfRange { age: u8 },
    #[error("invalid email address: {email}")]
    InvalidEmail { email: String },
}
