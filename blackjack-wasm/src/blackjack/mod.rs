use wasm_bindgen::prelude::*;

mod card;
mod shoe;
mod player;

use crate::blackjack::player::Player;
use crate::blackjack::shoe::Shoe;
use crate::blackjack::card::Card;

const NUM_DECKS: u8 = 7;

#[wasm_bindgen]
pub struct Blackjack {
    player: Player,
    shoe: Shoe,
    dealer_cards: Vec<Card>,
    player_cards: Vec<Card>,
}

impl Blackjack {
    pub fn init() -> Self {
        let player = Player::new();
        let shoe = Shoe::new(NUM_DECKS);
        let dealer_cards: Vec<Card> = vec![];
        let player_cards: Vec<Card> = vec![];

        Self {
            player,
            shoe,
            dealer_cards,
            player_cards
        }
    }
}