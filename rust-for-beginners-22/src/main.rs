fn main() {
    let mut i = 1;

    let something = loop {
        i *= 2;

        if i > 100 {
            break i; //break, and return i to something
        }
    };

    assert_eq!(something, 128);

    let mut counter = 0;

    while counter < 10 {
        println!("hello");
        counter += 1;
    }

    for item in 0..5 { //exclusive ending there - only goes through 0, 1, 2, 3, 4
        println!("{}", item*2)
    }
}
