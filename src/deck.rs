use rand::seq::SliceRandom;
use std::vec::Vec;

use crate::card::{Card, Rank, Suit};

pub struct Deck {
    pub cards: Vec<Card>,
    
}

impl Deck{
    pub fn print(&mut self) {
        for card in &self.cards {
            println!("{:?} of {:?}", card.rank, card.suit);
        }
    }
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng(); //https://stackoverflow.com/questions/75485721/better-random-shuffler-in-rust
        let _ = &self.cards.shuffle(&mut rng);
    
        for card in &self.cards {
            println!("{:?} of {:?}", card.rank, card.suit);
        }
    }

    pub fn deal(&mut self) -> Option<Card>{
        return self.cards.pop();
    }
}

const ALL_SUITS:[Suit; 4] = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
const ALL_RANKS:[Rank; 13] = [
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

pub fn full_deck() -> Deck {
    let mut cards = Vec::new();
    for suit in ALL_SUITS.iter() {
        for rank in ALL_RANKS.iter() {
            cards.push(Card {
                rank: rank.clone(),
                suit: suit.clone(),
            });
        }
    }
    
    Deck { cards }
}