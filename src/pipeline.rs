use crate::domain::PipelineError;
use crate::validation::ValidationConfig;
use crate::{enrich_user_with_mode, format_user, parse_line, validate_user};
use tracing::{error, info, instrument};

/// Runs the full pipeline against a single line of input.
#[instrument(name = "process_line", level = "debug", skip(line, cfg), fields(line_len = line.len()))]
pub fn process_line(line: &str, cfg: &ValidationConfig) -> Result<String, PipelineError> {
    parse_line(line)
        .and_then(|user| validate_user(user, cfg))
        .map(|user| enrich_user_with_mode(user, cfg.age_grouping))
        .map(|enriched| format_user(&enriched))
}

#[derive(Default)]
struct PipelineMetrics {
    lines_total: u64,
    lines_ok: u64,
    lines_err: u64,
}

/// Process multiple lines, short-circuiting on the first failure.
#[instrument(name = "process_lines", level = "info", skip(lines, cfg))]
pub fn process_lines<I>(lines: I, cfg: &ValidationConfig) -> Result<Vec<String>, PipelineError>
where
    I: IntoIterator<Item = String>,
{
    let mut metrics = PipelineMetrics::default();

    let result: Result<Vec<_>, _> = lines
        .into_iter()
        .map(|line| {
            metrics.lines_total += 1;
            match process_line(&line, cfg) {
                Ok(formatted) => {
                    metrics.lines_ok += 1;
                    Ok(formatted)
                }
                Err(err) => {
                    metrics.lines_err += 1;
                    Err(err)
                }
            }
        })
        .collect();

    match result {
        Ok(output) => {
            info!(
                lines_total = metrics.lines_total,
                lines_ok = metrics.lines_ok,
                lines_err = metrics.lines_err,
                "successfully processed lines"
            );
            Ok(output)
        }
        Err(err) => {
            error!(
                lines_total = metrics.lines_total,
                lines_ok = metrics.lines_ok,
                lines_err = metrics.lines_err,
                error = %err,
                "pipeline aborted due to error"
            );
            Err(err)
        }
    }
}
