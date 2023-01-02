use std::error::Error;
use wasm_bindgen::prelude::*;

mod card;
mod player;
mod shoe;
mod hand;

use self::card::Card;
use self::player::Player;
use self::shoe::Shoe;
use self::hand::{DealerHand, PlayerHand};
use self::hand::Hand;

use crate::constants::GameState;


const NUM_DECKS: u8 = 7;

#[wasm_bindgen]
pub struct Blackjack {
    pub player: Player,
    shoe: Shoe,
    dealer_cards: DealerHand,
    player_cards: PlayerHand,
    player_bet: i64,
    game_state: u8
}


impl Blackjack {
    pub fn init() -> Self {
        let player = Player::new();
        let shoe = Shoe::new(NUM_DECKS);
        let dealer_cards = DealerHand::new();
        let player_cards = PlayerHand::new();
        let game_state = GameState::HAND_STARTED;
        let player_bet = 0;

        Self {
            player,
            shoe,
            dealer_cards,
            player_cards,
            player_bet,
            game_state,
        }
    }

    pub fn start_hand(&mut self, bet: i64) -> Result<(), Box<dyn Error>> {
        if bet <= 0 {
            return Err("Bet must be greater than 0.".into());
        }
        if self.game_state != GameState::HAND_STARTED {
            return Err("Cannot start hand while hand is currently active.".into());
        }

        self.player_bet = bet;

        // deal cards to player and dealer, alternating between player and dealer
        for i in 0..4 {
            let card = self.shoe.draw_card()?;
            match i % 2 {
                0 => self.player_cards.add_card(card),
                _ => self.dealer_cards.add_card(card),
            }
        }

        self.game_state = GameState::PLAYER_TURN;

        Ok(())
    }

    pub fn player_hit(&mut self) -> Result<Option<i64>, Box<dyn Error>> {
        if self.game_state != GameState::PLAYER_TURN {
            return Err("Cannot hit when it is not the player turn.".into());
        }

        let card = self.shoe.draw_card()?;

        self.player_cards.add_card(card);

        let score = self.player_cards.clone().score_hand()?;
        let best_score = get_best_score(score)?;

        match best_score {
            Some(_) => (),
            None => self.game_state = GameState::DEALER_TURN,
        }

        Ok(best_score)
    }

    pub fn end_player_turn(&mut self) -> Result<Option<i64>, Box<dyn Error>> {
        if self.game_state != GameState::PLAYER_TURN {
            return Err("Cannot end players turn outside of the player's turn.".into());
        }

        self.game_state = GameState::DEALER_TURN;

        let score = self.player_cards.clone().score_hand()?;

        let best_score = get_best_score(score)?;

        return Ok(best_score);
    }

    pub fn do_dealer_turn(&mut self) -> Result<Option<i64>, Box<dyn Error>> {


        Ok(None)
    }
}

fn get_best_score(mut score: Vec<i64>) -> Result<Option<i64>, Box<dyn Error>> {
    score.sort_by(|a, b,| { b.cmp(a) });

    for s in score {
        if s > 21 { continue };
        return Ok(Some(s));
    }

    Ok(None)
}

fn should_dealer_hit(score: Vec<i64>) -> Result<bool, Box<dyn Error>> {
    let min_score = score.iter().min();

    match min_score {
        Some(t) => {
            if *t < 17 { return Ok(true) }
            else { return Ok(false) }
        },
        None => return Err("Could not derive minimum score for dealer.".into()),
    }
}

fn is_blackjack<T>(hand: T) -> Result<bool, Box<dyn Error>> 
where
    T: Hand
{

    Ok(true)
}