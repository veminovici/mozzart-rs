use mozzart_core::{ScalePattern, ScaleType, Interval};
use mozzart_core::constants::*;

pub struct MajorScaleType;
impl ScaleType for MajorScaleType {}

pub struct MajorScalePattern;
impl ScalePattern for MajorScalePattern {
    type Pattern = [Interval; 7];
    const PATTERN: Self::Pattern = [
        PERFECT_UNISON,
        MAJOR_SECOND,
        MINOR_THIRD,
        PERFECT_FOURTH,
        PERFECT_FIFTH,
        MAJOR_SIXTH,
        MAJOR_SEVENTH,
    ];

    type ScaleTyp = MajorScaleType;
    const TYPE: Self::ScaleTyp = MajorScaleType;
}