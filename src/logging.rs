use anyhow::{anyhow, Result};
use clap::ValueEnum;
use tracing_subscriber::{fmt, EnvFilter};

/// Log formatting modes supported by the binary.
#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum LoggingMode {
    Human,
    Json,
}

/// Initialise tracing/logging based on the requested mode and feature flags.
pub fn init_logging(mode: LoggingMode) -> Result<()> {
    match mode {
        LoggingMode::Human => init_human(),
        LoggingMode::Json => init_json(),
    }
}

#[cfg(feature = "human-logs")]
fn init_human() -> Result<()> {
    fmt()
        .with_env_filter(env_filter())
        .with_target(false)
        .try_init()
        .map_err(|err| anyhow!("failed to install human log subscriber: {err}"))
}

#[cfg(not(feature = "human-logs"))]
fn init_human() -> Result<()> {
    Err(anyhow!("human logging support is disabled at compile time"))
}

#[cfg(feature = "json-logs")]
fn init_json() -> Result<()> {
    fmt()
        .json()
        .with_env_filter(env_filter())
        .with_target(false)
        .try_init()
        .map_err(|err| anyhow!("failed to install json log subscriber: {err}"))
}

#[cfg(not(feature = "json-logs"))]
fn init_json() -> Result<()> {
    Err(anyhow!("json logging support is disabled at compile time"))
}

fn env_filter() -> EnvFilter {
    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("monadic_pipeline=info"))
}
