//! Harmonic major scale implementation.
//!
//! The harmonic major scale is one of the three forms of the major scale
//! in Western music. It is characterized by its exotic, Middle Eastern sound
//! due to the augmented second between the sixth and seventh scale degrees.
//!
//! # Scale Structure
//!
//! The harmonic major scale follows the pattern of whole and half steps:
//! ```text
//! W H W W H WH H
//! ```
//!
//! Where:
//! - W = Whole step (2 semitones)
//! - H = Half step (1 semitone)
//! - WH = Augmented second (3 semitones)
//!
//! # Interval Pattern
//!
//! The harmonic major scale is built using the following intervals from the root:
//! ```text
//! Root (0) + Major 2nd (2) + Major 3rd (4) + Perfect 4th (5) +
//! Perfect 5th (7) + Minor 6th (8) + Major 7th (11)
//! ```
use mozzart_core::constants::*;
use mozzart_core::{Interval, ScalePattern, ScaleType};

/// A marker type for harmonic minor scales.
///
/// This type is used to distinguish harmonic minor scales from other scale types
/// at the type level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HarmonicMajorScaleType;

impl ScaleType for HarmonicMajorScaleType {
    fn name() -> &'static str {
        "harmonic major"
    }
}

/// The pattern for a harmonic major scale.
///
/// This pattern defines the sequence of intervals that make up a harmonic major scale:
/// ```text
/// Root + Major 2nd + Minor 3rd + Perfect 4th +
/// Perfect 5th + Minor 6th + Major 7th
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HarmonicMajorScalePattern;

impl ScalePattern for HarmonicMajorScalePattern {
    type Pattern = [Interval; 7];
    const PATTERN: Self::Pattern = [
        PERFECT_UNISON,
        MAJOR_SECOND,
        MAJOR_THIRD,
        PERFECT_FOURTH,
        PERFECT_FIFTH,
        MINOR_SIXTH,
        MAJOR_SEVENTH,
    ];

    type ScaleTyp = HarmonicMajorScaleType;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mozzart_core::ScalePattern;

    #[test]
    fn test_harmonic_major_scale() {
        let scale = HarmonicMajorScalePattern::apply(C4);

        let pitches = scale.pitches();
        assert_eq!(pitches.len(), 7);
        assert_eq!(pitches[0], C4);
        assert_eq!(pitches[1], D4);
        assert_eq!(pitches[2], E4);
        assert_eq!(pitches[3], F4);
        assert_eq!(pitches[4], G4);
        assert_eq!(pitches[5], AFLAT4);
        assert_eq!(pitches[6], B4);

        assert_eq!(scale.root(), C4);
        assert_eq!(scale.name(), "harmonic major");
        assert_eq!(scale.to_string(), "C4 harmonic major");
    }
}
