/// Represents a musical note
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Note {
    pitch: u8,
}

impl Note {
    /// Creates a new note with the given pitch (0-127)
    pub fn new(pitch: u8) -> Self {
        assert!(pitch <= 127, "Pitch must be between 0 and 127");
        Self { pitch }
    }
}
