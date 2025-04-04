//! Musical chord representation and operations.
//!
//! This module provides functionality for working with musical chords, including:
//! - Chord type definitions
//! - Chord pattern implementations
//! - Common chord constants
//!
//! # Chord System
//!
//! A musical chord is a group of notes played simultaneously. In this system,
//! chords are defined by their type and pattern of intervals from the root note:
//!
//! ```text
//! Common Chord Types and Patterns:
//! Major Triad:      Root + Major Third + Perfect Fifth
//! Minor Triad:      Root + Minor Third + Perfect Fifth
//! Diminished Triad: Root + Minor Third + Diminished Fifth
//! Augmented Triad:  Root + Major Third + Augmented Fifth
//! ```
//!
//! # Examples
//!
//! Basic chord operations:
//! ```rust
//! use mozzart_core::chord::*;
//! use mozzart_core::interval::constants::*;
//! use mozzart_core::pitch::constants::*;
//!
//! // Create a C major triad
//! let root = C4;
//! let major_third = root.transpose(MAJOR_THIRD);
//! let perfect_fifth = root.transpose(PERFECT_FIFTH);
//!
//! // The notes of a C major triad are C, E, G
//! assert_eq!(major_third, E4);
//! assert_eq!(perfect_fifth, G4);
//! ```
//!
//! # Musical Concepts
//!
//! ## Chord Types
//! Chords are classified by their quality and structure:
//! - Triads: three-note chords (major, minor, diminished, augmented)
//! - Seventh chords: four-note chords (major 7th, minor 7th, dominant 7th)
//! - Extended chords: five or more notes (9th, 11th, 13th)
//!
//! ## Chord Patterns
//! A chord pattern defines the intervals that make up a chord:
//! ```text
//! Major Triad Pattern:
//! Root (0) + Major Third (4) + Perfect Fifth (7)
//!
//! Minor Triad Pattern:
//! Root (0) + Minor Third (3) + Perfect Fifth (7)
//! ```

use crate::Interval;

/// A marker trait for chord types.
///
/// This trait is used to distinguish between different types of chords
/// (e.g., major, minor, diminished, augmented) at the type level.
///
/// # Examples
///
/// ```rust
/// use mozzart_core::chord::ChordType;
///
/// struct Major;
/// impl ChordType for Major {}
///
/// struct Minor;
/// impl ChordType for Minor {}
/// ```
pub trait ChordType {}

/// A trait for defining chord patterns.
///
/// This trait allows defining the structure of a chord through its interval pattern
/// and associated chord type. Implementations should specify:
/// - The pattern of intervals that make up the chord
/// - The type of the chord (major, minor, etc.)
///
/// # Examples
///
/// ```rust
/// use mozzart_core::{Interval, ChordPattern, ChordType};
/// use mozzart_core::interval::constants::*;
///
/// struct MajorTriadType;
/// impl ChordType for MajorTriadType {}
///
/// struct MajorTriadPattern;
/// impl ChordPattern for MajorTriadPattern {
///     type Pattern = [Interval; 2];
///     const PATTERN: Self::Pattern = [MAJOR_THIRD, PERFECT_FIFTH];
///     type ChordTyp = MajorTriadType;
///     const TYPE: Self::ChordTyp = MajorTriadType;
/// }
/// ```
pub trait ChordPattern {
    /// The type of the interval pattern.
    /// This is typically an array or vector of intervals.
    type Pattern: IntoIterator<Item = Interval>;

    /// The interval pattern that defines the chord.
    /// This specifies the sequence of intervals from the root note
    /// that make up the chord.
    const PATTERN: Self::Pattern;

    /// The type of the chord.
    /// This associates the pattern with a specific chord type
    /// (e.g., major, minor, diminished).
    type ChordTyp: ChordType;

    /// The chord type instance.
    /// This provides a concrete instance of the chord type.
    const TYPE: Self::ChordTyp;
}


