//! Musical octave representation and operations.
//!
//! This module provides functionality for working with musical octaves, including:
//! - Octave representation and validation
//! - Octave constants for all supported octaves
//! - Octave range operations
//!
//! # Octave System
//!
//! The octave system used in this crate follows the MIDI standard:
//!
//! ```text
//! Octave -1 (OC): Lowest octave (MIDI notes 0-11)
//! Octave 0  (O0): First octave (MIDI notes 12-23)
//! Octave 1  (O1): Second octave (MIDI notes 24-35)
//! Octave 2  (O2): Third octave (MIDI notes 36-47)
//! Octave 3  (O3): Fourth octave (MIDI notes 48-59)
//! Octave 4  (O4): Fifth octave (MIDI notes 60-71)  <- Middle C (C4) is here
//! Octave 5  (O5): Sixth octave (MIDI notes 72-83)
//! Octave 6  (O6): Seventh octave (MIDI notes 84-95)
//! Octave 7  (O7): Eighth octave (MIDI notes 96-107)
//! Octave 8  (O8): Ninth octave (MIDI notes 108-119)
//! Octave 9  (O9): Tenth octave (MIDI notes 120-127)
//! ```
//!
//! # Examples
//!
//! Basic octave operations:
//! ```rust
//! use mozzart_core::Octave;
//! use mozzart_core::constants::*;
//!
//! // Create a new octave
//! let octave = O4; // Middle C octave
//!
//! // Check if octave is canonical (negative)
//! assert!(!octave.is_canonical());
//! assert!(OC.is_canonical());
//!
//! ```
//!
//! # Musical Concepts
//!
//! ## Octave
//! An octave is the interval between one musical pitch and another with double its frequency.
//! In this system, octaves are numbered from -1 to 9, where:
//! - Octave -1 (OC) is the lowest octave
//! - Octave 4 (O4) contains middle C
//! - Octave 9 (O9) is the highest octave
//!
//! ```text
//! Frequency Relationship:
//! C4 = 261.63 Hz
//! C5 = 523.25 Hz (double the frequency of C4)
//! C6 = 1046.50 Hz (double the frequency of C5)
//! ```
//!
//! ## Canonical Octave
//! A canonical octave is one that is negative (less than 0).
//! This is used to represent the lowest octave in the system.
//!
//! ```text
//! Canonical:    OC (-1)
//! Non-canonical: O0 (0), O1 (1), O2 (2), etc.
//! ```

use std::fmt;

use crate::Pitch;
/// Represents a musical octave.
///
/// An octave is the interval between one musical pitch and another with double its frequency.
/// In this system, octaves are numbered from -1 to 9, where:
/// - -1 is the lowest octave (OC)
/// - 4 contains middle C (O4)
/// - 9 is the highest octave (O9)
///
/// # Examples
///
/// ```rust
/// use mozzart_core::Octave;
/// use mozzart_core::constants::*;
///
/// // Create a new octave
/// let octave = O4; // Middle C octave
///
/// // Check if octave is canonical
/// assert!(!octave.is_canonical());
/// assert!(OC.is_canonical());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Octave(i8);

impl Octave {
    /// Creates a new octave with the given value.
    ///
    /// The value must be between -1 and 9 inclusive.
    /// - -1 represents the lowest octave (OC)
    /// - 0 represents the first octave (O0)
    /// - 4 represents the middle C octave (O4)
    /// - 9 represents the highest octave (O9)
    ///
    /// # Notes
    ///
    /// This function is private to the crate.
    #[inline]
    pub(crate) const fn new(value: i8) -> Self {
        Self(value)
    }

    /// Returns the value of this octave.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mozzart_core::Octave;
    /// use mozzart_core::constants::*;
    ///
    /// assert_eq!(OC.value(), -1);
    /// assert_eq!(O0.value(), 0);
    /// assert_eq!(O4.value(), 4);
    /// assert_eq!(O9.value(), 9);
    /// ```
    #[inline]
    pub const fn value(&self) -> i8 {
        self.0
    }

    /// Returns whether this octave is canonical.
    ///
    /// A canonical octave is one that is negative (less than 0).
    /// This is used to represent the lowest octave in the system.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mozzart_core::Octave;
    /// use mozzart_core::constants::*;
    ///
    /// assert!(OC.is_canonical());
    /// assert!(!O0.is_canonical());
    /// assert!(!O4.is_canonical());
    /// ```
    #[inline]
    pub const fn is_canonical(&self) -> bool {
        self.0 < 0
    }

    /// Returns a pitch with the given octave.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mozzart_core::Octave;
    /// use mozzart_core::constants::*;
    /// use mozzart_core::Pitch;
    ///
    /// let pitch = OC.update_pitch(C);
    /// assert_eq!(pitch, C0);
    /// ```
    #[inline]
    pub const fn update_octave(self, pitch: Pitch) -> Pitch {
        pitch.with_octave(self)
    }

    /// Returns a pitch with the given octave.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mozzart_core::Octave;
    /// use mozzart_core::constants::*;
    /// use mozzart_core::Pitch;
    ///
    /// let pitch = OC.to_pitch(C);
    /// assert_eq!(pitch, C0);
    /// ```
    #[inline]
    pub const fn to_pitch(self, canonical: Pitch) -> Pitch {
        assert!(canonical.is_canonical());
        canonical.with_octave(self)
    }
}

impl fmt::Display for Octave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Constants for all supported octaves.
///
/// This module provides constants for all octaves from -1 to 9:
/// - `OC`: Octave -1 (lowest octave)
/// - `O0`: Octave 0 (first octave)
/// - `O1`: Octave 1 (second octave)
/// - ...
/// - `O9`: Octave 9 (highest octave)
pub mod constants {
    use super::Octave;

    /// The number of semitones in an octave.
    pub const SEMITONES_PER_OCTAVE: u8 = 12;

    /// The lowest octave (-1).
    pub const OC: Octave = Octave(-1);
    /// The first octave (0).
    pub const O0: Octave = Octave(0);
    /// The second octave (1).
    pub const O1: Octave = Octave(1);
    /// The third octave (2).
    pub const O2: Octave = Octave(2);
    /// The fourth octave (3).
    pub const O3: Octave = Octave(3);
    /// The fifth octave (4).
    pub const O4: Octave = Octave(4);
    /// The sixth octave (5).
    pub const O5: Octave = Octave(5);
    /// The seventh octave (6).
    pub const O6: Octave = Octave(6);
    /// The eighth octave (7).
    pub const O7: Octave = Octave(7);
    /// The ninth octave (8).
    pub const O8: Octave = Octave(8);
    /// The tenth octave (9).
    pub const O9: Octave = Octave(9);

    /// An array of all supported octaves.
    pub const OCTAVES: [Octave; 11] = [OC, O0, O1, O2, O3, O4, O5, O6, O7, O8, O9];
}

#[cfg(test)]
mod tests {
    use crate::constants::*;

    use super::*;

    #[test]
    fn test_octaves() {
        for (i, octave) in constants::OCTAVES.iter().enumerate() {
            assert_eq!(octave.0, i as i8 - 1);
            if i == 0 {
                assert!(octave.is_canonical());
            } else {
                assert!(!octave.is_canonical());
            }
        }
    }

    #[test]
    fn test_update_octave() {
        let pitch = O4.update_octave(C1);
        assert_eq!(pitch, C4);
    }

    #[test]
    fn test_to_pitch() {
        let pitch = O4.to_pitch(C);
        assert_eq!(pitch, C4);
    }
}
