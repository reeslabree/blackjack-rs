use std::error::Error;
use wasm_bindgen::prelude::*;

mod card;
mod player;
mod shoe;
mod hand;

use crate::blackjack::card::Card;
use crate::blackjack::player::Player;
use crate::blackjack::shoe::Shoe;

const NUM_DECKS: u8 = 7;

#[wasm_bindgen]
pub struct Blackjack {
    pub player: Player,
    shoe: Shoe,
    dealer_cards: Vec<Card>,
    player_cards: Vec<Card>,
    player_bet: i64,
    active_hand: bool,
}

impl Blackjack {
    pub fn init() -> Self {
        let player = Player::new();
        let shoe = Shoe::new(NUM_DECKS);
        let dealer_cards: Vec<Card> = vec![];
        let player_cards: Vec<Card> = vec![];
        let active_hand = false;
        let player_bet = 0;

        Self {
            player,
            shoe,
            dealer_cards,
            player_cards,
            player_bet,
            active_hand,
        }
    }

    pub fn start_hand(&mut self, bet: i64) -> Result<(), Box<dyn Error>> {
        if bet <= 0 {
            return Err("Bet must be greater than 0.".into());
        }
        if self.active_hand {
            return Err("Cannot start hand while hand is currently active.".into());
        }

        // lock in the player bet
        self.player_bet = bet;

        Ok(())
    }
}
