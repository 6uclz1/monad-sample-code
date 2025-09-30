use monadic_pipeline::{process_line, process_lines, AgeGroupingMode, ValidationConfig};

fn default_config() -> ValidationConfig {
    ValidationConfig {
        min_age: 0,
        strict_email: true,
        age_grouping: AgeGroupingMode::Default,
    }
}

#[test]
fn process_line_produces_expected_output() {
    let cfg = default_config();
    let out = process_line("Alice,30,alice@example.com", &cfg).expect("pipeline should succeed");
    assert_eq!(out, "Alice (30, 30s) -> username=alice");
}

#[test]
fn process_lines_collects_outputs() {
    let cfg = default_config();
    let inputs = vec![
        "Alice,30,alice@example.com".to_string(),
        "Bob,45,bob@example.com".to_string(),
    ];
    let outputs = process_lines(inputs, &cfg).expect("processing should succeed");
    assert_eq!(outputs.len(), 2);
    assert!(outputs[0].contains("Alice"));
}

#[test]
fn process_lines_short_circuits_on_error() {
    let cfg = ValidationConfig {
        min_age: 40,
        strict_email: true,
        age_grouping: AgeGroupingMode::Default,
    };
    let inputs = vec![
        "Alice,30,alice@example.com".to_string(),
        "Carol,42,carol@example.com".to_string(),
    ];
    let err = process_lines(inputs, &cfg).expect_err("expected validation error");
    assert!(matches!(
        err,
        monadic_pipeline::PipelineError::InvalidAge { .. }
    ));
}
