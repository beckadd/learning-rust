# Error handling in Rust

- Panic
    - simplest way to handle errors
    - stops executing, and failure message is printed (whatever is in `panic!()`)
    - should only be used when program comes to an unrecoverable state.
    - Panic will happen when, for example, an index is out of bounds.
- Option Enum:
    - Handles possibility of null values using None
    - Often returns as a way to handle a recoverable error.
- Result Enum:
    - Used for recoverable errors that are more common
    - Enum Result <T, E> { Ok(T), Err(E) } 
    - Ok(T) element represents success, while Err(E) represents error.
    - used for recoverable I/O operations that may be a bit unreliable.
- Question mark (?) operator:
    - Similar to a match statement,
    - For Result Type, unwraps the value if Ok, or error if Err
    - For Option type, returns value with Some, or nothing for None. Was there a typo on her slide here? I didn't understand the "returns value *with* Some"

