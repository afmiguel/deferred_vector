
# Deferred Vector Library

## Introduction
The Deferred Vector Library offers `DeferredVec`, a generic, lazily-initialized vector structure in Rust, ideal for efficient resource management and performance optimization. Developed by Prof. Afonso Miguel at PUCPR.

## Features
- **Laziness**: Only initializes the vector when explicitly accessed.
- **Flexibility**: Compatible with any type `T` that implements the `Clone` trait.
- **Custom Initialization**: Uses a user-provided function for vector initialization.

## Usage
Instantiate `DeferredVec` with a `fetch_function` defining the initial state. The vector remains uninitialized (`None`) until methods like `get` or `len` are invoked, triggering initialization.

## Examples
Basic usage:
```rust
let mut deferred_vector = DeferredVec::new(|| vec![1, 2, 3]);
assert_eq!(deferred_vector.len(), 3);
```
Check if the vector is deferred:
```rust
assert_eq!(deferred_vector.is_deferred(), true);
let initialized_vector = deferred_vector.get();
assert_eq!(deferred_vector.is_deferred(), false);
```

## Testing
Includes unit tests focusing on lazy initialization and basic vector operations.

## Disclaimer
Provided as-is, without warranty. Test thoroughly before production use.

## Author
Prof. Afonso Miguel - PUCPR.

## License
MIT License - see the LICENSE file for details.
