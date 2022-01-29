fn main() {
    let mut students = vec![Student {
        name: "Ryan".to_string()
    }];

    students.push(Student {
        name: "Lisa".to_string(),
    });

    assert!(&students[0] == &Student {name: "Ryan".to_string()});

    assert!(students.get(0) == Some(&Student {name: "Ryan".to_string()})); //recall that .get() soft-fails by checking Some() vs None
    assert!(students.get(1000) == None); //won't fail but will gracefully resolve error

    for student in students.iter() {
        println!("Student name: {}", student.name);
    }

    use std::collections::HashMap;

    let mut enrollment = HashMap::new();

    enrollment.insert("biology".to_string(), students);
    
    let bio_students = enrollment.get("biology"); //recall, returns "Some()";
    let students = enrollment.remove("biology"); //just like removing key from a dict
} 

#[derive(PartialEq, Eq)] //this gives us ability to test that structs are equal if fields are equal.
struct Student {
    name: String,
}