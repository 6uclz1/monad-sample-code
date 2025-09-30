#![deny(unsafe_code)]

use anyhow::{Context, Result};
use clap::Parser;
use monadic_pipeline::{
    init_logging, process_lines, AgeGroupingMode, LoggingMode, ValidationConfig,
};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use tracing::{info, warn};

#[derive(Debug, Parser)]
#[command(name = "monadic-pipeline", version, about = "Monadic pipeline demo for CSV-like data", long_about = None)]
struct Cli {
    /// Input source: file path, directory, or '-' for stdin.
    #[arg(long = "in", value_name = "PATH", default_value = "-")]
    input: String,

    /// Optional output file (defaults to stdout).
    #[arg(long = "out", value_name = "FILE")]
    output: Option<PathBuf>,

    /// Minimum allowed age.
    #[arg(long = "min-age", value_name = "AGE")]
    min_age: Option<u8>,

    /// Enforce strict email validation using a regex.
    #[arg(long = "strict-email")]
    strict_email: bool,

    /// Age grouping strategy.
    #[arg(long = "age-grouping", value_enum)]
    age_grouping: Option<AgeGroupingMode>,

    /// Logging output format.
    #[arg(long = "log", value_enum)]
    log: Option<LoggingMode>,

    /// Hint for parallelism (currently informational only).
    #[arg(long = "parallel", value_name = "N", default_value_t = 0)]
    parallel: usize,
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{err:?}");
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let cli = Cli::parse();

    let logging_mode = cli.log.unwrap_or_else(default_logging_mode);
    init_logging(logging_mode).context("failed to initialise logging")?;

    if cli.parallel > 1 {
        warn!(
            requested = cli.parallel,
            "parallel flag is currently informational; running sequentially"
        );
    }

    let mut cfg = ValidationConfig::default();
    if let Some(min_age) = cli.min_age {
        cfg.min_age = min_age;
    }
    cfg.strict_email = cli.strict_email;
    if let Some(mode) = cli.age_grouping {
        cfg.age_grouping = mode;
    }

    let lines = read_input(&cli.input)?;
    let line_count = lines.len();
    info!(lines = line_count, "loaded input lines");
    let outputs = process_lines(lines, &cfg).context("pipeline execution failed")?;

    write_output(cli.output.as_deref(), &outputs)?;
    Ok(())
}

fn default_logging_mode() -> LoggingMode {
    if cfg!(feature = "human-logs") {
        LoggingMode::Human
    } else if cfg!(feature = "json-logs") {
        LoggingMode::Json
    } else {
        LoggingMode::Human
    }
}

fn read_input(source: &str) -> Result<Vec<String>> {
    if source == "-" {
        read_from_stdin()
    } else {
        let path = Path::new(source);
        if path.is_dir() {
            read_from_directory(path)
        } else {
            read_from_file(path)
        }
    }
}

fn read_from_stdin() -> Result<Vec<String>> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .context("failed to read stdin")?;
    Ok(lines
        .into_iter()
        .map(|line| line.trim_end().to_owned())
        .filter(|line| !line.is_empty())
        .collect())
}

fn read_from_file(path: &Path) -> Result<Vec<String>> {
    let file = File::open(path)
        .with_context(|| format!("failed to open input file {}", path.display()))?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .with_context(|| format!("failed to read input file {}", path.display()))?;
    Ok(lines
        .into_iter()
        .map(|line| line.trim_end().to_owned())
        .filter(|line| !line.is_empty())
        .collect())
}

fn read_from_directory(path: &Path) -> Result<Vec<String>> {
    let mut files: Vec<PathBuf> = fs::read_dir(path)
        .with_context(|| format!("failed to read directory {}", path.display()))?
        .map(|entry| entry.with_context(|| "failed to access directory entry".to_string()))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|entry| entry.path())
        .filter(|p| p.is_file())
        .collect();

    files.sort();

    let mut lines = Vec::new();
    for file in files {
        match file.extension().and_then(|ext| ext.to_str()) {
            Some(ext) if matches!(ext.to_ascii_lowercase().as_str(), "csv" | "txt") => {
                lines.extend(read_from_file(&file)?);
            }
            _ => {
                warn!(file = %file.display(), "skipping unsupported file");
            }
        }
    }

    Ok(lines)
}

fn write_output(path: Option<&Path>, lines: &[String]) -> Result<()> {
    match path {
        Some(path) => {
            let mut file = File::create(path)
                .with_context(|| format!("failed to create output file {}", path.display()))?;
            for line in lines {
                writeln!(file, "{line}").context("failed to write output line")?;
            }
            file.flush().context("failed to flush output file")?;
            Ok(())
        }
        None => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            for line in lines {
                writeln!(handle, "{line}").context("failed to write to stdout")?;
            }
            handle.flush().context("failed to flush stdout")
        }
    }
}
