# Monadic Pipeline

Rust sample application showcasing a monadic style processing pipeline built on top of `Result` combinators. Incoming records are parsed, validated, enriched, and formatted while preserving short-circuit error handling and high observability.

## Features
- Pipeline stages implemented as small pure functions composed with `Result`/`Option`
- Configurable validation (`--min-age`, `--strict-email`, `--age-grouping`)
- Multiple input sources (stdin / file / directory) and outputs (stdout / file)
- Structured logging with human and JSON formats via feature flags
- Instrumentation using `tracing` with metric-style counters
- Criterion benchmark and runnable example
- Unit, property, integration, and CLI tests

## Getting Started
```bash
cargo build
cargo test
cargo fmt -- --check
cargo clippy -- -D warnings
```

Run a single line through the pipeline:
```bash
echo "Alice,30,alice@example.com" | cargo run -- --in - --strict-email
```

Process all `.csv`/`.txt` files in a directory and write to a file with JSON logs:
```bash
cargo run --features json-logs -- --in samples --out out.txt --log json
```

## CLI Flags
- `--in <PATH|->`: Input source (`-` = stdin)
- `--out <PATH>`: Optional output file
- `--min-age <u8>`: Minimum required age
- `--strict-email`: Enable regex-based email validation
- `--age-grouping <default|fine-grained|wide>`: Choose age grouping strategy
- `--log <human|json>`: Select log format
- `--parallel <N>`: Informational hint (sequential processing today)

## Testing Strategy
- Unit & property tests live in `src/lib.rs`
- Integration tests for library (`tests/integration_lib.rs`) and CLI (`tests/integration_cli.rs`)
- Criterion benchmark located at `benches/pipeline_bench.rs`
- Example usage in `examples/basic.rs`

Run everything via `make` helper (optional):
```bash
make check      # fmt + clippy + test
make bench      # criterion smoke bench
```

## Observability
Logging is initialised through `logging::init_logging`. By default the binary builds with human-readable logs; enable the `json-logs` feature for structured output. Metrics-style counters (`lines_total`, `lines_ok`, `lines_err`) are emitted as part of `process_lines` events.
