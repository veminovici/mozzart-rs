use mozzart_core::constants::*;
use mozzart_core::{Interval, ScalePattern, ScaleType};

pub struct HarmonicMinorScaleType;
impl ScaleType for HarmonicMinorScaleType {}

pub struct HarmonicMinorScalePattern;
impl ScalePattern for HarmonicMinorScalePattern {
    type Pattern = [Interval; 7];
    const PATTERN: Self::Pattern = [
        PERFECT_UNISON,
        MAJOR_SECOND,
        MINOR_THIRD,
        PERFECT_FOURTH,
        PERFECT_FIFTH,
        MINOR_SIXTH,
        MAJOR_SEVENTH,
    ];

    type ScaleTyp = HarmonicMinorScaleType;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mozzart_core::ScalePattern;

    #[test]
    fn test_minor_scale() {
        let scale = HarmonicMinorScalePattern::apply(C4);
        assert_eq!(scale.len(), 7);
        assert_eq!(scale[0], C4);
        assert_eq!(scale[1], D4);
        assert_eq!(scale[2], EFLAT4);
        assert_eq!(scale[3], F4);
        assert_eq!(scale[4], G4);
        assert_eq!(scale[5], AFLAT4);
        assert_eq!(scale[6], B4);
    }
}
