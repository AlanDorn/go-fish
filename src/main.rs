
use rand::seq::SliceRandom;
use std::vec::Vec;

#[derive(Clone, Debug)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Debug)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

struct Card {
    rank: Rank,
    suit: Suit,
}

struct Deck {
    cards: Vec<Card>,
}
fn main() {
    let mut deck = new_deck();

    //Ordered Deck output
    // println!("Ordered Deck \n");
    // print_deck(&deck);

    //shuffled deck output
    println!("Shuffled Deck: \n");
    shuffle_deck(&mut deck);
}

// fn print_deck(deck: &Deck) {
//     for card in &deck.cards {
//         println!("{:?} of {:?}", card.rank, card.suit);
//     }
// }

//shuffle the fookin deck
fn shuffle_deck(deck: &mut Deck) {
    let mut rng = rand::thread_rng(); //https://stackoverflow.com/questions/75485721/better-random-shuffler-in-rust
    let _ = &deck.cards.shuffle(&mut rng);


    for card in &deck.cards {
        println!("{:?} of {:?}", card.rank, card.suit);
    }
}

fn new_deck() -> Deck {
    let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
    let ranks = [
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];

    let mut cards = Vec::new();
    for suit in suits.iter() {
        for rank in ranks.iter() {
            cards.push(Card {
                rank: rank.clone(),
                suit: suit.clone(),
            });
        }
    }

    Deck { cards }
}

