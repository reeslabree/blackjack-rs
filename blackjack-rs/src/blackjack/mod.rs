use std::error::Error;
use wasm_bindgen::prelude::*;

mod card;
mod hand;
mod player;
mod shoe;

use self::card::Card;
use self::hand::Hand;
use self::hand::{DealerHand, PlayerHand};
use self::player::Player;
use self::shoe::Shoe;

use crate::constants::GameState;

const NUM_DECKS: u8 = 7;

#[wasm_bindgen]
pub struct Blackjack {
    pub player: Player,
    shoe: Shoe,
    dealer_cards: DealerHand,
    player_cards: PlayerHand,
    player_bet: i64,
    game_state: u8,
}

#[wasm_bindgen]
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

    pub fn start_hand(&mut self, bet: i64) -> Result<(), String> {
        if bet <= 0 {
            return Err("Bet must be greater than 0.".into());
        }
        if self.game_state != GameState::HAND_STARTED {
            return Err("Cannot start hand while hand is currently active.".into());
        }

        self.player
            .debit(bet)
            .expect(ErrorCode::FailedToDebit.inspect());
        self.player_bet = bet;

        // deal cards to player and dealer, alternating between player and dealer
        for i in 0..4 {
            let card = self
                .shoe
                .draw_card()
                .expect(ErrorCode::FailedToDraw.inspect());
            match i % 2 {
                0 => self.player_cards.add_card(card),
                _ => self.dealer_cards.add_card(card),
            }
        }

        self.game_state = GameState::PLAYER_TURN;

        Ok(())
    }

    pub fn player_hit(&mut self) -> Result<Option<i64>, String> {
        if self.game_state != GameState::PLAYER_TURN {
            return Err("Cannot hit when it is not the player turn.".into());
        }

        let card = self
            .shoe
            .draw_card()
            .expect(ErrorCode::FailedToDraw.inspect());

        self.player_cards.add_card(card);

        let score = self
            .player_cards
            .clone()
            .score_hand()
            .expect(ErrorCode::FailedToScoreHand.inspect());
        let best_score = get_best_score(score)?;

        match best_score {
            Some(_) => (),
            None => self.game_state = GameState::DEALER_TURN,
        }

        Ok(best_score)
    }

    pub fn end_player_turn(&mut self) -> Result<Option<i64>, String> {
        if self.game_state != GameState::PLAYER_TURN {
            return Err("Cannot end players turn outside of the player's turn.".into());
        }

        self.game_state = GameState::DEALER_TURN;

        let score = self
            .player_cards
            .clone()
            .score_hand()
            .expect(ErrorCode::FailedToScoreHand.inspect());

        let best_score = get_best_score(score)?;

        return Ok(best_score);
    }

    pub fn do_dealer_turn(&mut self) -> Result<Option<i64>, String> {
        if self.game_state != GameState::DEALER_TURN {
            return Err("Cannot begin dealer turn outside of dealer's turn.".into());
        }

        loop {
            let score = self
                .dealer_cards
                .score_hand()
                .expect("Failed to score hand.");
            match should_dealer_hit(score)? {
                true => {
                    let card = self
                        .shoe
                        .draw_card()
                        .expect(ErrorCode::FailedToDraw.inspect());
                    self.dealer_cards.add_card(card);
                }
                false => break,
            }
        }

        let dealer_score = get_best_score(
            self.dealer_cards
                .score_hand()
                .expect("Failed to score hand."),
        )?;

        self.game_state = GameState::END_HAND;

        Ok(dealer_score)
    }

    pub fn end_hand(&mut self) -> Result<(), String> {
        let player_score = get_best_score(
            self.player_cards
                .score_hand()
                .expect(ErrorCode::FailedToScoreHand.inspect()),
        )?;
        let dealer_score = get_best_score(
            self.dealer_cards
                .score_hand()
                .expect(ErrorCode::FailedToScoreHand.inspect()),
        )?;

        match player_score {
            Some(t) => match dealer_score {
                Some(v) => {
                    if t > v {
                        self.player
                            .credit(self.player_bet)
                            .expect(ErrorCode::FailedToCredit.inspect());
                    }
                }
                None => (),
            },
            None => (),
        }

        self.reset_hand()?;

        Ok(())
    }

    fn reset_hand(&mut self) -> Result<(), String> {
        self.dealer_cards = DealerHand::new();
        self.player_cards = PlayerHand::new();
        self.player_bet = 0;
        self.game_state = GameState::HAND_STARTED;

        Ok(())
    }
}

fn get_best_score(mut score: Vec<i64>) -> Result<Option<i64>, String> {
    score.sort_by(|a, b| b.cmp(a));

    for s in score {
        if s > 21 {
            continue;
        };
        return Ok(Some(s));
    }

    Ok(None)
}

fn should_dealer_hit(score: Vec<i64>) -> Result<bool, String> {
    let min_score = score.iter().min();

    match min_score {
        Some(t) => {
            if *t < 17 {
                return Ok(true);
            } else {
                return Ok(false);
            }
        }
        None => return Err("Could not derive minimum score for dealer.".into()),
    }
}

enum ErrorCode {
    FailedToScoreHand,
    FailedToDebit,
    FailedToDraw,
    FailedToCredit,
}

impl ErrorCode {
    pub fn inspect(&self) -> &str {
        let msg = match self {
            Self::FailedToScoreHand => "Failed to score hand.",
            Self::FailedToDebit => "Failed to debit player.",
            Self::FailedToDraw => "Failed to draw card.",
            Self::FailedToCredit => "Failed to credit player.",
        };

        return msg;
    }
}
