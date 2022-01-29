# Control Flow

Takes two forms:

- if/else
    - as expected: provide condition, then execute block of code if condition is met
    - if (condition) {execute} else {execute this};
    - if (condition) {execute} else if (condition) {execute} ...
- match
    - A "scrutinee" expression provided to compare to the patterns
    - let x = 1;
    - Match can return ANY type
    
    `match x {
        1 => ...,
        2 => ...,
        ....
        _ => something else .. <- default value is underscore (_)
    }`