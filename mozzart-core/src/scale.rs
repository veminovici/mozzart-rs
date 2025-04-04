//! Musical scale representation and operations.
//!
//! This module provides functionality for working with musical scales, including:
//! - Scale pattern definitions
//! - Scale type classification
//! - Scale application to root pitches
//!
//! # Scale System
//!
//! A musical scale is a set of musical notes ordered by fundamental frequency or pitch.
//! In this system, scales are defined by their pattern of intervals from the root note.
//!
//! ```text
//! Common Scale Patterns:
//! Major Scale:      W W H W W W H  (2 2 1 2 2 2 1 semitones)
//! Natural Minor:    W H W W H W W  (2 1 2 2 1 2 2 semitones)
//! Harmonic Minor:   W H W W H WH H (2 1 2 2 1 3 1 semitones)
//! Melodic Minor:    W H W W W W H  (2 1 2 2 2 2 1 semitones)
//! Pentatonic:      W W m3 W m3     (2 2 3 2 3 semitones)
//! ```
//!
//! # Examples
//!
//! Basic scale operations:
//! ```rust
//! use mozzart_core::{Interval, Pitch, ScalePattern, ScaleType};
//! use mozzart_core::constants::*;
//!
//! pub struct MajorScaleType;
//! impl ScaleType for MajorScaleType {}
//!
//! // Define a major scale pattern
//! struct MajorScalePattern;
//! impl ScalePattern for MajorScalePattern {
//!     type Pattern = [Interval; 7];
//!     const PATTERN: Self::Pattern = [
//!         MAJOR_SECOND,
//!         MAJOR_SECOND,
//!         MINOR_SECOND,
//!         MAJOR_SECOND,
//!         MAJOR_SECOND,
//!         MAJOR_SECOND,
//!         MINOR_SECOND,
//!     ];
//!     type ScaleTyp = MajorScaleType;
//!     const TYPE: Self::ScaleTyp = MajorScaleType;
//! }
//!
//! // Apply the scale to a root note
//! let c_major = MajorScalePattern::apply(C4);
//! assert_eq!(c_major[0], D4);
//! assert_eq!(c_major[1], E4);
//! // ... and so on
//! ```
//!
//! Working with different scale types:
//! ```rust
//! use mozzart_core::scale::{ScalePattern, ScaleType};
//! use mozzart_core::pitch::constants::*;
//! use mozzart_core::interval::constants::*;
//!
//! ```
//!
//! # Musical Concepts
//!
//! ## Scale Pattern
//! A scale pattern defines the sequence of intervals that make up a scale.
//! The pattern is applied to a root note to generate the complete scale.
//!
//! ```text
//! C Major Scale Pattern:
//! Root: C
//! Pattern: W W H W W W H
//! Result: C D E F G A B C
//!
//! A Minor Scale Pattern:
//! Root: A
//! Pattern: W H W W H W W
//! Result: A B C D E F G A
//! ```
//!
//! ## Scale Type
//! A scale type categorizes scales based on their interval patterns and musical characteristics.
//! Common scale types include:
//! - Major scales
//! - Minor scales (natural, harmonic, melodic)
//! - Pentatonic scales
//! - Blues scales
//! - Modal scales
//!
//! ```text
//! Scale Type Characteristics:
//! Major: Bright, happy sound
//! Minor: Dark, sad sound
//! Pentatonic: Simple, versatile sound
//! Blues: Expressive, soulful sound
//! ```

use crate::{Interval, Pitch};

/// A trait representing a type of musical scale.
///
/// This trait is used to categorize scales based on their musical characteristics.
/// Implement this trait for your scale types to enable type-safe scale operations.
///
/// # Examples
///
/// ```rust
/// use mozzart_core::scale::ScaleType;
///
/// struct MajorScaleType;
/// impl ScaleType for MajorScaleType {}
///
/// struct MinorScaleType;
/// impl ScaleType for MinorScaleType {}
/// ```
pub trait ScaleType {}

/// A trait representing a musical scale pattern.
///
/// This trait defines the interval pattern that makes up a scale.
/// Implement this trait to create new scale patterns.
///
/// # Examples
///
/// ```rust
/// use mozzart_core::{Interval, Pitch, ScalePattern, ScaleType};
/// use mozzart_core::constants::*;
///
/// pub struct MajorScaleType;
/// impl ScaleType for MajorScaleType {}
///
/// struct MajorScalePattern;
/// impl ScalePattern for MajorScalePattern {
///     type Pattern = [Interval; 7];
///     const PATTERN: Self::Pattern = [
///         MAJOR_SECOND,
///         MAJOR_SECOND,
///         MINOR_SECOND,
///         MAJOR_SECOND,
///         MAJOR_SECOND,
///         MAJOR_SECOND,
///         MINOR_SECOND,
///     ];
///     type ScaleTyp = MajorScaleType;
///     const TYPE: Self::ScaleTyp = MajorScaleType;
/// }
/// ```
pub trait ScalePattern {
    /// The type of the interval pattern.
    type Pattern: IntoIterator<Item = Interval>;

    /// The interval pattern that defines the scale.
    const PATTERN: Self::Pattern;

    /// The type of the scale.
    type ScaleTyp: ScaleType;

    /// The scale type instance.
    const TYPE: Self::ScaleTyp;

    /// Applies the scale pattern to a root pitch.
    ///
    /// This method generates a sequence of pitches by applying the scale's
    /// interval pattern to the given root pitch.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mozzart_core::{Interval, Pitch, ScalePattern, ScaleType};
    /// use mozzart_core::constants::*;
    ///
    /// pub struct MajorScaleType;
    /// impl ScaleType for MajorScaleType {}
    ///
    /// pub struct MajorScalePattern;
    /// impl ScalePattern for MajorScalePattern {
    ///     type Pattern = [Interval; 7];
    ///     const PATTERN: Self::Pattern = [
    ///         MAJOR_SECOND,
    ///         MAJOR_SECOND,
    ///         MINOR_SECOND,
    ///         MAJOR_SECOND,
    ///         MAJOR_SECOND,
    ///         MAJOR_SECOND,
    ///         MINOR_SECOND,
    ///     ];
    ///     type ScaleTyp = MajorScaleType;
    ///     const TYPE: Self::ScaleTyp = MajorScaleType;
    /// }
    ///
    /// let c_major = MajorScalePattern::apply(C4);
    /// assert_eq!(c_major[0], D4);
    /// assert_eq!(c_major[1], E4);
    /// ```
    #[inline]
    fn apply(root: Pitch) -> Vec<Pitch> {
        apply_pattern(root, Self::PATTERN)
    }
}

/// Applies a scale pattern to a root pitch.
///
/// This function generates a sequence of pitches by applying the given
/// interval pattern to the root pitch.
///
/// # Notes
///
/// This function is not generic over the scale type, but over the pattern.
/// This is because the scale type is part of the scale pattern.
fn apply_pattern<P>(root: Pitch, pattern: P) -> Vec<Pitch>
where
    P: IntoIterator<Item = Interval>,
{
    let mut pitches = Vec::new();
    let mut current = root;

    for interval in pattern.into_iter() {
        current = current.transpose(interval);
        pitches.push(current);
    }

    pitches
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;
    use crate::pitch::constants::*;

    #[test]
    fn test_apply_pattern() {
        let pattern = [MAJOR_SECOND, PERFECT_FOURTH];
        let root = C4;
        let scale = apply_pattern(root, pattern);
        assert_eq!(scale, [D4, G4]);
    }
}
