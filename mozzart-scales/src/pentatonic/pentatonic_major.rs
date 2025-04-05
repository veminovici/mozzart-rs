use mozzart_core::constants::*;
use mozzart_core::{Interval, ScalePattern, ScaleType};

pub struct PentatonicMajorScaleType;
impl ScaleType for PentatonicMajorScaleType {
    fn name() -> &'static str {
        "pentatonic major"
    }
}

pub struct PentatonicMajorScalePattern;
impl ScalePattern for PentatonicMajorScalePattern {
    type Pattern = [Interval; 5];
    const PATTERN: Self::Pattern = [
        PERFECT_UNISON,
        MAJOR_SECOND,
        MAJOR_THIRD,
        PERFECT_FIFTH,
        MAJOR_SIXTH,
    ];

    type ScaleTyp = PentatonicMajorScaleType;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mozzart_core::ScalePattern;

    #[test]
    fn test_pentatonic_major_scale() {
        let scale = PentatonicMajorScalePattern::apply(C4);

        let pitches = scale.pitches();
        assert_eq!(pitches.len(), 5);
        assert_eq!(pitches[0], C4);
        assert_eq!(pitches[1], D4);
        assert_eq!(pitches[2], E4);
        assert_eq!(pitches[3], G4);
        assert_eq!(pitches[4], A4);

        assert_eq!(scale.root(), C4);
        assert_eq!(scale.name(), "pentatonic major");
        assert_eq!(scale.to_string(), "C4 pentatonic major");
    }
}
