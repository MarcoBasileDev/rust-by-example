#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    let suits = ["Hearts", "Spades", "Diamonds"];
    let values = ["Ace", "Two", "Three"];

    let mut deck = Deck { cards: Vec::new() };

    for suit in &suits {
        for value in &values {
            deck.cards.push(format!("{} of {}",value, suit));
        }
    }


    println!("Here's your deck: {:?}", deck);
}
