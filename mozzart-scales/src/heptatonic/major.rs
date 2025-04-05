use mozzart_core::constants::*;
use mozzart_core::{Interval, ScalePattern, ScaleType};

pub struct MajorScaleType;
impl ScaleType for MajorScaleType {}

pub struct MajorScalePattern;
impl ScalePattern for MajorScalePattern {
    type Pattern = [Interval; 7];
    const PATTERN: Self::Pattern = [
        PERFECT_UNISON,
        MAJOR_SECOND,
        MAJOR_THIRD,
        PERFECT_FOURTH,
        PERFECT_FIFTH,
        MAJOR_SIXTH,
        MAJOR_SEVENTH,
    ];

    type ScaleTyp = MajorScaleType;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mozzart_core::ScalePattern;

    #[test]
    fn test_major_scale() {
        let scale = MajorScalePattern::apply(C4);
        assert_eq!(scale.len(), 7);
        assert_eq!(scale[0], C4);
        assert_eq!(scale[1], D4);
        assert_eq!(scale[2], E4);
        assert_eq!(scale[3], F4);
        assert_eq!(scale[4], G4);
        assert_eq!(scale[5], A4);
        assert_eq!(scale[6], B4);
    }
}
