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
//!         PERFECT_UNISON,
//!         MAJOR_SECOND,
//!         MAJOR_THIRD,
//!         PERFECT_FOURTH,
//!         PERFECT_FIFTH,
//!         MAJOR_SIXTH,
//!         MAJOR_SEVENTH,
//!     ];
//!     type ScaleTyp = MajorScaleType;
//! }
//!
//! // Apply the scale to a root note
//! let c_major = MajorScalePattern::apply(C4);
//! let pitches = c_major.pitches();
//! assert_eq!(pitches.len(), 7);
//! assert_eq!(pitches[0], C4);
//! assert_eq!(pitches[1], D4);
//! assert_eq!(pitches[2], E4);
//! assert_eq!(pitches[3], F4);
//! assert_eq!(pitches[4], G4);
//! assert_eq!(pitches[5], A4);
//! assert_eq!(pitches[6], B4);
//! // ... and so on
//! ```
//!
//! Working with different scale types:
//! ```rust
//! use mozzart_core::scale::{ScalePattern, ScaleType};
//! use mozzart_core::constants::*;
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

use std::marker::PhantomData;

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
///         PERFECT_UNISON,
///         MAJOR_SECOND,
///         MAJOR_THIRD,
///         PERFECT_FOURTH,
///         PERFECT_FIFTH,
///         MAJOR_SIXTH,
///         MAJOR_SEVENTH,
///     ];
///     type ScaleTyp = MajorScaleType;
/// }
///
/// let scale = MajorScalePattern::apply(C4);
/// let pitches = scale.pitches();
/// assert_eq!(pitches.len(), 7);
/// assert_eq!(pitches[0], C4);
/// assert_eq!(pitches[1], D4);
/// assert_eq!(pitches[2], E4);
/// assert_eq!(pitches[3], F4);
/// assert_eq!(pitches[4], G4);
/// assert_eq!(pitches[5], A4);
/// assert_eq!(pitches[6], B4);
/// ```
pub trait ScalePattern {
    /// The type of the interval pattern.
    type Pattern: IntoIterator<Item = Interval>;

    /// The interval pattern that defines the scale.
    const PATTERN: Self::Pattern;

    /// The type of the scale.
    type ScaleTyp: ScaleType;

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
    ///         PERFECT_UNISON,
    ///         MAJOR_SECOND,
    ///         MAJOR_THIRD,
    ///         PERFECT_FOURTH,
    ///         PERFECT_FIFTH,
    ///         MAJOR_SIXTH,
    ///         MAJOR_SEVENTH,
    ///     ];
    ///     type ScaleTyp = MajorScaleType;
    /// }
    ///
    /// let scale = MajorScalePattern::apply(C4);
    /// let pitches = scale.pitches();
    /// assert_eq!(pitches.len(), 7);
    /// assert_eq!(pitches[0], C4);
    /// assert_eq!(pitches[1], D4);
    /// assert_eq!(pitches[2], E4);
    /// assert_eq!(pitches[3], F4);
    /// assert_eq!(pitches[4], G4);
    /// assert_eq!(pitches[5], A4);
    /// assert_eq!(pitches[6], B4);
    /// ```
    #[inline]
    fn apply(root: Pitch) -> Scale<Self::ScaleTyp> {
        let pitches = apply_pattern(root, Self::PATTERN);
        Scale::<Self::ScaleTyp>::new(pitches)
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

    for interval in pattern.into_iter() {
        let pitch = root.transpose(interval);
        pitches.push(pitch);
    }

    pitches
}

/// A musical scale.
///
/// A scale is a sequence of pitches ordered by pitch height, defined by a specific
/// pattern of intervals from a root note. The `Scale` struct represents a scale
/// with a specific type (e.g., major, minor) and contains the sequence of pitches
/// that make up the scale.
///
/// # Examples
///
/// Creating and using a major scale:
/// ```rust
/// use mozzart_core::scale::*;
/// use mozzart_core::pitch::constants::*;
///
/// // Define a major scale type
/// struct MajorScaleType;
/// impl ScaleType for MajorScaleType {}
///
/// // Create a C major scale
/// let c_major = Scale::<MajorScaleType>::new(vec![C4, D4, E4, F4, G4, A4, B4]);
///
/// // Get the pitches of the scale
/// let pitches = c_major.pitches();
/// assert_eq!(pitches[0], C4);
/// assert_eq!(pitches[1], D4);
/// assert_eq!(pitches[2], E4);
/// assert_eq!(pitches[3], F4);
/// assert_eq!(pitches[4], G4);
/// assert_eq!(pitches[5], A4);
/// assert_eq!(pitches[6], B4);
/// ```
///
/// Working with different scale types:
/// ```rust
/// use mozzart_core::scale::*;
/// use mozzart_core::pitch::constants::*;
///
/// // Define a minor scale type
/// struct MinorScaleType;
/// impl ScaleType for MinorScaleType {}
///
/// // Create an A minor scale
/// let a_minor = Scale::<MinorScaleType>::new(vec![A4, B4, C5, D5, E5, F5, G5]);
///
/// // Get the pitches of the scale
/// let pitches = a_minor.pitches();
/// assert_eq!(pitches[0], A4);
/// assert_eq!(pitches[1], B4);
/// assert_eq!(pitches[2], C5);
/// assert_eq!(pitches[3], D5);
/// assert_eq!(pitches[4], E5);
/// assert_eq!(pitches[5], F5);
/// assert_eq!(pitches[6], G5);
/// ```
///
/// # Musical Concepts
///
/// ## Scale Structure
/// A scale is defined by:
/// - A root note (the starting pitch)
/// - A pattern of intervals from the root
/// - A specific scale type (e.g., major, minor)
///
/// ```text
/// C Major Scale Structure:
/// Root: C4
/// Pattern: W W H W W W H (whole and half steps)
/// Pitches: C4 D4 E4 F4 G4 A4 B4
/// ```
///
/// ## Scale Types
/// Different scale types have different interval patterns:
/// ```text
/// Major Scale:      W W H W W W H
/// Natural Minor:    W H W W H W W
/// Harmonic Minor:   W H W W H WH H
/// Melodic Minor:    W H W W W W H
/// ```
pub struct Scale<S: ScaleType> {
    /// The sequence of pitches that make up the scale.
    pitches: Vec<Pitch>,
    /// A phantom data marker to associate the scale with its type.
    typ: PhantomData<S>,
}

impl<S: ScaleType> Scale<S> {
    /// Creates a new scale from a sequence of pitches.
    ///
    /// # Arguments
    ///
    /// * `pitches` - A vector of pitches that make up the scale.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mozzart_core::scale::*;
    /// use mozzart_core::pitch::constants::*;
    ///
    /// struct MajorScaleType;
    /// impl ScaleType for MajorScaleType {}
    ///
    /// // Create a G major scale
    /// let g_major = Scale::<MajorScaleType>::new(vec![G4, A4, B4, C5, D5, E5, FSHARP5]);
    /// assert_eq!(g_major.pitches()[0], G4);
    /// assert_eq!(g_major.pitches()[6], FSHARP5);
    /// ```
    #[inline]
    pub const fn new(pitches: Vec<Pitch>) -> Self {
        Self {
            pitches,
            typ: PhantomData,
        }
    }

    /// Returns a reference to the sequence of pitches in the scale.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mozzart_core::scale::*;
    /// use mozzart_core::pitch::constants::*;
    ///
    /// struct MajorScaleType;
    /// impl ScaleType for MajorScaleType {}
    ///
    /// // Create a D major scale
    /// let d_major = Scale::<MajorScaleType>::new(vec![D4, E4, FSHARP4, G4, A4, B4, CSHARP5]);
    ///
    /// // Get the pitches
    /// let pitches = d_major.pitches();
    /// assert_eq!(pitches[0], D4);
    /// assert_eq!(pitches[2], FSHARP4);
    /// assert_eq!(pitches[6], CSHARP5);
    /// ```
    #[inline]
    pub fn pitches(&self) -> &[Pitch] {
        &self.pitches
    }
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
        assert_eq!(scale, [D4, F4]);
    }
}
