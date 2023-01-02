use std::error::Error;

pub struct Player {
    balance: i64
}

impl Player {
    pub fn new() -> Self {
        Self {
            balance: 0
        }
    }

    pub fn get_balance(&self) -> i64 {
        self.balance
    }

    pub(in crate::blackjack) fn debit(&mut self, amount: i64) -> Result<i64, Box<dyn Error>> {
        if self.balance < amount {
            return Err("Balance too low".into());
        }

        let res = self.balance.checked_sub(amount);
        match res {
            Some(t) => Ok(t),
            None => Err("Overflow.".into())
        }
    }

    pub(in crate::blackjack) fn credit(&mut self, amount: i64) -> Result<i64, Box<dyn Error>> {
        let res = self.balance.checked_add(amount);
        match res {
            Some(t) => Ok(t),
            None => Err("Overflow.".into())
        }
    }
}