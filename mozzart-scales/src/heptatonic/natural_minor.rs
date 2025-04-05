use mozzart_core::constants::*;
use mozzart_core::{Interval, ScalePattern, ScaleType};

pub struct NaturalMinorScaleType;
impl ScaleType for NaturalMinorScaleType {}

pub struct NaturalMinorScalePattern;
impl ScalePattern for NaturalMinorScalePattern {
    type Pattern = [Interval; 7];
    const PATTERN: Self::Pattern = [
        PERFECT_UNISON,
        MAJOR_SECOND,
        MINOR_THIRD,
        PERFECT_FOURTH,
        PERFECT_FIFTH,
        MINOR_SIXTH,
        MINOR_SEVENTH,
    ];

    type ScaleTyp = NaturalMinorScaleType;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mozzart_core::ScalePattern;

    #[test]
    fn test_natural_minor_scale() {
        let scale = NaturalMinorScalePattern::apply(C4);
        assert_eq!(scale.len(), 7);
        assert_eq!(scale[0], C4);
        assert_eq!(scale[1], D4);
        assert_eq!(scale[2], EFLAT4);
        assert_eq!(scale[3], F4);
        assert_eq!(scale[4], G4);
        assert_eq!(scale[5], AFLAT4);
        assert_eq!(scale[6], BFLAT4);
    }
}
