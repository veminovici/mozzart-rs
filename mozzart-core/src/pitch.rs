use crate::{Interval, Octave};

/// Represents a musical pitch
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pitch(u8);

impl Pitch {
    #[inline]
    pub const fn semitones(&self) -> u8 {
        self.0
    }

    #[inline]
    pub const fn canonical(&self) -> Pitch {
        let semitone = self.semitones() % crate::constants::SEMITONES_PER_OCTAVE;
        Pitch(semitone)
    }

    #[inline]
    pub const fn is_canonical(&self) -> bool {
        self.semitones() < crate::constants::SEMITONES_PER_OCTAVE
    }

    #[inline]
    pub const fn octave(&self) -> Octave {
        let octave = (self.semitones() / crate::constants::SEMITONES_PER_OCTAVE) as i8 - 1;
        Octave::new(octave)
    }

    #[inline]
    pub const fn transpose(&self, interval: Interval) -> Pitch {
        Pitch(self.semitones() + interval.semitones())
    }
}

macro_rules! generate_octave_pitches {
    ($octave:literal) => {
        paste::item! {
            pub const [<C $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + C.semitones());
            pub const [<CSHARP $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + CSHARP.semitones());
            pub const [<DFLAT $octave>]: Pitch = [<CSHARP $octave>];
            pub const [<D $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + D.semitones());
            pub const [<DSHARP $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + DSHARP.semitones());
            pub const [<EFLAT $octave>]: Pitch = [<DSHARP $octave>];
            pub const [<E $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + E.semitones());
            pub const [<F $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + F.semitones());
            pub const [<FSHARP $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + FSHARP.semitones());
            pub const [<GFLAT $octave>]: Pitch = [<FSHARP $octave>];
            pub const [<G $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + G.semitones());
            pub const [<GSHARP $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + GSHARP.semitones());
            pub const [<AFLAT $octave>]: Pitch = [<GSHARP $octave>];
            pub const [<A $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + A.semitones());
            pub const [<ASHARP $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + ASHARP.semitones());
            pub const [<BFLAT $octave>]: Pitch = [<ASHARP $octave>];
            pub const [<B $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * ($octave + 1) + B.semitones());

            pub const [<PITCHES $octave>]: [Pitch; crate::constants::SEMITONES_PER_OCTAVE as usize] = [
                [<C $octave>], [<CSHARP $octave>], [<D $octave>], [<DSHARP $octave>],
                [<E $octave>], [<F $octave>], [<FSHARP $octave>], [<G $octave>],
                [<GSHARP $octave>], [<A $octave>], [<ASHARP $octave>], [<B $octave>]
            ];
        }
    };
}

pub mod constants {
    use super::Pitch;

    pub const C: Pitch = Pitch(0);
    pub const CSHARP: Pitch = Pitch(1);
    pub const DFLAT: Pitch = CSHARP;
    pub const D: Pitch = Pitch(2);
    pub const DSHARP: Pitch = Pitch(3);
    pub const EFLAT: Pitch = DSHARP;
    pub const E: Pitch = Pitch(4);
    pub const F: Pitch = Pitch(5);
    pub const FSHARP: Pitch = Pitch(6);
    pub const GFLAT: Pitch = FSHARP;
    pub const G: Pitch = Pitch(7);
    pub const GSHARP: Pitch = Pitch(8);
    pub const AFLAT: Pitch = GSHARP;
    pub const A: Pitch = Pitch(9);
    pub const ASHARP: Pitch = Pitch(10);
    pub const BFLAT: Pitch = ASHARP;
    pub const B: Pitch = Pitch(11);

    pub const PITCHES: [Pitch; crate::constants::SEMITONES_PER_OCTAVE as usize] = [
        C, CSHARP, D, DSHARP, E, F, FSHARP, G, GSHARP, A, ASHARP, B
    ];

    // Generate pitches for octaves 0-9
    generate_octave_pitches!(0);
    generate_octave_pitches!(1);
    generate_octave_pitches!(2);
    generate_octave_pitches!(3);
    generate_octave_pitches!(4);
    generate_octave_pitches!(5);
    generate_octave_pitches!(6);
    generate_octave_pitches!(7);
    generate_octave_pitches!(8);
    generate_octave_pitches!(9);
}

#[cfg(test)]
mod tests {
    use crate::constants::*;
    use super::*;

    fn test_pitches(pitches: [Pitch; crate::constants::SEMITONES_PER_OCTAVE as usize], octave: Octave) {
        for (i, pitch) in pitches.iter().enumerate() {
            assert_eq!(pitch.canonical().semitones(), i as u8);
            assert_eq!(pitch.octave(), octave);
            assert_eq!(pitch.is_canonical(), octave.is_canonical());
        }
    }

    #[test]
    fn test_all_pitches() {
        test_pitches(constants::PITCHES, OC);
        test_pitches(constants::PITCHES0, O0);
        test_pitches(constants::PITCHES1, O1);
        test_pitches(constants::PITCHES2, O2);
        test_pitches(constants::PITCHES3, O3);
        test_pitches(constants::PITCHES4, O4);
        test_pitches(constants::PITCHES5, O5);
        test_pitches(constants::PITCHES6, O6);
        test_pitches(constants::PITCHES7, O7);
        test_pitches(constants::PITCHES8, O8);
        test_pitches(constants::PITCHES9, O9);
    }

    #[test]
    fn test_transpose() {
        assert_eq!(C.transpose(PERFECT_UNISON), C);
        assert_eq!(C.transpose(MAJOR_SECOND), D);
        assert_eq!(C.transpose(MINOR_THIRD), EFLAT);
        assert_eq!(C.transpose(PERFECT_FOURTH), F);
    }
}        
