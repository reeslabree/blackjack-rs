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