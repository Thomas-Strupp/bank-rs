#[derive(Debug)]
pub struct Account {
    pub id: String,
    pub balance: u32,
}

impl Account {
    pub fn new(id: String, balance:u32) -> Self {
        Self {
            id,
            balance,
        }
    }
    fn sufficient_funds(&self, funds:u32) -> bool {
        self.balance >= funds
    }
    pub fn withdraw(&mut self, funds: u32) {
        if self.sufficient_funds(funds) {
            self.balance -= funds
        }
    }
    pub fn deposit(&mut self, funds:u32) {
        self.balance += funds
    }
}
