use crate::blackjack::card::Card;
use crate::constants::{Suit, Suit::*};
use crate::constants::Value::*;

pub(in crate::blackjack) struct Shoe {
    cards: Vec<Card>
}

impl Shoe {
    pub fn new(num_decks: u8) -> Self {
        let mut cards: Vec<Card> = vec![];

        let mut generate_suit = |suit: Suit| {
            cards.push(Card::new(suit, One));
            cards.push(Card::new(suit, Two));
            cards.push(Card::new(suit, Three));
            cards.push(Card::new(suit, Four));
            cards.push(Card::new(suit, Five));
            cards.push(Card::new(suit, Six));
            cards.push(Card::new(suit, Seven));
            cards.push(Card::new(suit, Eight));
            cards.push(Card::new(suit, Nine));
            cards.push(Card::new(suit, Ten));
            cards.push(Card::new(suit, Jack));
            cards.push(Card::new(suit, King));
            cards.push(Card::new(suit, Queen));
            cards.push(Card::new(suit, Ace));
        };

        let mut generate_deck = || {
            generate_suit(Hearts);
            generate_suit(Spades);
            generate_suit(Diamonds);
            generate_suit(Clubs);
        };

        for _ in 0..num_decks {
            generate_deck();
        };

        Self {
            cards
        }
    }
}