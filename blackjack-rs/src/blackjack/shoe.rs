use std::error::Error;

use crate::blackjack::card::Card;
use crate::constants::Value::*;
use crate::constants::{Suit, Suit::*};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub(in crate::blackjack) struct Shoe {
    cards: Vec<Card>,
}

impl Shoe {
    pub fn new(num_decks: u8) -> Self {
        let mut cards: Vec<Card> = vec![];

        let mut generate_suit = |suit: Suit| {
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
        }

        cards.shuffle(&mut thread_rng());

        Self { cards }
    }

    pub fn draw_card(&mut self) -> Result<Card, Box<dyn Error>>{
        let card = self.cards.pop();

        match card {
            Some(t) => Ok(t),
            None => Err("Empty deck.".into())
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::hash::Hash;

    #[test]
    fn can_create_new_shoe() {
        let shoe = Shoe::new(1);
        assert_eq!(shoe.cards.len(), 52);

        assert!(has_unique_elements(shoe.cards));

        let shoe = Shoe::new(2);
        assert_eq!(shoe.cards.len(), 52 * 2);
    }

    #[test]
    fn cards_are_shuffled() {
        let shoe_1 = Shoe::new(1);
        let mut shoe_2 = Shoe::new(1);

        let mut cards_1 = shoe_1.cards;
        cards_1.reverse();

        for card in cards_1 {
            let comp_card = shoe_2.cards.pop().unwrap();

            if card != comp_card {
                assert!(true);
                return;
            }
        }
        assert!(false);
    }

    fn has_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
    {
        let mut uniq = HashSet::new();
        iter.into_iter().all(move |x| uniq.insert(x))
    }
}
