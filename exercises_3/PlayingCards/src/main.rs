#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];
        for suit in &suits {
            for value in &values {
                cards.push(format!("{} of {}",value, suit));
            }
        }
        let deck = Deck { cards };
        deck
    }
}

fn main() {
    let deck = Deck::new();

    println!("Here's your deck: {:#?}", deck);
}
