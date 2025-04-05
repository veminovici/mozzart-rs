use mozzart_core::constants::*;
use mozzart_core::{Interval, ScalePattern, ScaleType};

pub struct BluesScaleType;
impl ScaleType for BluesScaleType {
    fn name() -> &'static str {
        "blues"
    }
}

pub struct BluesScalePattern;
impl ScalePattern for BluesScalePattern {
    type Pattern = [Interval; 6];
    const PATTERN: Self::Pattern = [
        PERFECT_UNISON,
        MINOR_THIRD,
        PERFECT_FOURTH,
        DIMINISHED_FIFTH,
        PERFECT_FIFTH,
        MINOR_SEVENTH,
    ];

    type ScaleTyp = BluesScaleType;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mozzart_core::ScalePattern;

    #[test]
    fn test_blues_scale() {
        let scale = BluesScalePattern::apply(C4);
        let pitches = scale.pitches();
        assert_eq!(pitches.len(), 6);
        assert_eq!(pitches[0], C4);
        assert_eq!(pitches[1], EFLAT4);
        assert_eq!(pitches[2], F4);
        assert_eq!(pitches[3], GFLAT4);
        assert_eq!(pitches[4], G4);
        assert_eq!(pitches[5], BFLAT4);
    }
}
