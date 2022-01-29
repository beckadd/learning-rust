fn main() {
    println!("Hello, world!");

    if 1 == 2 {
        // unlike most languages, if blocks can act as expressions
        println!("Math is broken");
    } else {
        println!("everything is good")
    }

    let formal = true;
    
    let greeting = if formal {
        println!("Good evening")
    } else {
        println!("Hey friend")
    } //hypothesis: assigns greeting to the () unit as a null return type
    //CORRECT!

    let boolean = true;
    
    let binary = match boolean {
        false => 0,
        true => 1
    };

}
