use criterion::{criterion_group, criterion_main, Criterion};
use numseries::series::fibonacci::{fibonacci_sequence, nth_fibonacci, nth_fibonacci_memoized};

fn generate_5th_fibonacci(c: &mut Criterion) {
    c.bench_function("generate_5th_fibonacci", |b| {
        b.iter(|| nth_fibonacci(5));
    });
}

fn generate_100th_fibonacci(c: &mut Criterion) {
    c.bench_function("generate_100th_fibonacci", |b| {
        b.iter(|| nth_fibonacci(100));
    });
}

fn generate_5th_fibonacci_memoized(c: &mut Criterion) {
    c.bench_function("generate_5th_fibonacci_memoized", |b| {
        b.iter(|| nth_fibonacci_memoized(5));
    });
}

fn generate_100th_fibonacci_memoized(c: &mut Criterion) {
    c.bench_function("generate_100th_fibonacci_memoized", |b| {
        b.iter(|| nth_fibonacci_memoized(100));
    });
}

fn generate_fibonacci_series_of_500(c: &mut Criterion) {
    c.bench_function("generate_fibonacci_series_of_500", |b| {
        b.iter(|| fibonacci_sequence(500));
    });
}

criterion_group!(
    benches,
    generate_5th_fibonacci,
    generate_100th_fibonacci,
    generate_5th_fibonacci_memoized,
    generate_100th_fibonacci_memoized,
    generate_fibonacci_series_of_500,
);

criterion_main!(benches);
