# Borrowing in Rust

Borrowing is similar to ownership, but transfers ownership **back** to the original pointer once the second pointer is done with it.

Recall that passing a pointer's data to another pointer invalidates the original variable. This isn't always desireable, especially when we are performing a function call and just want some output but also want to keep a persistent pointer on our data.

if we initialize `let mut variable = 3`, then `let var2 = variable.clone()` creates another pointer pointing to another location on the heap that has a COPY of the data. If we drop our first variable, var2 is still valid, as they own two different objects.

Cloning isn't always preferred - if an object is large, then we don't want to copy it several times!

Borrowing is the solution - it creates a temporary reference to the variable's data, rather than passing on the ownership of this data.

Rules for borrowing:
- At any given time, you can have either one mutable reference, or any number of immutable references.
- References **must** be valid.