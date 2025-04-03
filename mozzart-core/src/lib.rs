//! Core music theory functionality
//!
//! This crate provides fundamental music theory concepts and structures.

pub mod chord;
pub mod interval;
pub mod note;

pub use chord::Chord;
pub use interval::Interval;
pub use note::Note;
