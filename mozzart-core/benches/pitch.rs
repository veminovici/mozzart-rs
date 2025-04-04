use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mozzart_core::interval::constants::*;
use mozzart_core::pitch::constants::*;

fn pitch_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Pitch Operations");

    // Benchmark pitch creation and basic operations
    group.bench_function("create_pitch", |b| {
        b.iter(|| {
            let pitch = C4;
            black_box(pitch);
        });
    });

    group.bench_function("semitones", |b| {
        let pitch = C4;
        b.iter(|| {
            black_box(pitch.semitones());
        });
    });

    group.bench_function("canonical", |b| {
        let pitch = C4;
        b.iter(|| {
            black_box(pitch.canonical());
        });
    });

    group.bench_function("is_canonical", |b| {
        let pitch = C4;
        b.iter(|| {
            black_box(pitch.is_canonical());
        });
    });

    group.bench_function("octave", |b| {
        let pitch = C4;
        b.iter(|| {
            black_box(pitch.octave());
        });
    });

    // Benchmark transposition with different intervals
    group.bench_function("transpose_perfect_fifth", |b| {
        let pitch = C4;
        b.iter(|| {
            black_box(pitch.transpose(PERFECT_FIFTH));
        });
    });

    group.bench_function("transpose_major_third", |b| {
        let pitch = C4;
        b.iter(|| {
            black_box(pitch.transpose(MAJOR_THIRD));
        });
    });

    group.bench_function("transpose_octave", |b| {
        let pitch = C4;
        b.iter(|| {
            black_box(pitch.transpose(PERFECT_OCTAVE));
        });
    });
}

criterion_group!(benches, pitch_benchmarks);
criterion_main!(benches);
