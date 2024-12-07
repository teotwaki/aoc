use common::utils::{number_length, number_length_successors};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("number_length", |b| b.iter(|| number_length(u64::MAX)));
    c.bench_function("number_length_successors", |b| {
        b.iter(|| number_length_successors(u64::MAX))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
