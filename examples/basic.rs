use monadic_pipeline::{process_line, AgeGroupingMode, ValidationConfig};

fn main() {
    let cfg = ValidationConfig {
        min_age: 18,
        strict_email: true,
        age_grouping: AgeGroupingMode::Default,
    };

    let line = "Alice,30,alice@example.com";
    match process_line(line, &cfg) {
        Ok(output) => println!("{output}"),
        Err(err) => eprintln!("processing failed: {err}"),
    }
}
