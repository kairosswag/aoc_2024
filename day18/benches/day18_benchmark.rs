use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../../input/day05");
    c.bench_function("bench day18 real", |b| b.iter(|| day18::run(black_box(&input[..]))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);