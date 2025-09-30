use crate::domain::AgeGroupingMode;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Configuration toggles for the validation step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    pub min_age: u8,
    pub strict_email: bool,
    pub age_grouping: AgeGroupingMode,
}

impl ValidationConfig {
    pub fn new(min_age: u8, strict_email: bool, age_grouping: AgeGroupingMode) -> Self {
        Self {
            min_age,
            strict_email,
            age_grouping,
        }
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            min_age: 0,
            strict_email: false,
            age_grouping: AgeGroupingMode::Default,
        }
    }
}

static STRICT_EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$")
        .expect("strict email regex must be valid")
});

/// Validates an email address according to the configured strictness level.
pub fn is_valid_email(email: &str, strict: bool) -> bool {
    let candidate = email.trim();
    if candidate.is_empty() {
        return false;
    }

    if strict {
        STRICT_EMAIL_REGEX.is_match(candidate)
    } else {
        let mut parts = candidate.split('@');
        match (parts.next(), parts.next(), parts.next()) {
            (Some(local), Some(domain), None) => {
                !local.is_empty() && !domain.is_empty() && domain.contains('.')
            }
            _ => false,
        }
    }
}
