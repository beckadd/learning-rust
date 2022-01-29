struct Person {
    name: String,
    age: u8,
    likes_oranges: bool
}

//tuple struct - we don't care about the name of the first, second values, just the order
struct Point2D(u32, u32);
fn main() {
    let person = Person {
        name: "Adam".to_string(), //way easier to do it this way
        likes_oranges: true, //notice out-of-order assignment - doesn't matter to Rust
        age: 25
    };

    //access values with dot notation

    let point = Point2D(100, 200);

    println!("Point's x, y values are ({:?}, {:?})", point.0, point.1);
    println!("Person's name is {}", person.name);

    let Point2D(x, y) = point; //purrr destructuring assignment

    println!("Point contains {:?}, {:?}", x, y);
}
