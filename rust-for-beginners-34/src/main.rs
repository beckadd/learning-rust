struct Film {
    title: String,
    director: String,
    studio: String,
}

struct Book {
    title: String,
    author: String,
    publisher: String
}

trait Catalog {
    fn describe(&self) {
        // default behavior
        println!("We need more information about this type of media"); //default case
    } // AB"C" for these items.
}

impl Catalog for Film {
    fn describe(&self) {
        println!(
            "{} was directed by {} through {} studios",
            self.title,
            self.director,
            self.studio
        )
    }
}

impl Catalog for Book {
    fn describe(&self) {
        println!(
            "{} was written by {} and published by {}",
            self.title,
            self.author,
            self.publisher
        )
    }
}

struct Album {
    title: String,
    artist: String,
    label: String
}

impl Catalog for Album {
    //empty - this will imply that we use the default methods for this implementation
}

fn main() {
    let cpt_marvel = Film {
        title: String::from("Captain Marvel"),
        director: String::from("Anna Boden and Ryan Fleck"),
        studio: String::from("Marvel")
    };

    cpt_marvel.describe();

    let elantris = Book {
        title: String::from("Elantris"),
        author: String::from("Brandon Sanderson"),
        publisher: String::from("Tor Books")
    };

    elantris.describe();

    let let_it_be = Album {
        title: String::from("Let it Be"),
        artist: String::from("The Beatles"),
        label: String::from("Apple")
    };
    
    let_it_be.describe();
}

