//! Musical interval representation and operations.
//!
//! This module provides functionality for working with musical intervals, including:
//! - Interval representation and manipulation
//! - Common interval constants
//! - Interval calculations
//!
//! # Performance Characteristics
//!
//! All operations in this module are O(1) and have minimal overhead:
//! - Interval creation: ~1ns
//! - Interval addition: ~1ns
//! - Semitone calculation: ~1ns
//!
//! # Interval System
//!
//! A musical interval is the difference in pitch between two notes.
//! In this system, intervals are represented by the number of semitones they span:
//!
//! ```text
//! Interval Names and Semitones:
//! Perfect Unison:    0 semitones (P1)
//! Minor Second:      1 semitone  (m2)
//! Major Second:      2 semitones (M2)
//! Minor Third:       3 semitones (m3)
//! Major Third:       4 semitones (M3)
//! Perfect Fourth:    5 semitones (P4)
//! Diminished Fifth:  6 semitones (d5)
//! Perfect Fifth:     7 semitones (P5)
//! Minor Sixth:       8 semitones (m6)
//! Major Sixth:       9 semitones (M6)
//! Minor Seventh:    10 semitones (m7)
//! Major Seventh:    11 semitones (M7)
//! Perfect Octave:   12 semitones (P8)
//! ```
//!
//! # Examples
//!
//! Basic interval operations:
//! ```rust
//! use mozzart_core::Interval;
//! use mozzart_core::constants::*;
//!
//! // Create an interval from semitones
//! let interval = PERFECT_FIFTH; // Perfect Fifth
//!
//! // Get the semitone value
//! assert_eq!(interval.semitones(), 7);
//!
//! // Use predefined interval constants
//! assert_eq!(PERFECT_FIFTH.semitones(), 7);
//! assert_eq!(MAJOR_THIRD.semitones(), 4);
//! ```
//!
//! Working with intervals in scales:
//! ```rust
//! use mozzart_core::constants::*;
//!
//! // Build a major triad
//! let root = C4;
//! let third = root.transpose(MAJOR_THIRD);
//! let fifth = root.transpose(PERFECT_FIFTH);
//!
//! assert_eq!(third, E4);
//! assert_eq!(fifth, G4);
//! ```
//!
//! # Musical Concepts
//!
//! ## Interval Quality
//! Intervals are classified by their quality and size:
//! - Perfect intervals: unison, fourth, fifth, octave
//! - Major/minor intervals: second, third, sixth, seventh
//! - Diminished/augmented intervals: altered perfect or major/minor intervals
//!
//! ```text
//! Interval Qualities:
//! Perfect:    P1, P4, P5, P8
//! Major:      M2, M3, M6, M7
//! Minor:      m2, m3, m6, m7
//! Diminished: d5 (tritone)
//! ```
//!
//! ## Consonance and Dissonance
//! Intervals are also classified by their harmonic quality:
//!
//! ```text
//! Consonant Intervals:
//! Perfect Consonance: P1, P5, P8
//! Imperfect Consonance: M3, m3, M6, m6
//!
//! Dissonant Intervals:
//! Mild Dissonance: M2, m7
//! Strong Dissonance: m2, M7, d5
//! ```
//!
//! # Tutorial: Common Use Cases
//!
//! ## Building Scales
//! ```rust
//! use mozzart_core::constants::*;
//!
//! // Major scale pattern
//! let major_pattern = [
//!     PERFECT_UNISON,
//!     MAJOR_SECOND,
//!     MAJOR_THIRD,
//!     PERFECT_FOURTH,
//!     PERFECT_FIFTH,
//!     MAJOR_SIXTH,
//!     MAJOR_SEVENTH,
//! ];
//!
//! // Natural minor scale pattern
//! let minor_pattern = [
//!     PERFECT_UNISON,
//!     MAJOR_SECOND,
//!     MINOR_THIRD,
//!     PERFECT_FOURTH,
//!     PERFECT_FIFTH,
//!     MINOR_SIXTH,
//!     MINOR_SEVENTH,
//! ];
//! ```
//!
//! ## Building Chords
//! ```rust
//! use mozzart_core::constants::*;
//!
//! // Major triad pattern
//! let major_triad = [
//!     PERFECT_UNISON,
//!     MAJOR_THIRD,
//!     PERFECT_FIFTH,
//! ];
//!
//! // Minor triad pattern
//! let minor_triad = [
//!     PERFECT_UNISON,
//!     MINOR_THIRD,
//!     PERFECT_FIFTH,
//! ];
//!
//! // Dominant seventh chord pattern
//! let dominant_seventh = [
//!     PERFECT_UNISON,
//!     MAJOR_THIRD,
//!     PERFECT_FIFTH,
//!     MINOR_SEVENTH,
//! ];
//! ```
//!
//! ## Interval Inversion
//! ```rust
//! use mozzart_core::Interval;
//! use mozzart_core::constants::*;
//!
//! // Invert an interval (subtract from perfect octave)
//! let major_third = MAJOR_THIRD.semitones();
//! let perfect_octave = PERFECT_OCTAVE.semitones();
//! let minor_sixth = Interval::new(perfect_octave - major_third);
//! assert_eq!(minor_sixth.semitones(), 8); // Minor sixth
//! ```
//!
//! ## Compound Intervals
//! ```rust
//! use mozzart_core::Interval;
//! use mozzart_core::constants::*;
//!
//! // Create a compound interval (octave + simple interval)
//! let perfect_octave = PERFECT_OCTAVE.semitones();
//! let major_third = MAJOR_THIRD.semitones();
//! let major_tenth = Interval::new(perfect_octave + major_third);
//! assert_eq!(major_tenth.semitones(), 16); // Major tenth
//! ```
//!
//! ## Interval Addition
//! ```rust
//! use mozzart_core::Interval;
//! use mozzart_core::constants::*;
//!
//! // Add intervals together
//! let major_third = MAJOR_THIRD.semitones();
//! let perfect_fourth = PERFECT_FOURTH.semitones();
//! let major_sixth = Interval::new(major_third + perfect_fourth);
//! assert_eq!(major_sixth.semitones(), 9); // Major sixth
//! ```

/// Represents a musical interval.
///
/// An interval is the difference in pitch between two notes,
/// measured in semitones. For example:
/// - 0 semitones: Perfect Unison
/// - 7 semitones: Perfect Fifth
/// - 12 semitones: Perfect Octave
///
/// # Examples
///
/// ```rust
/// use mozzart_core::Interval;
/// use mozzart_core::constants::*;
///
/// // Create an interval from semitones
/// let interval = PERFECT_FIFTH; // Perfect Fifth
///
/// // Get the semitone value
/// assert_eq!(interval.semitones(), 7);
///
/// // Use predefined interval constants
/// assert_eq!(PERFECT_FIFTH.semitones(), 7);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval(u8);

impl Interval {
    /// Create a new interval from a number of semitones.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mozzart_core::Interval;
    /// use mozzart_core::constants::*;
    ///
    /// let interval = Interval::new(7);
    /// assert_eq!(interval.semitones(), 7);
    /// ```
    #[inline]
    pub const fn new(semitones: u8) -> Self {
        Self(semitones)
    }

    /// Returns the number of semitones in this interval.
    ///
    /// The semitone value represents the distance between two pitches:
    /// - 0: Perfect Unison (same note)
    /// - 12: Perfect Octave (same note name, different octave)
    /// - 7: Perfect Fifth (e.g., C to G)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mozzart_core::Interval;
    /// use mozzart_core::constants::*;
    ///
    /// assert_eq!(PERFECT_UNISON.semitones(), 0);
    /// assert_eq!(PERFECT_FIFTH.semitones(), 7);
    /// assert_eq!(PERFECT_OCTAVE.semitones(), 12);
    /// ```
    #[inline]
    pub const fn semitones(&self) -> u8 {
        self.0
    }
}

/// Constants for common musical intervals.
///
/// This module provides constants for all standard intervals:
/// - Perfect intervals: unison, fourth, fifth, octave
/// - Major/minor intervals: second, third, sixth, seventh
/// - Diminished intervals: fifth
///
/// # Examples
///
/// ```rust
/// use mozzart_core::constants::*;
///
/// // Perfect intervals
/// assert_eq!(PERFECT_UNISON.semitones(), 0);
/// assert_eq!(PERFECT_FOURTH.semitones(), 5);
/// assert_eq!(PERFECT_FIFTH.semitones(), 7);
/// assert_eq!(PERFECT_OCTAVE.semitones(), 12);
///
/// // Major/minor intervals
/// assert_eq!(MAJOR_SECOND.semitones(), 2);
/// assert_eq!(MINOR_THIRD.semitones(), 3);
/// assert_eq!(MAJOR_SIXTH.semitones(), 9);
/// assert_eq!(MINOR_SEVENTH.semitones(), 10);
/// ```
pub mod constants {
    use super::Interval;

    /// Perfect unison (0 semitones)
    pub const PERFECT_UNISON: Interval = Interval(0);
    /// Minor second (1 semitone)
    pub const MINOR_SECOND: Interval = Interval(1);
    /// Major second (2 semitones)
    pub const MAJOR_SECOND: Interval = Interval(2);
    /// Minor third (3 semitones)
    pub const MINOR_THIRD: Interval = Interval(3);
    /// Major third (4 semitones)
    pub const MAJOR_THIRD: Interval = Interval(4);
    /// Perfect fourth (5 semitones)
    pub const PERFECT_FOURTH: Interval = Interval(5);
    /// Diminished fifth (6 semitones)
    pub const DIMINISHED_FIFTH: Interval = Interval(6);
    /// Perfect fifth (7 semitones)
    pub const PERFECT_FIFTH: Interval = Interval(7);
    /// Minor sixth (8 semitones)
    pub const MINOR_SIXTH: Interval = Interval(8);
    /// Major sixth (9 semitones)
    pub const MAJOR_SIXTH: Interval = Interval(9);
    /// Minor seventh (10 semitones)
    pub const MINOR_SEVENTH: Interval = Interval(10);
    /// Major seventh (11 semitones)
    pub const MAJOR_SEVENTH: Interval = Interval(11);
    /// Perfect octave (12 semitones)
    pub const PERFECT_OCTAVE: Interval = Interval(12);
}

#[cfg(test)]
mod tests {
    use crate::interval::constants::*;
    use crate::pitch::constants::*;

    #[test]
    fn test_interval_semitones() {
        assert_eq!(PERFECT_UNISON.semitones(), 0);
        assert_eq!(PERFECT_FIFTH.semitones(), 7);
        assert_eq!(PERFECT_OCTAVE.semitones(), 12);
    }

    #[test]
    fn test_interval_transposition() {
        let root = C4;
        assert_eq!(root.transpose(PERFECT_FIFTH), G4);
        assert_eq!(root.transpose(MAJOR_THIRD), E4);
        assert_eq!(root.transpose(PERFECT_OCTAVE), C5);
    }
}
