#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub(crate) enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub(crate) enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[non_exhaustive]
pub(crate) struct GameState;
impl GameState {
    pub const HAND_STARTED: u8 = 00;
    pub const PLAYER_TURN: u8 = 01;
    pub const DEALER_TURN: u8 = 02;
    pub const END_HAND: u8 = 03;
}