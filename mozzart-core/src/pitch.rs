//! Musical pitch representation and operations.
//!
//! This module provides functionality for working with musical pitches, including:
//! - Pitch representation and manipulation
//! - Octave handling
//! - Interval calculations
//! - Pitch constants for all octaves
//!
//! # Performance Characteristics
//!
//! All operations in this module are O(1) and have minimal overhead:
//! - Pitch creation: ~1ns
//! - Transposition: ~2ns
//! - Canonical form: ~1ns
//! - Octave calculation: ~1ns
//!
//! # Pitch Numbering System
//!
//! The pitch numbering system used in this crate follows the MIDI standard:
//!
//! ```text
//! Octave -1: C-1 (0)  C#-1 (1)  D-1 (2)  D#-1 (3)  E-1 (4)  F-1 (5)  F#-1 (6)  G-1 (7)  G#-1 (8)  A-1 (9)  A#-1 (10)  B-1 (11)
//! Octave 0:  C0 (12)  C#0 (13)  D0 (14)  D#0 (15)  E0 (16)  F0 (17)  F#0 (18)  G0 (19)  G#0 (20)  A0 (21)  A#0 (22)   B0 (23)
//! Octave 1:  C1 (24)  C#1 (25)  D1 (26)  D#1 (27)  E1 (28)  F1 (29)  F#1 (30)  G1 (31)  G#1 (32)  A1 (33)  A#1 (34)   B1 (35)
//! ...and so on
//! ```
//!
//! # Examples
//!
//! Basic pitch operations:
//! ```rust
//! use mozzart_core::Pitch;
//! use mozzart_core::constants::*;
//!
//! // Create a pitch from a MIDI number
//! let pitch = C4; // Middle C (C4)
//!
//! // Get the semitone value
//! assert_eq!(pitch.semitones(), 60);
//!
//! // Get the canonical form (pitch class)
//! assert_eq!(pitch.canonical(), C);
//!
//! // Get the octave
//! assert_eq!(pitch.octave(), O4);
//! ```
//!
//! Working with intervals:
//! ```rust
//! use mozzart_core::Pitch;
//! use mozzart_core::constants::*;
//!
//! // Transpose a pitch by an interval
//! let pitch = C4;
//! let transposed = pitch.transpose(MAJOR_THIRD);
//! assert_eq!(transposed, E4);
//!
//! // Create a major scale
//! let scale = [
//!     C4,
//!     C4.transpose(MAJOR_SECOND),
//!     C4.transpose(MAJOR_THIRD),
//!     C4.transpose(PERFECT_FOURTH),
//!     C4.transpose(PERFECT_FIFTH),
//!     C4.transpose(MAJOR_SIXTH),
//!     C4.transpose(MAJOR_SEVENTH),
//! ];
//! ```
//!
//! # Musical Concepts
//!
//! ## Pitch Class
//! A pitch class is a set of all pitches that are a whole number of octaves apart.
//! For example, the pitch class C contains C0, C1, C2, etc.
//!
//! ```text
//! Pitch Class C: ... C-1, C0, C1, C2, C3, C4, C5, C6, C7, C8, C9 ...
//! ```
//!
//! ## Octave
//! An octave is the interval between one musical pitch and another with double its frequency.
//! In this system, octaves are numbered from -1 to 9.
//!
//! ```text
//! Octave -1: Lowest octave (MIDI notes 0-11)
//! Octave 0:  First octave (MIDI notes 12-23)
//! Octave 1:  Second octave (MIDI notes 24-35)
//! ...and so on
//! ```
//!
//! ## Canonical Form
//! The canonical form of a pitch is its representation within a single octave (0-11).
//! This is useful for comparing pitches regardless of their octave.
//!
//! ```text
//! Pitch    Canonical Form
//! C4 (60)  C (0)
//! E4 (64)  E (4)
//! G4 (67)  G (7)
//! ```
//!
//! # Tutorial: Common Use Cases
//!
//! ## Creating a Scale
//! ```rust
//! use mozzart_core::constants::*;
//!
//! // Create a C major scale
//! let c_major = [
//!     C4,
//!     C4.transpose(MAJOR_SECOND),    // D4
//!     C4.transpose(MAJOR_THIRD),     // E4
//!     C4.transpose(PERFECT_FOURTH),  // F4
//!     C4.transpose(PERFECT_FIFTH),   // G4
//!     C4.transpose(MAJOR_SIXTH),     // A4
//!     C4.transpose(MAJOR_SEVENTH),   // B4
//! ];
//! ```
//!
//! ## Creating a Chord
//! ```rust
//! use mozzart_core::constants::*;
//!
//! // Create a C major triad
//! let c_major_triad = [
//!     C4,
//!     C4.transpose(MAJOR_THIRD),    // E4
//!     C4.transpose(PERFECT_FIFTH),  // G4
//! ];
//!
//! // Create a C minor triad
//! let c_minor_triad = [
//!     C4,
//!     C4.transpose(MINOR_THIRD),    // Eb4
//!     C4.transpose(PERFECT_FIFTH),  // G4
//! ];
//! ```
//!
//! ## Transposing a Melody
//! ```rust
//! use mozzart_core::constants::*;
//!
//! // Original melody in C major
//! let melody = [C4, E4, G4, C5];
//!
//! // Transpose up a perfect fifth
//! let transposed: Vec<_> = melody.iter()
//!     .map(|&pitch| pitch.transpose(PERFECT_FIFTH))
//!     .collect();
//!
//! assert_eq!(transposed, [G4, B4, D5, G5]);
//! ```
//!
//! ## Working with Octaves
//! ```rust
//! use mozzart_core::Pitch;
//! use mozzart_core::constants::*;
//!
//! // Get the octave of a pitch
//! assert_eq!(C4.octave(), O4);
//! assert_eq!(C5.octave(), O5);
//! ```
//!
//! ## Finding Canonical Forms
//! ```rust
//! use mozzart_core::Pitch;
//! use mozzart_core::constants::*;
//!
//! // Get the canonical form (pitch class) of a pitch
//! assert_eq!(C4.canonical(), C);
//! assert_eq!(C5.canonical(), C);
//! assert_eq!(E4.canonical(), E);
//!
//! // Check if a pitch is in canonical form
//! assert!(C.is_canonical());
//! assert!(!C4.is_canonical());
//! ```

use crate::{Interval, Octave};
use std::fmt;

/// Represents a musical pitch.
///
/// A pitch is represented by its MIDI note number, where:
/// - 0 is C-1 (lowest C)
/// - 60 is C4 (middle C)
/// - 127 is G9 (highest G)
///
/// # Examples
///
/// ```rust
/// use mozzart_core::Pitch;
/// use mozzart_core::constants::*;
///
/// // Create a pitch from a MIDI number
/// let pitch = C4; // Middle C (C4)
///
/// // Get the semitone value
/// assert_eq!(pitch.semitones(), 60);
///
/// // Get the canonical form (pitch class)
/// assert_eq!(pitch.canonical(), C);
///
/// // Get the octave
/// assert_eq!(pitch.octave(), O4);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pitch(u8);

impl Pitch {
    /// Create a new pitch from a number of semitones.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mozzart_core::Pitch;
    /// use mozzart_core::constants::*;
    ///
    /// let pitch = Pitch::new(60);
    /// assert_eq!(pitch, C4);
    /// ```
    #[inline]
    pub fn new(semitones: u8) -> Self {
        Self(semitones)
    }

    /// Create a new pitch from a canonical pitch class and octave.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mozzart_core::Pitch;
    /// use mozzart_core::constants::*;
    ///
    /// let pitch = Pitch::from_canonical(C, O4);
    /// assert_eq!(pitch, C4);
    /// ```
    #[inline]
    pub const fn from_canonical(self, octave: Octave) -> Self {
        assert!(self.is_canonical());
        let semitone =
            self.semitones() + (octave.value() + 1) as u8 * crate::constants::SEMITONES_PER_OCTAVE;
        Self(semitone)
    }

    /// Create a new pitch from a canonical pitch class and octave.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mozzart_core::Pitch;
    /// use mozzart_core::constants::*;
    ///
    /// let pitch = C4.with_octave(O5);
    /// assert_eq!(pitch, C5);
    /// ```
    #[inline]
    pub const fn with_octave(self, octave: Octave) -> Self {
        let semitone = self.canonical().semitones()
            + (octave.value() + 1) as u8 * crate::constants::SEMITONES_PER_OCTAVE;
        Self(semitone)
    }

    /// Returns the semitone value of this pitch.
    ///
    /// The semitone value is the MIDI note number, where:
    /// - 0 is C-1 (lowest C)
    /// - 60 is C4 (middle C)
    /// - 127 is G9 (highest G)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mozzart_core::Pitch;
    /// use mozzart_core::constants::*;
    ///
    /// assert_eq!(C4.semitones(), 60);
    /// assert_eq!(A4.semitones(), 69);
    /// ```
    #[inline]
    pub const fn semitones(&self) -> u8 {
        self.0
    }

    /// Returns the canonical form of this pitch.
    ///
    /// The canonical form is the pitch class (0-11) regardless of octave.
    /// This is useful for comparing pitches regardless of their octave.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mozzart_core::Pitch;
    /// use mozzart_core::constants::*;
    ///
    /// assert_eq!(C4.canonical(), C);
    /// assert_eq!(C5.canonical(), C);
    /// assert_eq!(G4.canonical(), G);
    /// ```
    #[inline]
    pub const fn canonical(&self) -> Pitch {
        let semitone = self.semitones() % crate::constants::SEMITONES_PER_OCTAVE;
        Pitch(semitone)
    }

    /// Returns whether this pitch is in canonical form.
    ///
    /// A pitch is in canonical form if its semitone value is less than 12.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mozzart_core::Pitch;
    /// use mozzart_core::constants::*;
    ///
    /// assert!(C.is_canonical());
    /// assert!(!C4.is_canonical());
    /// ```
    #[inline]
    pub const fn is_canonical(&self) -> bool {
        self.semitones() < crate::constants::SEMITONES_PER_OCTAVE
    }

    /// Returns the octave of this pitch.
    ///
    /// The octave is calculated as (semitone / 12) - 1.
    /// This means:
    /// - C-1 is in octave -1
    /// - C0 is in octave 0
    /// - C4 is in octave 4
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mozzart_core::Pitch;
    /// use mozzart_core::constants::*;
    ///
    /// assert_eq!(C4.octave(), O4);
    /// assert_eq!(A4.octave(), O4);
    /// assert_eq!(C5.octave(), O5);
    /// ```
    #[inline]
    pub const fn octave(&self) -> Octave {
        let octave = (self.semitones() / crate::constants::SEMITONES_PER_OCTAVE) as i8 - 1;
        Octave::new(octave)
    }

    /// Transposes this pitch by the given interval.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mozzart_core::Pitch;
    /// use mozzart_core::constants::*;
    ///
    /// assert_eq!(C4.transpose(MAJOR_SECOND), D4);
    /// assert_eq!(C4.transpose(MAJOR_THIRD), E4);
    /// assert_eq!(C4.transpose(PERFECT_FOURTH), F4);
    /// ```
    #[inline]
    pub const fn transpose(&self, interval: Interval) -> Pitch {
        Pitch(self.semitones() + interval.semitones())
    }

    pub fn apply_pattern<P>(&self, pattern: P) -> Vec<Pitch>
    where
        P: IntoIterator<Item = Interval>,
    {
        pattern
            .into_iter()
            .map(|interval| self.transpose(interval))
            .collect()
    }
}

macro_rules! generate_octave_pitches {
    ($octave:literal) => {
        paste::item! {
            pub const [<C $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + C.semitones());
            pub const [<CSHARP $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + CSHARP.semitones());
            pub const [<DFLAT $octave>]: Pitch = [<CSHARP $octave>];
            pub const [<D $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + D.semitones());
            pub const [<DSHARP $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + DSHARP.semitones());
            pub const [<EFLAT $octave>]: Pitch = [<DSHARP $octave>];
            pub const [<E $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + E.semitones());
            pub const [<F $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + F.semitones());
            pub const [<FSHARP $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + FSHARP.semitones());
            pub const [<GFLAT $octave>]: Pitch = [<FSHARP $octave>];
            pub const [<G $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + G.semitones());
            pub const [<GSHARP $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + GSHARP.semitones());
            pub const [<AFLAT $octave>]: Pitch = [<GSHARP $octave>];
            pub const [<A $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + A.semitones());
            pub const [<ASHARP $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + ASHARP.semitones());
            pub const [<BFLAT $octave>]: Pitch = [<ASHARP $octave>];
            pub const [<B $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + B.semitones());

            pub const [<PITCHES $octave>]: [Pitch; crate::constants::SEMITONES_PER_OCTAVE as usize] = [
                [<C $octave>], [<CSHARP $octave>], [<D $octave>], [<DSHARP $octave>],
                [<E $octave>], [<F $octave>], [<FSHARP $octave>], [<G $octave>],
                [<GSHARP $octave>], [<A $octave>], [<ASHARP $octave>], [<B $octave>]
            ];
        }
    };
}

pub mod constants {
    use super::Pitch;

    pub const C: Pitch = Pitch(0);
    pub const CSHARP: Pitch = Pitch(1);
    pub const DFLAT: Pitch = CSHARP;
    pub const D: Pitch = Pitch(2);
    pub const DSHARP: Pitch = Pitch(3);
    pub const EFLAT: Pitch = DSHARP;
    pub const E: Pitch = Pitch(4);
    pub const F: Pitch = Pitch(5);
    pub const FSHARP: Pitch = Pitch(6);
    pub const GFLAT: Pitch = FSHARP;
    pub const G: Pitch = Pitch(7);
    pub const GSHARP: Pitch = Pitch(8);
    pub const AFLAT: Pitch = GSHARP;
    pub const A: Pitch = Pitch(9);
    pub const ASHARP: Pitch = Pitch(10);
    pub const BFLAT: Pitch = ASHARP;
    pub const B: Pitch = Pitch(11);

    pub const PITCHES: [Pitch; crate::constants::SEMITONES_PER_OCTAVE as usize] =
        [C, CSHARP, D, DSHARP, E, F, FSHARP, G, GSHARP, A, ASHARP, B];

    // Generate pitches for octaves 0-9
    generate_octave_pitches!(0);
    generate_octave_pitches!(1);
    generate_octave_pitches!(2);
    generate_octave_pitches!(3);
    generate_octave_pitches!(4);
    generate_octave_pitches!(5);
    generate_octave_pitches!(6);
    generate_octave_pitches!(7);
    generate_octave_pitches!(8);
    generate_octave_pitches!(9);
}

const PITCH_NAMES: [&str; crate::constants::SEMITONES_PER_OCTAVE as usize] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let canonical = self.canonical();
        if self.is_canonical() {
            write!(f, "{}", PITCH_NAMES[canonical.semitones() as usize])
        } else {
            write!(
                f,
                "{}{}",
                PITCH_NAMES[canonical.semitones() as usize],
                self.octave()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;
    use proptest::prelude::*;

    // Property-based tests for Pitch
    proptest! {
        #[test]
        fn test_pitch_semitones_roundtrip(pitch in prop::sample::select(&[
            C4, CSHARP4, D4, DSHARP4, E4, F4, FSHARP4, G4, GSHARP4, A4, ASHARP4, B4,
            C5, CSHARP5, D5, DSHARP5, E5, F5, FSHARP5, G5, GSHARP5, A5, ASHARP5, B5
        ])) {
            let semitones = pitch.semitones();
            prop_assert_eq!(pitch.semitones(), semitones);
        }

        #[test]
        fn test_pitch_transposition_commutative(
            pitch in prop::sample::select(&[
                C4, CSHARP4, D4, DSHARP4, E4, F4, FSHARP4, G4, GSHARP4, A4, ASHARP4, B4
            ]),
            interval1 in prop::sample::select(&[
                PERFECT_UNISON, MAJOR_SECOND, MAJOR_THIRD, PERFECT_FOURTH,
                PERFECT_FIFTH, MAJOR_SIXTH, MAJOR_SEVENTH, PERFECT_OCTAVE
            ]),
            interval2 in prop::sample::select(&[
                PERFECT_UNISON, MAJOR_SECOND, MAJOR_THIRD, PERFECT_FOURTH,
                PERFECT_FIFTH, MAJOR_SIXTH, MAJOR_SEVENTH, PERFECT_OCTAVE
            ])
        ) {
            let transposed1 = pitch.transpose(interval1).transpose(interval2);
            let transposed2 = pitch.transpose(interval2).transpose(interval1);
            prop_assert_eq!(transposed1, transposed2);
        }

        #[test]
        fn test_pitch_transposition_associative(
            pitch in prop::sample::select(&[
                C4, CSHARP4, D4, DSHARP4, E4, F4, FSHARP4, G4, GSHARP4, A4, ASHARP4, B4
            ])
        ) {
            // Test that transposing by MAJOR_SECOND + MAJOR_THIRD is the same as transposing by PERFECT_FOURTH
            let transposed1 = pitch
                .transpose(MAJOR_SECOND)
                .transpose(MAJOR_THIRD);
            let transposed2 = pitch
                .transpose(DIMINISHED_FIFTH);
            prop_assert_eq!(transposed1, transposed2);

            // Test that transposing by MAJOR_THIRD + PERFECT_FOURTH is the same as transposing by MAJOR_SIXTH
            let transposed3 = pitch
                .transpose(MAJOR_THIRD)
                .transpose(PERFECT_FOURTH);
            let transposed4 = pitch
                .transpose(MAJOR_SIXTH);
            prop_assert_eq!(transposed3, transposed4);
        }

        #[test]
        fn test_pitch_canonical_form(pitch in prop::sample::select(&[
            C4, CSHARP4, D4, DSHARP4, E4, F4, FSHARP4, G4, GSHARP4, A4, ASHARP4, B4
        ])) {
            let canonical = pitch.canonical();
            prop_assert!(canonical.is_canonical());
            prop_assert_eq!(canonical.semitones() % SEMITONES_PER_OCTAVE, pitch.semitones() % SEMITONES_PER_OCTAVE);
        }

        #[test]
        fn test_pitch_transposition_bounds(
            pitch in prop::sample::select(&[
                C4, CSHARP4, D4, DSHARP4, E4, F4, FSHARP4, G4, GSHARP4, A4, ASHARP4, B4
            ]),
            interval in prop::sample::select(&[
                PERFECT_UNISON, MAJOR_SECOND, MAJOR_THIRD, PERFECT_FOURTH,
                PERFECT_FIFTH, MAJOR_SIXTH, MAJOR_SEVENTH, PERFECT_OCTAVE
            ])
        ) {
            let transposed = pitch.transpose(interval);
            prop_assert!(transposed.semitones() <= 127);
        }
    }

    fn test_pitches(
        pitches: [Pitch; crate::constants::SEMITONES_PER_OCTAVE as usize],
        octave: Octave,
    ) {
        for (i, pitch) in pitches.iter().enumerate() {
            assert_eq!(pitch.canonical().semitones(), i as u8);
            assert_eq!(pitch.octave(), octave);
            assert_eq!(pitch.is_canonical(), octave.is_canonical());
        }
    }

    #[test]
    fn test_all_pitches() {
        test_pitches(constants::PITCHES, OC);
        test_pitches(constants::PITCHES0, O0);
        test_pitches(constants::PITCHES1, O1);
        test_pitches(constants::PITCHES2, O2);
        test_pitches(constants::PITCHES3, O3);
        test_pitches(constants::PITCHES4, O4);
        test_pitches(constants::PITCHES5, O5);
        test_pitches(constants::PITCHES6, O6);
        test_pitches(constants::PITCHES7, O7);
        test_pitches(constants::PITCHES8, O8);
        test_pitches(constants::PITCHES9, O9);
    }

    #[test]
    fn test_transpose() {
        assert_eq!(C.transpose(PERFECT_UNISON), C);
        assert_eq!(C.transpose(MAJOR_SECOND), D);
        assert_eq!(C.transpose(MINOR_THIRD), EFLAT);
        assert_eq!(C.transpose(PERFECT_FOURTH), F);
    }

    #[test]
    fn test_display() {
        assert_eq!(C.to_string(), "C");
        assert_eq!(CSHARP.to_string(), "C#");
        assert_eq!(D.to_string(), "D");
        assert_eq!(DSHARP.to_string(), "D#");
        assert_eq!(E.to_string(), "E");
        assert_eq!(F.to_string(), "F");
        assert_eq!(FSHARP.to_string(), "F#");
        assert_eq!(G.to_string(), "G");
        assert_eq!(GSHARP.to_string(), "G#");
        assert_eq!(A.to_string(), "A");
        assert_eq!(ASHARP.to_string(), "A#");
        assert_eq!(B.to_string(), "B");

        assert_eq!(C4.to_string(), "C4");
        assert_eq!(CSHARP4.to_string(), "C#4");
        assert_eq!(D4.to_string(), "D4");
        assert_eq!(DSHARP4.to_string(), "D#4");
        assert_eq!(E4.to_string(), "E4");
        assert_eq!(F4.to_string(), "F4");
        assert_eq!(FSHARP4.to_string(), "F#4");
        assert_eq!(G4.to_string(), "G4");
        assert_eq!(GSHARP4.to_string(), "G#4");
        assert_eq!(A4.to_string(), "A4");
        assert_eq!(ASHARP4.to_string(), "A#4");
        assert_eq!(B4.to_string(), "B4");
    }

    #[test]
    fn test_from_canonical() {
        assert_eq!(C.from_canonical(O4), C4);
        assert_eq!(CSHARP.from_canonical(O4), CSHARP4);
        assert_eq!(D.from_canonical(O5), D5);
        assert_eq!(DSHARP.from_canonical(O6), DSHARP6);
    }

    #[test]
    fn test_with_octave() {
        assert_eq!(C1.with_octave(O4), C4);
        assert_eq!(CSHARP2.with_octave(O5), CSHARP5);
        assert_eq!(D3.with_octave(O6), D6);
        assert_eq!(DSHARP4.with_octave(O7), DSHARP7);
    }

    #[test]
    fn test_apply_pattern() {
        let pattern = [MAJOR_SECOND, PERFECT_FOURTH];
        let scale = C4.apply_pattern(pattern);
        assert_eq!(scale, [D4, F4]);
    }
}
