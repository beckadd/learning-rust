# Strings in Rust

Always UTF-8 and not null-terminated.

Difference between String and &str:

String:
- An "owned" string - variable owns string, and string data is dropped with drop to reference.

&str:
- A borrowed string "slice"
- doesn't own it's string data, and data isn't freed when the value is dropped
- represents a "view" into the original string.
- contains two parts - the length of the string, and a data pointer.

string literals are embedded directly into the binary, and therefore cannot be owned since they aren't on the heap
Instead, they are viewed with "&str"

Unlike other languages, Rust strings are not arrays of characters, and cannot be indexed as typical.