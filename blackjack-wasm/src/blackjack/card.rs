use std::fmt;

use crate::constants::{Suit, Value};

#[derive(Eq, Hash, Debug)]
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

impl PartialEq for Card{
    fn eq(&self, other: &Self) -> bool {
        if self.suit == other.suit && self.value == other.value {
            return true;
        } else {
            return false;
        }
    }
}