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
use mozzart_core::*;
use mozzart_core::constants::*;

// Create a pitch
let c4 = C4;

// Get pitch information
assert_eq!(c4.semitones(), 60);
assert_eq!(c4.octave(), 4);

// Transpose pitches
let e4 = c4.transpose(MAJOR_THIRD);
assert_eq!(e4, E4);
```

#### Building Patterns from a Pitch

```rust
use mozzart_core::*;
use mozzart_core::constants::*;

// Create a major scale pattern using absolute distances from root
let major_pattern = vec![
    PERFECT_UNISON,    // Root
    MAJOR_SECOND,      // 2 semitones from root
    MAJOR_THIRD,       // 4 semitones from root
    PERFECT_FOURTH,    // 5 semitones from root
    PERFECT_FIFTH,     // 7 semitones from root
    MAJOR_SIXTH,       // 9 semitones from root
    MAJOR_SEVENTH,     // 11 semitones from root
];

// Apply the pattern to different roots
let c_major = C4.apply_pattern(major_pattern.clone());
let g_major = G4.apply_pattern(major_pattern);

// Create a pentatonic scale pattern using absolute distances
let pentatonic_pattern = vec![
    PERFECT_UNISON,    // Root
    MAJOR_SECOND,      // 2 semitones from root
    MAJOR_THIRD,       // 4 semitones from root
    PERFECT_FIFTH,     // 7 semitones from root
    MAJOR_SIXTH,       // 9 semitones from root
];

// Apply the pattern to create a pentatonic scale
let c_pentatonic = C4.apply_pattern(pentatonic_pattern);

// Create a blues scale pattern using absolute distances
let blues_pattern = vec![
    PERFECT_UNISON,    // Root
    MINOR_THIRD,       // 3 semitones from root
    PERFECT_FOURTH,    // 5 semitones from root
    DIMINISHED_FIFTH,  // 6 semitones from root
    PERFECT_FIFTH,     // 7 semitones from root
    MINOR_SEVENTH,     // 10 semitones from root
];

// Apply the pattern to create a blues scale
let c_blues = C4.apply_pattern(blues_pattern);

// Create a chord pattern (major triad) using absolute distances
let major_triad_pattern = vec![
    PERFECT_UNISON,    // Root
    MAJOR_THIRD,       // 4 semitones from root
    PERFECT_FIFTH,     // 7 semitones from root
];

// Apply the pattern to create a major triad
let c_major_triad = C4.apply_pattern(major_triad_pattern);

// Create a seventh chord pattern using absolute distances
let seventh_chord_pattern = vec![
    PERFECT_UNISON,    // Root
    MAJOR_THIRD,       // 4 semitones from root
    PERFECT_FIFTH,     // 7 semitones from root
    MINOR_SEVENTH,     // 10 semitones from root
];

// Apply the pattern to create a dominant seventh chord
let c_dominant_seventh = C4.apply_pattern(seventh_chord_pattern);

// Create a modal pattern (Dorian mode) using absolute distances
let dorian_pattern = vec![
    PERFECT_UNISON,    // Root
    MAJOR_SECOND,      // 2 semitones from root
    MINOR_THIRD,       // 3 semitones from root
    PERFECT_FOURTH,    // 5 semitones from root
    PERFECT_FIFTH,     // 7 semitones from root
    MAJOR_SIXTH,       // 9 semitones from root
    MINOR_SEVENTH,     // 10 semitones from root
];

// Apply the pattern to create a Dorian scale
let d_dorian = D4.apply_pattern(dorian_pattern);
```

#### Working with Intervals

```rust
use mozzart_core::*;
use mozzart_core::constants::*;

// Use predefined intervals
assert_eq!(PERFECT_FIFTH.semitones(), 7);
assert_eq!(MAJOR_THIRD.semitones(), 4);

// Create custom intervals
let custom_interval = Interval::new(3); // Minor third
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