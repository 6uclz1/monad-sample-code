use criterion::{black_box, criterion_group, criterion_main, Criterion};
use monadic_pipeline::{process_lines, AgeGroupingMode, ValidationConfig};

fn pipeline_benchmark(c: &mut Criterion) {
    let cfg = ValidationConfig {
        min_age: 18,
        strict_email: true,
        age_grouping: AgeGroupingMode::Default,
    };

    let inputs: Vec<String> = (0..1_000)
        .map(|i| format!("User{i},30,user{i}@example.com"))
        .collect();

    c.bench_function("process_lines", |b| {
        b.iter(|| {
            let lines = inputs.clone();
            let result = process_lines(lines, &cfg).expect("benchmark should not fail");
            black_box(result);
        });
    });
}

criterion_group!(benches, pipeline_benchmark);
criterion_main!(benches);
