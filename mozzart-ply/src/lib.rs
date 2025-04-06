use std::marker::PhantomData;

pub trait Transposable<U> {
    type Output;

    fn transpose(&self, other: U) -> Self::Output;
}

pub trait Pattern {
    type Distance;
    type Pattern: IntoIterator<Item = Self::Distance>;
    const PATTERN: Self::Pattern;

    fn apply<T>(root: T) -> Vec<T::Output>
    where
        T: Transposable<Self::Distance>,
    {
        Self::PATTERN
            .into_iter()
            .map(move |interval| root.transpose(interval))
            .collect()
    }
}

pub trait ScaleType {
    fn name() -> &'static str;
}

pub trait ScalePattern: Pattern {
    type Type: ScaleType;

    fn to_scale(root: Pitch) -> Scale<Self::Type> {
        let pitches = MajorScalePattern::apply(root);
        Scale::<Self::Type>::new(pitches)
    }
}

pub struct MajorScaleType;
impl ScaleType for MajorScaleType {
    fn name() -> &'static str {
        "major"
    }
}

pub struct MajorScalePattern;
impl Pattern for MajorScalePattern {
    type Distance = Interval;
    type Pattern = [Interval; 7];
    const PATTERN: Self::Pattern = [
        Interval(0),
        Interval(2),
        Interval(4),
        Interval(5),
        Interval(7),
        Interval(9),
        Interval(11),
    ];
}
impl ScalePattern for MajorScalePattern {
    type Type = MajorScaleType;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pitch(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Interval(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Octave(i8);

impl Transposable<Interval> for Pitch {
    type Output = Pitch;

    fn transpose(&self, other: Interval) -> Self::Output {
        let semitones = self.0 + other.0;
        Pitch(semitones)
    }
}

impl Transposable<Octave> for Pitch {
    type Output = Pitch;

    fn transpose(&self, other: Octave) -> Self::Output {
        let semitones = self.0 as i8 + other.0 * 12;
        Pitch(semitones as u8)
    }
}

pub struct Scale<T: ScaleType> {
    pitches: Vec<Pitch>,
    typ: PhantomData<T>,
}

impl<T: ScaleType> Scale<T> {
    pub fn new(pitches: Vec<Pitch>) -> Self {
        Self {
            pitches,
            typ: PhantomData,
        }
    }

    pub fn pitches(&self) -> &[Pitch] {
        &self.pitches
    }

    pub fn root(&self) -> &Pitch {
        &self.pitches[0]
    }

    pub fn name(&self) -> String {
        format!("{:?} {}", self.root(), T::name())
    }
}
