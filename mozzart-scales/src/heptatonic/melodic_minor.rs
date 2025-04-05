//! Melodic minor scale implementation.
//!
//! The melodic minor scale is one of the three forms of the minor scale
//! in Western music. It has different ascending and descending forms,
//! making it unique among the minor scales.
//!
//! # Scale Structure
//!
//! The melodic minor scale has two forms:
//!
//! ## Ascending Form
//! ```text
//! W H W W W W H
//! ```
//!
//! ## Descending Form
//! ```text
//! W W H W W H W
//! ```
//!
//! Where:
//! - W = Whole step (2 semitones)
//! - H = Half step (1 semitone)
//!
//! # Interval Pattern
//!
//! ## Ascending Form
//! ```text
//! Root (0) + Major 2nd (2) + Minor 3rd (3) + Perfect 4th (5) +
//! Perfect 5th (7) + Major 6th (9) + Major 7th (11)
//! ```
//!
//! ## Descending Form
//! ```text
//! Root (0) + Major 2nd (2) + Minor 3rd (3) + Perfect 4th (5) +
//! Perfect 5th (7) + Minor 6th (8) + Minor 7th (10)
//! ```

use mozzart_core::constants::*;
use mozzart_core::{Interval, ScalePattern, ScaleType};

/// A marker type for melodic minor scales.
///
/// This type is used to distinguish melodic minor scales from other scale types
/// at the type level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MelodicMinorScaleType;

impl ScaleType for MelodicMinorScaleType {}

/// The pattern for a melodic minor scale.
///
/// This pattern defines the sequence of intervals that make up a melodic minor scale:
///
/// ## Ascending Form
/// ```text
/// Root + Major 2nd + Minor 3rd + Perfect 4th +
/// Perfect 5th + Major 6th + Major 7th
/// ```
///
/// ## Descending Form
/// ```text
/// Root + Major 2nd + Minor 3rd + Perfect 4th +
/// Perfect 5th + Minor 6th + Minor 7th
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        let ascending = scale.pitches();
        assert_eq!(ascending.len(), 7);
        assert_eq!(ascending[0], C4);
        assert_eq!(ascending[1], D4);
        assert_eq!(ascending[2], EFLAT4);
        assert_eq!(ascending[3], F4);
        assert_eq!(ascending[4], G4);
        assert_eq!(ascending[5], A4);
        assert_eq!(ascending[6], B4);
    }
}
