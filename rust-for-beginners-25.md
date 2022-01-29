# Ownership in Rust

Concepts of memory management and how Rust guarantees memory safety without garbage collector:

Ownership relates to how program manages memory

Memory is managed in one of two places:
- Stack
- Heap

Stack is a LIFO queue: 
- where rust stores data with a known, fixed size
- for example, in `let a:i32 = 5`, rust knows the value of a at compile time, as well as the size; it's stored on the stack.

Heaps
- where rust stores data with mutable size (a vector, for example, has a pointer in the stack pointing to the heap, where its values are stored. When the vector is dropped from memory, rust eliminates the pointer and the data pointed to).
- Pointers to data are stored on the stack, but the data pointed to is stored on the heap.
- If we create another pointer and assign it to the sample location on the heap, when we drop either pointer, the heap data is deleted. This causes a dangling pointer - what happens here in Rust?
    - In C/C++, programmer needs to be able detect these issues. This is difficult in practice and becomes more unwieldy as codebase grows.
    - In interpreted languages, garbage collector is used to clean up bad references.
    - Rust implements Ownership - Rust compiler verifies a set of rules at compile-time.


Rust Ownership Rules:
- Each value in rust has a variable called its owner.
- There can only be a single owner of a value at a time. If we assign a second variable to the location pointed to by the first, we reassign value to the new owner. This invalidates the first variable. This rule invalidates possibility of dangling pointers, double-free memory, memory leakage.
- When the owner goes out of scope, the value is dropped. A sort of primitive garbage collection rule(?)

