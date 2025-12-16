#[derive(Debug)]
enum Media {
    Book { title: String , author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn desription(&self) -> String {
        match self {
            Media::Book { title, author } => format!("Book: {} {}", title, author),
            Media::Movie { title, director } => format!("Movie: {} {}", title, director),
            Media::Audiobook { title } => format!("Audiobook: {}", title),
            Media::Podcast(number) => format!("Podcast: {}", number),
            Media::Placeholder => "Placeholder".to_string(),
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: Vec::new() }
    }

    fn add(&mut self, item: Media) {
        self.items.push(item);
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() < index {
            None
        } else {
            Some(&self.items[index])
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

    let podcast = Media::Podcast(10);
    print_media(&podcast);

    let placeholder = Media::Placeholder;
    print_media(&placeholder);

    println!("{}", audiobook.desription());
    println!("{}", book.desription());
    println!("{}", movie.desription());
    println!("{}", podcast.desription());
    println!("{}", placeholder.desription());

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(book);
    catalog.add(movie);
    catalog.add(podcast);
    catalog.add(placeholder);

    println!("{:#?}", catalog);

    if let Some(value) = catalog.get_by_index(3) {
        println!("{:#?}", value)
    } else {
        println!("Nothing found")
    }

}
