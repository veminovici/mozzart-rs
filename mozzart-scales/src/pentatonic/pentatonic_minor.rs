use mozzart_core::constants::*;
use mozzart_core::{Interval, ScalePattern, ScaleType};

pub struct PentatonicMinorScaleType;
impl ScaleType for PentatonicMinorScaleType {}

pub struct PentatonicMinorScalePattern;
impl ScalePattern for PentatonicMinorScalePattern {
    type Pattern = [Interval; 5];
    const PATTERN: Self::Pattern = [
        PERFECT_UNISON,
        MINOR_THIRD,
        PERFECT_FOURTH,
        PERFECT_FIFTH,
        MINOR_SEVENTH,
    ];

    type ScaleTyp = PentatonicMinorScaleType;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mozzart_core::ScalePattern;

    #[test]
    fn test_pentatonic_minor_scale() {
        let scale = PentatonicMinorScalePattern::apply(C4);
        assert_eq!(scale.len(), 5);
        assert_eq!(scale[0], C4);
        assert_eq!(scale[1], EFLAT4);
        assert_eq!(scale[2], F4);
        assert_eq!(scale[3], G4);
        assert_eq!(scale[4], BFLAT4);
    }
}
