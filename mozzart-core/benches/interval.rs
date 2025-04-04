use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mozzart_core::interval::constants::*;

fn interval_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Interval Operations");

    // Benchmark interval creation and basic operations
    group.bench_function("create_interval", |b| {
        b.iter(|| {
            let interval = PERFECT_FOURTH;
            black_box(interval);
        });
    });

    group.bench_function("semitones", |b| {
        let interval = PERFECT_FIFTH;
        b.iter(|| {
            black_box(interval.semitones());
        });
    });

    // Benchmark common interval constants
    group.bench_function("perfect_unison", |b| {
        b.iter(|| {
            black_box(PERFECT_UNISON);
        });
    });

    group.bench_function("major_third", |b| {
        b.iter(|| {
            black_box(MAJOR_THIRD);
        });
    });

    group.bench_function("perfect_fifth", |b| {
        b.iter(|| {
            black_box(PERFECT_FIFTH);
        });
    });

    group.bench_function("perfect_octave", |b| {
        b.iter(|| {
            black_box(PERFECT_OCTAVE);
        });
    });
}

criterion_group!(benches, interval_benchmarks);
criterion_main!(benches); 