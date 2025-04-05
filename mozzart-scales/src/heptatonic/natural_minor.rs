//! Natural minor scale implementation.
//!
//! The natural minor scale is one of the three forms of the minor scale
//! in Western music. It is characterized by its dark, melancholic sound
//! and is used extensively in various musical genres.
//!
//! # Scale Structure
//!
//! The natural minor scale follows the pattern of whole and half steps:
//! ```text
//! W H W W H W W
//! ```
//!
//! Where:
//! - W = Whole step (2 semitones)
//! - H = Half step (1 semitone)
//!
//! # Interval Pattern
//!
//! The natural minor scale is built using the following intervals from the root:
//! ```text
//! Root (0) + Major 2nd (2) + Minor 3rd (3) + Perfect 4th (5) +
//! Perfect 5th (7) + Minor 6th (8) + Minor 7th (10)
//! ```
use mozzart_core::constants::*;
use mozzart_core::{Interval, ScalePattern, ScaleType};

/// A marker type for natural minor scales.
///
/// This type is used to distinguish natural minor scales from other scale types
/// at the type level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NaturalMinorScaleType;

impl ScaleType for NaturalMinorScaleType {}

/// The pattern for a natural minor scale.
///
/// This pattern defines the sequence of intervals that make up a natural minor scale:
/// ```text
/// Root + Major 2nd + Minor 3rd + Perfect 4th +
/// Perfect 5th + Minor 6th + Minor 7th
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        let pitches = scale.pitches();
        assert_eq!(pitches.len(), 7);
        assert_eq!(pitches[0], C4);
        assert_eq!(pitches[1], D4);
        assert_eq!(pitches[2], EFLAT4);
        assert_eq!(pitches[3], F4);
        assert_eq!(pitches[4], G4);
        assert_eq!(pitches[5], AFLAT4);
        assert_eq!(pitches[6], BFLAT4);
    }

    #[test]
    fn test_natural_minor_scale_e() {
        let scale = NaturalMinorScalePattern::apply(E4);
        let pitches = scale.pitches();
        assert_eq!(pitches.len(), 7);
        assert_eq!(pitches[0], E4);
        assert_eq!(pitches[1], FSHARP4);
        assert_eq!(pitches[2], G4);
        assert_eq!(pitches[3], A4);
        assert_eq!(pitches[4], B4);
        assert_eq!(pitches[5], C5);
        assert_eq!(pitches[6], D5);
    }
}
