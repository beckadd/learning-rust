fn main() {
    let mut my_vec = vec![1, 2, 3];
    println!("{:?}", my_vec);

    add_to_vec(&mut my_vec); //we're borrowing here, but notice I had to pass the mut keyword to make sure the borrowed reference was also mutable.
    println!("{:?}", my_vec);

    let say = String::from("Cat");
    let say2 = &say; //borrowing!

    println!("{}", say);

    drop(say); //say drops the heap and stack, invalidating say2 as well - lifetime of say2 is inherently tied to say and dies with say;

    println!("{}", say2) // causes a failure to compile!
}

fn add_to_vec(a_vec: &mut Vec<i32>){
    a_vec.push(4);
    //notice, returns () because this is an in-place operation, ick
}

