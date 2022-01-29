fn main() {
    let text = "Hello\nworld!\n";
    let upper = text.to_uppercase();
    let stripped = upper.strip_prefix("HELLO\n").unwrap();
    println!("{}", first_line(text));

}

pub fn first_line(string: &str) -> &str {
    //attempting to return this causes an error, because we are getting a &str from this operation.
    //string.lines().next().unwrap(); 

    //turning this into an owned string makes it a String.

    string.lines().next().unwrap()
}