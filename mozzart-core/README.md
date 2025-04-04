# Mozzart Core

A Rust library for music theory and composition, providing core functionality for working with musical elements.

## Features

- **Pitch System**: Represent and manipulate musical pitches
  - MIDI note number support
  - Pitch class and octave handling
  - Transposition and interval calculations
  - Canonical form representation

- **Interval System**: Work with musical intervals
  - Common interval constants (unison, third, fifth, etc.)
  - Interval quality and size
  - Interval arithmetic and transposition

- **Octave System**: Handle octave-related operations
  - Octave numbering (0-9)
  - Octave range iteration
  - Canonical octave determination

- **Scale System**: Define and work with musical scales
  - Scale type definitions
  - Scale pattern implementations
  - Scale construction and manipulation

- **Chord System**: Represent and manipulate musical chords
  - Chord type definitions
  - Chord pattern implementations
  - Common chord constants

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
mozzart-core = "0.1.0"
```

### Basic Examples

#### Working with Pitches

```rust
use mozzart_core::pitch::constants::*;
use mozzart_core::interval::constants::*;

// Create a pitch
let c4 = C4;

// Get pitch information
assert_eq!(c4.semitones(), 60);
assert_eq!(c4.octave(), 4);

// Transpose pitches
let e4 = c4.transpose(MAJOR_THIRD);
assert_eq!(e4, E4);
```

#### Working with Intervals

```rust
use mozzart_core::interval::constants::*;

// Use predefined intervals
assert_eq!(PERFECT_FIFTH.semitones(), 7);
assert_eq!(MAJOR_THIRD.semitones(), 4);

// Create custom intervals
let custom_interval = Interval(3); // Minor third
```

#### Working with Scales

```rust
use mozzart_core::scale::*;
use mozzart_core::pitch::constants::*;

// Create a C major scale
let c_major = MajorScale::apply(C4);
assert_eq!(c_major[0], C4);
assert_eq!(c_major[1], D4);
assert_eq!(c_major[2], E4);
```

#### Working with Chords

```rust
use mozzart_core::chord::*;
use mozzart_core::pitch::constants::*;
use mozzart_core::interval::constants::*;

// Create a C major triad
let root = C4;
let major_third = root.transpose(MAJOR_THIRD);
let perfect_fifth = root.transpose(PERFECT_FIFTH);

// The notes of a C major triad are C, E, G
assert_eq!(major_third, E4);
assert_eq!(perfect_fifth, G4);
```

## Performance

The library is designed for high performance with:
- Zero-cost abstractions
- Minimal memory allocation
- Efficient data structures
- Optimized algorithms

Benchmarks are available in the `benches` directory.

## Documentation

Comprehensive documentation is available:
- [API Documentation](https://docs.rs/mozzart-core)
- [Examples](examples/)
- [Benchmarks](benches/)

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 