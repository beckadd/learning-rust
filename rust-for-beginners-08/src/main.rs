fn main() {
    let mut x = 1;
    println!("The value of x is: {}", x);
    x = 2;
    println!("The value of x is {}", x);
    let y = true;
    println!("The value of y is: {}", y);

    let y = false; // this is shadowing
    println!("the value of y is: {}", y);

    const STRING: &str = "hello";
    println!("the value of STRING is {}", STRING);
}
