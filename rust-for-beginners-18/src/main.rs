enum WebEvent {
    PageLoad,
    PageUnload, //unit like variants
    KeyPress(char),
    Paste(String),
    Click {x:i64, y:i64},
}


//option enum: part of standard library
enum Option<T> { //Option is not required, but this is helpful
    Some(T), // T represents a generic type - holds any type of data
    None,
}

fn main() {
    let quit = WebEvent::KeyPress('q');

    let something = Some(1);
}
