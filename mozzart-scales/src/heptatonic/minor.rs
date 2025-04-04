use mozzart_core::constants::*;
use mozzart_core::{Interval, ScalePattern, ScaleType};

pub struct MinorScaleType;
impl ScaleType for MinorScaleType {}

pub struct MinorScalePattern;
impl ScalePattern for MinorScalePattern {
    type Pattern = [Interval; 7];
    const PATTERN: Self::Pattern = [
        PERFECT_UNISON,
        MINOR_SECOND,
        MINOR_THIRD,
        PERFECT_FOURTH,
        PERFECT_FIFTH,
        MINOR_SIXTH,
        MINOR_SEVENTH,
    ];

    type ScaleTyp = MinorScaleType;
    const TYPE: Self::ScaleTyp = MinorScaleType;
}
