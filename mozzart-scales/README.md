# Mozzart Scales

A Rust library for working with musical scales, built on top of the `mozzart-core` library. This library provides implementations of various scale types and patterns used in Western music theory.

## Features

- **Heptatonic Scales**
  - Major Scale
  - Natural Minor Scale
  - Harmonic Minor Scale
  - Melodic Minor Scale (with ascending and descending forms)
- **Scale Patterns**
  - Clear interval definitions
  - Type-safe scale construction
  - Easy transposition
- **Musical Theory**
  - Comprehensive documentation
  - Examples and usage patterns
  - ASCII diagrams for visualization

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
mozzart-scales = "0.1.0"
```

### Examples

#### Creating a Major Scale

```rust
use mozzart_scales::heptatonic::major::*;
use mozzart_core::pitch::constants::*;

// Create a C major scale
let c_major = MajorScalePattern::apply(C4);
let pitches = c_major.pitches();

// The notes of a C major scale are C, D, E, F, G, A, B
assert_eq!(pitches[0], C4);
assert_eq!(pitches[1], D4);
assert_eq!(pitches[2], E4);
assert_eq!(pitches[3], F4);
assert_eq!(pitches[4], G4);
assert_eq!(pitches[5], A4);
assert_eq!(pitches[6], B4);
```

#### Creating a Natural Minor Scale

```rust
use mozzart_scales::heptatonic::natural_minor::*;
use mozzart_core::pitch::constants::*;

// Create an A natural minor scale
let a_minor = NaturalMinorScalePattern::apply(A4);
let pitches = a_minor.pitches();

// The notes of an A natural minor scale are A, B, C, D, E, F, G
assert_eq!(pitches[0], A4);
assert_eq!(pitches[1], B4);
assert_eq!(pitches[2], C5);
assert_eq!(pitches[3], D5);
assert_eq!(pitches[4], E5);
assert_eq!(pitches[5], F5);
assert_eq!(pitches[6], G5);
```

#### Creating a Harmonic Minor Scale

```rust
use mozzart_scales::heptatonic::harmonic_minor::*;
use mozzart_core::pitch::constants::*;

// Create an A harmonic minor scale
let a_harmonic_minor = HarmonicMinorScalePattern::apply(A4);
let pitches = a_harmonic_minor.pitches();

// The notes of an A harmonic minor scale are A, B, C, D, E, F, G#
assert_eq!(pitches[0], A4);
assert_eq!(pitches[1], B4);
assert_eq!(pitches[2], C5);
assert_eq!(pitches[3], D5);
assert_eq!(pitches[4], E5);
assert_eq!(pitches[5], F5);
assert_eq!(pitches[6], GSHARP5);
```

#### Creating a Melodic Minor Scale

```rust
use mozzart_scales::heptatonic::melodic_minor::*;
use mozzart_core::pitch::constants::*;

// Create an A melodic minor scale
let a_melodic_minor = MelodicMinorScalePattern::apply(A4);
let ascending = a_melodic_minor.ascending_pitches();
let descending = a_melodic_minor.descending_pitches();

// Ascending form: A, B, C, D, E, F#, G#
assert_eq!(ascending[0], A4);
assert_eq!(ascending[1], B4);
assert_eq!(ascending[2], C5);
assert_eq!(ascending[3], D5);
assert_eq!(ascending[4], E5);
assert_eq!(ascending[5], FSHARP5);
assert_eq!(ascending[6], GSHARP5);

// Descending form: A, B, C, D, E, F, G
assert_eq!(descending[0], A4);
assert_eq!(descending[1], B4);
assert_eq!(descending[2], C5);
assert_eq!(descending[3], D5);
assert_eq!(descending[4], E5);
assert_eq!(descending[5], F5);
assert_eq!(descending[6], G5);
```

## Scale Patterns

### Major Scale
```text
W W H W W W H
```
Where:
- W = Whole step (2 semitones)
- H = Half step (1 semitone)

### Natural Minor Scale
```text
W H W W H W W
```

### Harmonic Minor Scale
```text
W H W W H WH H
```
Where:
- WH = Augmented second (3 semitones)

### Melodic Minor Scale
#### Ascending Form
```text
W H W W W W H
```
#### Descending Form
```text
W W H W W H W
```

## Documentation

For detailed documentation, including:
- Scale structures and patterns
- Interval definitions
- Usage examples
- Musical theory explanations

Run:
```bash
cargo doc --open
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 