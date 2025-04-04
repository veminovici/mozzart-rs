#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Octave(u8);

pub mod constants {
    use super::Octave;

    pub const SEMITONES_PER_OCTAVE: u8 = 12;

    pub const O0: Octave = Octave(0);
    pub const O1: Octave = Octave(1);
    pub const O2: Octave = Octave(2);
    pub const O3: Octave = Octave(3);
    pub const O4: Octave = Octave(4);
    pub const O5: Octave = Octave(5);
    pub const O6: Octave = Octave(6);
    pub const O7: Octave = Octave(7);
    pub const O8: Octave = Octave(8);
    pub const O9: Octave = Octave(9);

    pub const OCTAVES: [Octave; 10] = [
        O0, O1, O2, O3, O4, O5, O6, O7, O8, O9
    ];
}