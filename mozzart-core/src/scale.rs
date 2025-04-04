use crate::Interval;

pub trait ScaleType{}

pub trait ScalePattern {
    type Pattern: IntoIterator<Item = Interval>;
    const PATTERN: Self::Pattern;

    type ScaleTyp: ScaleType;
    const TYPE: Self::ScaleTyp;
}
