fn main() {
    let say = String::from("cat");
    print_out(say); // we've just passed the data in "say" to be owned by the function
    println!("Again, {}", say); //this is invalidated because say no longer has any ownership

    //there are ways to pass data to a function without passing ownership - see Borrowing.
}

fn print_out(to_print: String) {
    println!("{}", to_print);
}