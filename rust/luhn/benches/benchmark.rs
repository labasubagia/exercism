use criterion::{criterion_group, criterion_main, Criterion};

use luhn::is_valid;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("is valid", |b| b.iter(|| is_valid("234 567 891 234")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
