use content::media::Media;
use content::catalog::Catalog;

mod content;


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

    match catalog.get_by_index(3) {
        Some(media_item) => print_media(&media_item),
        None => println!("No items found in catalog!"),
    }

}
