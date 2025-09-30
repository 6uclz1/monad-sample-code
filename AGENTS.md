# Repository Guidelines

## Project Structure & Module Organization
- `src/` holds the Rust crate modules:
  - `lib.rs` exposes the monadic pipeline stages (`parse`, `validate`, `enrich`, `format`).
  - `domain.rs`, `validation.rs`, `pipeline.rs`, `logging.rs`, and `main.rs` define domain types, validation logic, composition helpers, logging, and the CLI entrypoint.
- `tests/` contains integration suites (`integration_cli.rs`, `integration_lib.rs`) and fixtures under `tests/data/`.
- `benches/` provides Criterion benchmarks; `examples/` includes runnable usage samples.
- `.github/workflows/ci.yml` defines CI; `Makefile` wraps common cargo flows; `README.ja.md` supplies Japanese documentation.

## Build, Test, and Development Commands
- `cargo build` – compile the crate and dependencies.
- `cargo test` – run unit, property, integration, and CLI tests.
- `cargo fmt -- --check` – verify formatting; run `cargo fmt` to auto-format.
- `cargo clippy -- -D warnings` – lint with Clippy, treating warnings as errors.
- `cargo bench -- --sample-size 10` – execute the Criterion smoke benchmark.
- `make check` – shortcut for fmt + clippy + test.

## Coding Style & Naming Conventions
- Rust 2021 edition; `rustfmt` default settings (4-space indent).
- `#![deny(unsafe_code)]` enforced in binaries and libraries.
- Use expressive snake_case function names (`process_line`), UpperCamelCase for types (`ValidationConfig`).
- Keep comments intentional; prefer doc comments (`///`) on public APIs with examples.

## Testing Guidelines
- Unit & property tests live in `src/lib.rs` under `#[cfg(test)]` using `proptest`.
- Integration tests should mirror CLI workflows and live in `tests/`.
- Regenerate fixtures via deterministic commands; ensure `cargo test` stays green before pushing.

## Commit & Pull Request Guidelines
- Follow conventional, descriptive commit messages (e.g., "feat: add strict email validation").
- PRs should describe scope, testing performed (`cargo test`, `cargo fmt`), and link issues when applicable.
- Include CLI output snippets or screenshots when behavior changes.

## Security & Configuration Tips
- No `unsafe` Rust; mask PII via `mask_email` helpers when logging.
- Respect feature flags (`human-logs`, `json-logs`) during testing; default logs should remain human-readable unless `--log json` is requested.
