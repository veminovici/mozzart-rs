use crate::{Interval, Pitch};

pub trait ScaleType{}

pub trait ScalePattern {
    type Pattern: IntoIterator<Item = Interval>;
    const PATTERN: Self::Pattern;

    type ScaleTyp: ScaleType;
    const TYPE: Self::ScaleTyp;

    #[inline]
    fn apply(root: Pitch) -> Vec<Pitch> {
        apply_pattern(root, Self::PATTERN)
    }
}

fn apply_pattern<P>(root: Pitch, pattern: P) -> Vec<Pitch>
where 
    P: IntoIterator<Item = Interval>
{
    let mut pitches = Vec::new();

    for interval in pattern.into_iter() {
        pitches.push(root.transpose(interval));
    }
    pitches
}

#[cfg(test)]
mod tests {
    use crate::constants::{MAJOR_SECOND, PERFECT_FOURTH};

    use super::*;
    use crate::constants::*;

    #[test]
    fn test_apply() {
        let pattern = [MAJOR_SECOND, PERFECT_FOURTH];
        let root = C;
        let scale = apply_pattern(root, pattern);
        assert_eq!(scale, [D, F]);
    }    
}
