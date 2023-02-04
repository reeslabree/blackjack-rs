use crate::constants::{Suit, Value};

#[derive(Eq, Hash, Debug, Clone, Copy)]
pub(in crate::blackjack) struct Card {
    suit: Suit,
    pub(in crate::blackjack) value: Value
}

impl Card {
    pub fn new(suit: Suit, value: Value) -> Self {
        Self {
            suit,
            value
        }
    }

    pub fn score(&self) -> (i64, Option<i64>) {
        let value = match self.value {
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ace => return (11, Some(1)),
            _ => 10,
        };
        (value, None)
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