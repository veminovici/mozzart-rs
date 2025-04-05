//! Major scale implementation.
//!
//! The major scale is one of the most common scales in Western music.
//! It is characterized by its bright, happy sound and is used extensively
//! in various musical genres.
//!
//! # Scale Structure
//!
//! The major scale follows the pattern of whole and half steps:
//! ```text
//! W W H W W W H
//! ```
//!
//! Where:
//! - W = Whole step (2 semitones)
//! - H = Half step (1 semitone)
//!
//! # Interval Pattern
//!
//! The major scale is built using the following intervals from the root:
//! ```text
//! Root (0) + Major 2nd (2) + Major 3rd (4) + Perfect 4th (5) +
//! Perfect 5th (7) + Major 6th (9) + Major 7th (11)
//! ```
use mozzart_core::constants::*;
use mozzart_core::{Interval, ScalePattern, ScaleType};

/// A marker type for major scales.
///
/// This type is used to distinguish major scales from other scale types
/// at the type level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MajorScaleType;

impl ScaleType for MajorScaleType {
    fn name() -> &'static str {
        "major"
    }
}

/// The pattern for a major scale.
///
/// This pattern defines the sequence of intervals that make up a major scale:
/// ```text
/// Root + Major 2nd + Major 3rd + Perfect 4th +
/// Perfect 5th + Major 6th + Major 7th
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

        let pitches = scale.pitches();
        assert_eq!(pitches.len(), 7);
        assert_eq!(pitches[0], C4);
        assert_eq!(pitches[1], D4);
        assert_eq!(pitches[2], E4);
        assert_eq!(pitches[3], F4);
        assert_eq!(pitches[4], G4);
        assert_eq!(pitches[5], A4);
        assert_eq!(pitches[6], B4);

        assert_eq!(scale.root(), C4);
        assert_eq!(scale.name(), "major");
        assert_eq!(scale.to_string(), "C4 major");
    }

    #[test]
    fn test_major_scale_g() {
        let scale = MajorScalePattern::apply(G4);
        let pitches = scale.pitches();
        assert_eq!(pitches.len(), 7);
        assert_eq!(pitches[0], G4);
        assert_eq!(pitches[1], A4);
        assert_eq!(pitches[2], B4);
        assert_eq!(pitches[3], C5);
        assert_eq!(pitches[4], D5);
        assert_eq!(pitches[5], E5);
        assert_eq!(pitches[6], FSHARP5);
    }
}
