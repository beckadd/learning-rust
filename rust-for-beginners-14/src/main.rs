fn main() {
    last_char(String::from("Hello World"));
}

fn last_char(string: String) -> char {
    if string.is_empty() {return 'e'};
    string.chars().next_back().unwrap()    
}
