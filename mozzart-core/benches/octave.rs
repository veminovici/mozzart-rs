use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mozzart_core::octave::constants::*;

fn octave_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Octave Operations");

    // Benchmark octave creation and basic operations
    group.bench_function("create_octave", |b| {
        b.iter(|| {
            let octave = O4;
            black_box(octave);
        });
    });

    group.bench_function("is_canonical", |b| {
        let octave = O4;
        b.iter(|| {
            black_box(octave.is_canonical());
        });
    });

    // Benchmark common octave constants
    group.bench_function("octave_0", |b| {
        b.iter(|| {
            black_box(O0);
        });
    });

    group.bench_function("octave_4", |b| {
        b.iter(|| {
            black_box(O4);
        });
    });

    group.bench_function("octave_9", |b| {
        b.iter(|| {
            black_box(O9);
        });
    });
}

criterion_group!(benches, octave_benchmarks);
criterion_main!(benches); 