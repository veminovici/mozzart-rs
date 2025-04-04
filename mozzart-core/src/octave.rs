#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Octave(i8);

impl Octave {
    #[inline]
    pub(crate) const fn new(value: i8) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn is_canonical(&self) -> bool {
        self.0 < 0
    }
}

pub mod constants {
    use super::Octave;

    pub const SEMITONES_PER_OCTAVE: u8 = 12;

    pub const OC: Octave = Octave(-1);
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

    pub const OCTAVES: [Octave; 11] = [
        OC, O0, O1, O2, O3, O4, O5, O6, O7, O8, O9
    ];
}

#[cfg(test)]
mod tests {
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
}