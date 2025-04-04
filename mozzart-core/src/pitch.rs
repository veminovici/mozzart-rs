/// Represents a musical pitch
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pitch(u8);

impl Pitch {
    #[inline]
    pub const fn semitone(&self) -> u8 {
        self.0
    }

    #[inline]
    pub const fn canonical(&self) -> Pitch {
        let semitone = self.semitone() % crate::constants::SEMITONES_PER_OCTAVE;
        Pitch(semitone)
    }
}

macro_rules! generate_pitches {
    ($octave:expr) => {
        paste::item! {
            pub const [<C $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * $octave + C.semitone());
            pub const [<CSHARP $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * $octave + CSHARP.semitone());
            pub const [<DFLAT $octave>]: Pitch = [<CSHARP $octave>];
            pub const [<D $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * $octave + D.semitone());
            pub const [<DSHARP $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * $octave + DSHARP.semitone());
            pub const [<EFLAT $octave>]: Pitch = [<DSHARP $octave>];
            pub const [<E $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * $octave + E.semitone());
            pub const [<F $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * $octave + F.semitone());
            pub const [<FSHARP $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * $octave + FSHARP.semitone());
            pub const [<GFLAT $octave>]: Pitch = [<FSHARP $octave>];
            pub const [<G $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * $octave + G.semitone());
            pub const [<GSHARP $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * $octave + GSHARP.semitone());
            pub const [<AFLAT $octave>]: Pitch = [<GSHARP $octave>];
            pub const [<A $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * $octave + A.semitone());
            pub const [<ASHARP $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * $octave + ASHARP.semitone());
            pub const [<BFLAT $octave>]: Pitch = [<ASHARP $octave>];
            pub const [<B $octave>]: Pitch = Pitch(crate::constants::SEMITONES_PER_OCTAVE * $octave + B.semitone());

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
    generate_pitches!(0);
    generate_pitches!(1);
    generate_pitches!(2);
    generate_pitches!(3);
    generate_pitches!(4);
    generate_pitches!(5);
    generate_pitches!(6);
    generate_pitches!(7);
    generate_pitches!(8);
    generate_pitches!(9);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_pitches(pitches: [Pitch; crate::constants::SEMITONES_PER_OCTAVE as usize]) {
        for (i, pitch) in pitches.iter().enumerate() {
            let canonical = pitch.canonical();
            assert_eq!(canonical.semitone(), i as u8);
        }
    }

    #[test]
    fn test_canonical_pitches() {
        test_pitches(constants::PITCHES);
        test_pitches(constants::PITCHES0);
        test_pitches(constants::PITCHES1);
        test_pitches(constants::PITCHES2);
        test_pitches(constants::PITCHES3);
        test_pitches(constants::PITCHES4);
        test_pitches(constants::PITCHES5);
        test_pitches(constants::PITCHES6);
        test_pitches(constants::PITCHES7);
        test_pitches(constants::PITCHES8);
        test_pitches(constants::PITCHES9);
    }
}
