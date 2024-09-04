
mod card;
mod deck;
use deck::full_deck;

fn main() {
    let mut deck = full_deck();
    deck.shuffle();
    let card = deck.deal();
    // println!("Shuffled Deck: \n");
       
    println!("\n");
    match card {
        Some(card) => println!("{:?} of {:?}", card.rank, card.suit),
        None => (),
    }
    println!("\n");
    deck.print();
    // println!("{:?} of {:?}", card.rank, card.suit); 
}