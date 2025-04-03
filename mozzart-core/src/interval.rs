/// Represents a musical interval
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    semitones: i8,
}

impl Interval {
    /// Creates a new interval with the given number of semitones
    pub fn new(semitones: i8) -> Self {
        Self { semitones }
    }
}
