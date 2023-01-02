use std::error::Error;

use crate::blackjack::Card;

pub(in crate::blackjack) struct DealerHand {
    cards: Vec<Card>
}

impl DealerHand {
    pub fn reveal_hand(&self) -> Result<Vec<Card>, Box<dyn Error>> {
        if self.cards.len() == 0 {
            return Err("Empty hand.".into());
        }

        Ok(self.cards.clone())
    }
}

pub(in crate::blackjack) struct PlayerHand {
    cards: Vec<Card>
}

trait Hand {
    fn new() -> Self;
    fn show_hand(&self) -> Result<Vec<Card>, Box<dyn Error>>;
    fn add_card(&mut self, card: Card);
    fn score_hand(&self) -> Result<i64, Box<dyn Error>>;
}

impl Hand for DealerHand {
    fn new() -> Self {
        let cards: Vec<Card> = vec![];
        Self {
            cards
        }
    }

    fn show_hand(&self) -> Result<Vec<Card>, Box<dyn Error>> {
        let top_card = self.cards.get(0);
        match top_card {
            Some(t) => Ok(vec![*t]),
            None => Err("Dealer hand contains no cards.".into())
        }
    }

    fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn score_hand(&self) -> Result<i64, Box<dyn Error>> {
        if self.cards.len() <= 0 {
            return Err("Cannot score hand that holds zero cards.".into())
        }
        
        let mut sum = 0;
        for card in &self.cards {
            let score = card.score();
            match score.1 {
                Some(t) => todo!("Implement a way to score with aces"),
                None => sum += score.0,
            }
        }

        Ok(sum)
    }
}

impl Hand for PlayerHand {
    fn new() -> Self {
        let cards: Vec<Card> = vec![];
        Self {
            cards
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

    fn score_hand(&self) -> Result<i64, Box<dyn Error>> {
        if self.cards.len() <= 0 {
            return Err("Cannot score hand that holds zero cards.".into())
        }
        
        let mut sum = 0;
        for card in &self.cards {
            let score = card.score();
            match score.1 {
                Some(t) => todo!("implement a way to score with aces"),
                None => sum += score.0,
            }
        }

        Ok(sum)
    }
}