//! Core music theory functionality
//!
//! This crate provides fundamental music theory concepts and structures.

pub mod interval;
pub mod octave;
pub mod pitch;
pub mod scale;

pub use interval::Interval;
pub use octave::Octave;
pub use pitch::Pitch;
pub use scale::{ScalePattern, ScaleType};

pub mod constants {
    pub use crate::interval::constants::*;
    pub use crate::octave::constants::*;
    pub use crate::pitch::constants::*;
}

