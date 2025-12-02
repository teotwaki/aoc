use common::{DigitString, DigitStringU128};
use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("digit_string_add", |b| {
        let mut ds = DigitString::new();

        b.iter(|| {
            for _ in 0..10 {
                ds.push(u64::MAX);
            }
        })
    });
    c.bench_function("digit_string_u128", |b| {
        let mut ds = DigitStringU128::new();

        b.iter(|| {
            for _ in 0..10 {
                ds.push(u128::MAX);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
