use std::fs::File; //import File function

fn main() {
    // panic!("Farewell");
    //let v = vec![0, 1, 2, 3];
    //println!("{}", v[6]) //<- will fail with another panic

    let fruits = vec!["banana", "apple", "coconut"];
    let first = fruits.get(0);
    println!("{:?}", first); //prints something()

    let third = fruits.get(2).unwrap();
    println!("{:?}", third); //prints actual val

    let nonexistent = fruits.get(100);

    println!("{:?}", nonexistent); //prints None from Option::None

    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // even less code for similar, cleaner, and more semantic output
    //let f = File::open("hello.txt").unwrap(); // cleaner than below for same output with a little less narrative
    //let f = match f {
    //    Ok(file) => file,
    //    Err(error) => panic!("can't open the file, panicked with error:\n{:?}", error)
    //};
}
