//! Core music theory functionality
//!
//! This crate provides fundamental music theory concepts and structures.

mod chord;
mod interval;
mod octave;
mod pitch;
mod scale;

pub use chord::{ChordPattern, ChordType};
pub use interval::Interval;
pub use octave::Octave;
pub use pitch::Pitch;
pub use scale::{Scale, ScalePattern, ScaleType};

pub mod constants {
    pub use crate::interval::constants::*;
    pub use crate::octave::constants::*;
    pub use crate::pitch::constants::*;
}
