# Mozzart Ply

A Rust library for pattern-based musical composition.

## Features

- Pattern-based composition system
- Musical pattern definitions and transformations
- Pattern application and manipulation
- Pattern analysis and comparison

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
mozzart-ply = "0.1.0"
```

## Examples

```rust
use mozzart_ply::*;

// Create a pattern
let pattern = Pattern::new(vec![1, 2, 3, 4]);

// Apply the pattern
let result = pattern.apply(0);
assert_eq!(result, vec![1, 2, 3, 4]);
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 