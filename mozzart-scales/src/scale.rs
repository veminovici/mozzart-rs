use mozzart_core::{Interval, Note};

/// Represents a musical scale
#[derive(Debug, Clone)]
pub struct Scale {
    root: Note,
    intervals: Vec<Interval>,
}

impl Scale {
    /// Creates a new scale with the given root note and intervals
    pub fn new(root: Note, intervals: Vec<Interval>) -> Self {
        Self { root, intervals }
    }

    /// Returns the root note of the scale
    pub fn root(&self) -> Note {
        self.root
    }

    /// Returns the intervals that make up this scale
    pub fn intervals(&self) -> &[Interval] {
        &self.intervals
    }

    /// Returns all notes in the scale
    pub fn notes(&self) -> Vec<Note> {
        let mut notes = Vec::with_capacity(self.intervals.len() + 1);
        notes.push(self.root);

        let current_note = self.root;
        for _interval in &self.intervals {
            // TODO: Implement note transposition
            notes.push(current_note);
        }

        notes
    }
}
