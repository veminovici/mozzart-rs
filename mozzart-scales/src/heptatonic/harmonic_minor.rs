//! Harmonic minor scale implementation.
//!
//! The harmonic minor scale is one of the three forms of the minor scale
//! in Western music. It is characterized by its exotic, Middle Eastern sound
//! due to the augmented second between the sixth and seventh scale degrees.
//!
//! # Scale Structure
//!
//! The harmonic minor scale follows the pattern of whole and half steps:
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
//! The harmonic minor scale is built using the following intervals from the root:
//! ```text
//! Root (0) + Major 2nd (2) + Minor 3rd (3) + Perfect 4th (5) +
//! Perfect 5th (7) + Minor 6th (8) + Major 7th (11)
//! ```
use mozzart_core::constants::*;
use mozzart_core::{Interval, ScalePattern, ScaleType};

/// A marker type for harmonic minor scales.
///
/// This type is used to distinguish harmonic minor scales from other scale types
/// at the type level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HarmonicMinorScaleType;

impl ScaleType for HarmonicMinorScaleType {
    fn name() -> &'static str {
        "harmonic minor"
    }
}

/// The pattern for a harmonic minor scale.
///
/// This pattern defines the sequence of intervals that make up a harmonic minor scale:
/// ```text
/// Root + Major 2nd + Minor 3rd + Perfect 4th +
/// Perfect 5th + Minor 6th + Major 7th
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    fn test_harmonic_minor_scale() {
        let scale = HarmonicMinorScalePattern::apply(C4);
        let pitches = scale.pitches();
        assert_eq!(pitches.len(), 7);
        assert_eq!(pitches[0], C4);
        assert_eq!(pitches[1], D4);
        assert_eq!(pitches[2], EFLAT4);
        assert_eq!(pitches[3], F4);
        assert_eq!(pitches[4], G4);
        assert_eq!(pitches[5], AFLAT4);
        assert_eq!(pitches[6], B4);
    }

    #[test]
    fn test_harmonic_minor_scale_e() {
        let scale = HarmonicMinorScalePattern::apply(E4);
        let pitches = scale.pitches();
        assert_eq!(pitches.len(), 7);
        assert_eq!(pitches[0], E4);
        assert_eq!(pitches[1], FSHARP4);
        assert_eq!(pitches[2], G4);
        assert_eq!(pitches[3], A4);
        assert_eq!(pitches[4], B4);
        assert_eq!(pitches[5], C5);
        assert_eq!(pitches[6], DSHARP5);
    }
}
