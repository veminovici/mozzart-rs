use mozzart_core::constants::*;
use mozzart_core::{Interval, ScalePattern, ScaleType};

pub struct MelodicMinorScaleType;
impl ScaleType for MelodicMinorScaleType {}

pub struct MelodicMinorScalePattern;
impl ScalePattern for MelodicMinorScalePattern {
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

    type ScaleTyp = MelodicMinorScaleType;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mozzart_core::ScalePattern;

    #[test]
    fn test_melodic_minor_scale() {
        let scale = MelodicMinorScalePattern::apply(C4);
        assert_eq!(scale.len(), 7);
        assert_eq!(scale[0], C4);
        assert_eq!(scale[1], D4);
        assert_eq!(scale[2], EFLAT4);
        assert_eq!(scale[3], F4);
        assert_eq!(scale[4], G4);
        assert_eq!(scale[5], A4);
        assert_eq!(scale[6], B4);
    }
}
