use crate::constants::{Suit, Value};

pub(in crate::blackjack) struct Card {
    suit: Suit,
    value: Value
}

impl Card {
    pub fn new(suit: Suit, value: Value) -> Self {
        Self {
            suit,
            value
        }
    }
}