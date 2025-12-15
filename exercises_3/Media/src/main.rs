#[derive(Debug)]
enum Media {
    Book { title: String , author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
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
}
