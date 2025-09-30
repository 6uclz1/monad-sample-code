#![deny(unsafe_code)]

pub mod domain;
pub mod logging;
pub mod pipeline;
pub mod validation;

pub use crate::domain::{AgeGroup, AgeGroupingMode, EnrichedUser, PipelineError, User};
pub use crate::logging::{init_logging, LoggingMode};
pub use crate::pipeline::{process_line, process_lines};
pub use crate::validation::ValidationConfig;

use crate::validation::is_valid_email;
use tracing::instrument;

const MAX_SUPPORTED_AGE: u8 = 120;

/// Parse a single CSV-like line into a `User` struct.
#[instrument(level = "debug", skip(line), fields(line_len = line.len()))]
pub fn parse_line(line: &str) -> Result<User, PipelineError> {
    let mut parts = line.split(',').map(str::trim);
    let name = parts
        .next()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| PipelineError::Parse {
            reason: "missing name field".into(),
        })?;
    let age_str = parts.next().ok_or_else(|| PipelineError::Parse {
        reason: "missing age field".into(),
    })?;
    let email = parts
        .next()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| PipelineError::Parse {
            reason: "missing email field".into(),
        })?;

    if parts.next().is_some() {
        return Err(PipelineError::Parse {
            reason: "too many fields".into(),
        });
    }

    let age: u8 = age_str.parse().map_err(|_| PipelineError::Parse {
        reason: format!("invalid age `{age_str}`"),
    })?;

    Ok(User {
        name: name.to_owned(),
        age,
        email: email.to_owned(),
    })
}

/// Apply validation rules to the parsed user.
#[instrument(level = "debug", skip(cfg))]
pub fn validate_user(mut user: User, cfg: &ValidationConfig) -> Result<User, PipelineError> {
    user.name = user.name.trim().to_owned();
    if user.name.is_empty() {
        return Err(PipelineError::EmptyName);
    }

    if user.age < cfg.min_age {
        return Err(PipelineError::InvalidAge {
            age: user.age,
            min_age: cfg.min_age,
        });
    }

    if user.age > MAX_SUPPORTED_AGE {
        return Err(PipelineError::AgeOutOfRange { age: user.age });
    }

    if !is_valid_email(&user.email, cfg.strict_email) {
        return Err(PipelineError::InvalidEmail {
            email: mask_email(&user.email),
        });
    }

    Ok(user)
}

/// Annotate the user with derived information such as age group and username.
#[instrument(level = "debug", skip(user))]
pub fn enrich_user(user: User) -> EnrichedUser {
    enrich_user_with_mode(user, AgeGroupingMode::Default)
}

pub(crate) fn enrich_user_with_mode(user: User, mode: AgeGroupingMode) -> EnrichedUser {
    let age_group = compute_age_group(user.age, mode);
    let username = generate_username(&user);
    EnrichedUser {
        user,
        age_group,
        username,
    }
}

fn compute_age_group(age: u8, mode: AgeGroupingMode) -> AgeGroup {
    match mode {
        AgeGroupingMode::Default => {
            let label = match age {
                0..=12 => "<teen",
                13..=19 => "teens",
                20..=29 => "20s",
                30..=39 => "30s",
                40..=49 => "40s",
                _ => "50+",
            };
            AgeGroup::new(label)
        }
        AgeGroupingMode::FineGrained => {
            let start = age / 5 * 5;
            let end = (start + 4).min(MAX_SUPPORTED_AGE);
            AgeGroup::new(format!("{}-{}", start, end))
        }
        AgeGroupingMode::Wide => {
            let label = match age {
                0..=17 => "young",
                18..=45 => "adult",
                _ => "senior",
            };
            AgeGroup::new(label)
        }
    }
}

fn generate_username(user: &User) -> String {
    let mut raw = user
        .name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == ' ')
        .collect::<String>()
        .to_ascii_lowercase();
    raw.retain(|c| c.is_ascii_alphanumeric());
    if raw.is_empty() {
        user.email
            .split('@')
            .next()
            .map(|local| local.to_ascii_lowercase())
            .unwrap_or_else(|| "user".to_string())
    } else {
        raw
    }
}

/// Format the enriched user for display or downstream consumption.
#[instrument(level = "debug")]
pub fn format_user(enriched: &EnrichedUser) -> String {
    format!(
        "{} ({}, {}) -> username={}",
        enriched.user.name, enriched.user.age, enriched.age_group, enriched.username
    )
}

/// Mask the local part of an email address for logging.
pub fn mask_email(email: &str) -> String {
    let trimmed = email.trim();
    match trimmed.split_once('@') {
        Some((local, domain)) if !local.is_empty() && !domain.is_empty() => {
            let visible = local.chars().next().unwrap_or('*');
            format!("{}***@{}", visible, domain)
        }
        _ => "***".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn parse_line_success() {
        let user = parse_line("Alice,30,alice@example.com").expect("parse should succeed");
        assert_eq!(user.name, "Alice");
        assert_eq!(user.age, 30);
        assert_eq!(user.email, "alice@example.com");
    }

    #[test]
    fn parse_line_rejects_extra_fields() {
        let err = parse_line("Alice,30,alice@example.com,extra").unwrap_err();
        assert!(matches!(err, PipelineError::Parse { .. }));
    }

    #[test]
    fn validate_user_rejects_underage() {
        let cfg = ValidationConfig {
            min_age: 21,
            strict_email: false,
            age_grouping: AgeGroupingMode::Default,
        };
        let user = User {
            name: "Bob".into(),
            age: 18,
            email: "bob@example.com".into(),
        };
        let err = validate_user(user, &cfg).unwrap_err();
        assert!(matches!(err, PipelineError::InvalidAge { .. }));
    }

    #[test]
    fn mask_email_obscures_local_part() {
        assert_eq!(mask_email("user@example.com"), "u***@example.com");
        assert_eq!(mask_email("invalid"), "***");
    }

    #[test]
    fn strict_email_accepts_valid() {
        assert!(validation::is_valid_email("alice@example.com", true));
    }

    proptest! {
        #[test]
        fn parse_line_round_trip(name in "[A-Za-z]{1,16}", age in 0u8..=90, local in "[a-z0-9]{1,8}", domain in "[a-z]{2,10}") {
            let email = format!("{local}@{domain}.com");
            let line = format!("{name},{age},{email}");
            let user = parse_line(&line).expect("valid synthetic input");
            prop_assert_eq!(user.name, name);
            prop_assert_eq!(user.age, age);
            prop_assert_eq!(user.email, email);
        }

        #[test]
        fn strict_email_rejects_invalid(local in "[A-Za-z]{1,6}") {
            let email = local.to_string();
            prop_assume!(!email.contains('@'));
            let cfg = ValidationConfig {
                min_age: 0,
                strict_email: true,
                age_grouping: AgeGroupingMode::Default,
            };
            let user = User {
                name: "Tester".into(),
                age: 30,
                email,
            };
            let result = validate_user(user, &cfg);
            let is_invalid_email = matches!(result, Err(PipelineError::InvalidEmail { .. }));
            prop_assert!(is_invalid_email);
        }
    }
}
