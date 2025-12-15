#[derive(Debug)]
enum Media {
    Book { title: String , author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

impl Media {
    fn desription(&self) -> String {
        match self {
            Media::Book { title, author } => format!("Book: {} {}", title, author),
            Media::Movie { title, director } => format!("Movie: {} {}", title, director),
            Media::Audiobook { title } => format!("Audiobook: {}", title),
        }
    }
}

fn print_media(media: &Media) {
    println!("{:?}", media);
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("Media book"),
    };
    print_media(&audiobook);

    let book = Media::Book {
        title: String::from("Book"),
        author: String::from("Book author"),
    };
    print_media(&book);

    let movie = Media::Movie {
        title: String::from("movie"),
        director: String::from("Movie director"),
    };
    print_media(&movie);

    println!("{}", audiobook.desription());
    println!("{}", book.desription());
    println!("{}", movie.desription());

}
