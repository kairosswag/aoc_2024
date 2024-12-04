
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day04;
pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../../input/day04");
    c.bench_function("bench day04 real", |b| b.iter(|| day04::run(black_box(&input[..]))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);