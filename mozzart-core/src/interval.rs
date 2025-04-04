/// Represents a musical interval
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval(u8);

impl Interval {
    #[inline]
    pub const fn semitones(&self) -> u8 {
        self.0
    }
    
}

pub mod constants {
    use super::Interval;

    pub const PERFECT_UNISON: Interval = Interval(0);
    pub const MINOR_SECOND: Interval = Interval(1);
    pub const MAJOR_SECOND: Interval = Interval(2);
    pub const MINOR_THIRD: Interval = Interval(3);
    pub const MAJOR_THIRD: Interval = Interval(4);
    pub const PERFECT_FOURTH: Interval = Interval(5);
    pub const DIMINISHED_FIFTH: Interval = Interval(6);
    pub const PERFECT_FIFTH: Interval = Interval(7);
    pub const MINOR_SIXTH: Interval = Interval(8);
    pub const MAJOR_SIXTH: Interval = Interval(9);
    pub const MINOR_SEVENTH: Interval = Interval(10);
    pub const MAJOR_SEVENTH: Interval = Interval(11);
    pub const PERFECT_OCTAVE: Interval = Interval(12);
    
}
