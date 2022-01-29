# Rust Compound Data Types

- Arrays
    - Defined by `let array = [1, 2, 3]`
    - fixed length
    - zero-indexed
    - Rust will warn if we try to index out of bounds, and panic at runtime

- Tuples
    - continuous groups of items
    - fixed length
    - length known at compile time
    - Homogeneous - you can use different types of items
    - `let tuple = (true, 2, 3)`
    - The empty tuple has special name "unit"
    - instead of bracket indexing, we used numbered dot notation: tuple.0, tuple.1, etc.
    - Rust will always give compile error if we attempt to index tuple out of bounds
