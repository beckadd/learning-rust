# Data types in Rust

- Integers:
    - Integers are whole numbers and are either signed or unsigned.
    - Length can be i8, i16, i32, i64, i128
    - Rust defaults to i32; it's generally the fastest
        - Why? Reddit explains that it possibly has something to do with alignment.
        - Other sources indicate that minimizing L2 cache hits is desireable, so reducing the byte size as much as possible is desireable.
    - data types can be inferred

- Floating-point:
    - Numbers with decimal points.
    - f32/f64
    - Rust defaults to f64 - modern CPUs have equal performance on f32/f64, so we get extra precision for our time

- Booleans
    - true/false
    - Used in conditional/control flow (if) statements
    - One byte in size

- Characters
    - represent letters
    - specified by the char keyword
    - four bytes in size
    - UNICODE

