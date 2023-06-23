use criterion::{criterion_group, criterion_main, Criterion};

use luhn::{is_valid, is_valid2};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("is valid (check whitespace inside try_fold)", |b| {
        b.iter(|| is_valid("234 567 891 234"))
    });
    c.bench_function("is valid (filter whitespace)", |b| {
        b.iter(|| is_valid2("234 567 891 234"))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
