use std::error::Error;

use crate::blackjack::Card;
use crate::constants::Value;

#[derive(Clone)]
pub(in crate::blackjack) struct DealerHand {
    cards: Vec<Card>,
}

impl DealerHand {
    pub fn reveal_hand(&self) -> Result<Vec<Card>, Box<dyn Error>> {
        if self.cards.len() == 0 {
            return Err("Empty hand.".into());
        }

        Ok(self.cards.clone())
    }
}

#[derive(Clone)]
pub(in crate::blackjack) struct PlayerHand {
    cards: Vec<Card>,
}

pub(in crate::blackjack) trait Hand {
    fn new() -> Self;
    fn show_hand(&self) -> Result<Vec<Card>, Box<dyn Error>>;
    fn add_card(&mut self, card: Card);
    fn score_hand(&self) -> Result<Vec<i64>, Box<dyn Error>>;
    fn is_blackjack(&self) -> bool;
}

impl Hand for DealerHand {
    fn new() -> Self {
        let cards: Vec<Card> = vec![];
        Self { cards }
    }

    fn is_blackjack(&self) -> bool {
        match self.cards.len() {
            2 => {
                if self.cards[0].value == Value::Jack && self.cards[1].value == Value::Ace
                    || self.cards[0].value == Value::Ace && self.cards[1].value == Value::Ace
                {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn show_hand(&self) -> Result<Vec<Card>, Box<dyn Error>> {
        let top_card = self.cards.get(0);
        match top_card {
            Some(t) => Ok(vec![*t]),
            None => Err("Dealer hand contains no cards.".into()),
        }
    }

    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn score_hand(&self) -> Result<Vec<i64>, Box<dyn Error>> {
        if self.cards.len() <= 0 {
            return Err("Cannot score hand that holds zero cards.".into());
        }

        let mut scores = vec![0];
        for card in &self.cards {
            let score = card.score();
            let num_scores = scores.len();

            match score.1 {
                Some(t) => {
                    let scores_clone = scores.clone();
                    scores.extend(scores_clone);
                    for i in 0..(2 * num_scores) {
                        match i % 2 {
                            0 => scores[i] += score.0,
                            _ => scores[i] += t,
                        }
                    }
                }
                None => {
                    for i in 0..num_scores {
                        scores[i] += score.0
                    }
                }
            }
        }

        Ok(scores)
    }
}

impl Hand for PlayerHand {
    fn new() -> Self {
        let cards: Vec<Card> = vec![];
        Self { cards }
    }

    fn is_blackjack(&self) -> bool {
        match self.cards.len() {
            2 => {
                if self.cards[0].value == Value::Jack && self.cards[1].value == Value::Ace
                    || self.cards[0].value == Value::Ace && self.cards[1].value == Value::Ace
                {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn show_hand(&self) -> Result<Vec<Card>, Box<dyn Error>> {
        if self.cards.len() == 0 {
            return Err("Empty hand.".into());
        }

        Ok(self.cards.clone())
    }

    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn score_hand(&self) -> Result<Vec<i64>, Box<dyn Error>> {
        if self.cards.len() <= 0 {
            return Err("Cannot score hand that holds zero cards.".into());
        }

        let mut scores = vec![0];
        for card in &self.cards {
            let score = card.score();
            let num_scores = scores.len();

            match score.1 {
                Some(t) => {
                    let scores_clone = scores.clone();
                    scores.extend(scores_clone);
                    for i in 0..(2 * num_scores) {
                        match i % 2 {
                            0 => scores[i] += score.0,
                            _ => scores[i] += t,
                        }
                    }
                }
                None => {
                    for i in 0..num_scores {
                        scores[i] += score.0
                    }
                }
            }
        }

        Ok(scores)
    }
}
