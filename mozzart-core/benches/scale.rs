use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mozzart_core::interval::constants::*;
use mozzart_core::pitch::constants::*;
use mozzart_core::{Interval, ScalePattern, ScaleType};

struct MajorScaleType;
impl ScaleType for MajorScaleType {
    fn name() -> &'static str {
        "major"
    }
}

struct MajorScalePattern;
impl ScalePattern for MajorScalePattern {
    type Pattern = [Interval; 7];
    const PATTERN: Self::Pattern = [
        MAJOR_SECOND,
        MAJOR_SECOND,
        MINOR_SECOND,
        MAJOR_SECOND,
        MAJOR_SECOND,
        MAJOR_SECOND,
        MINOR_SECOND,
    ];
    type ScaleTyp = MajorScaleType;
}

fn scale_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Scale Operations");

    // Benchmark scale pattern application
    group.bench_function("apply_major_scale", |b| {
        let root = C4;
        b.iter(|| {
            black_box(MajorScalePattern::apply(black_box(root)));
        });
    });

    // Benchmark scale pattern with different roots
    group.bench_function("apply_major_scale_c4", |b| {
        b.iter(|| {
            black_box(MajorScalePattern::apply(C4));
        });
    });

    group.bench_function("apply_major_scale_g4", |b| {
        b.iter(|| {
            black_box(MajorScalePattern::apply(G4));
        });
    });

    group.bench_function("apply_major_scale_f4", |b| {
        b.iter(|| {
            black_box(MajorScalePattern::apply(F4));
        });
    });
}

criterion_group!(benches, scale_benchmarks);
criterion_main!(benches);
