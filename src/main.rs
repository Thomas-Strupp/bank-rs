// Vecs and Hashmaps
// Modularity for code layouts, and importing code
// Custom types (Enums, Generics, and Traits)
use std::collections::HashMap;
fn main() {
    let mut my_bank = Bank::new();
    my_bank.add_account("08802", 0);
    my_bank.add_account("19122", 0);
    my_bank.add_account("28027", 0);
    my_bank.add_account("18045", 0);
    if let Some(account) = my_bank.access_account("18045") {
        account.deposit(500);
        account.withdraw(400);
        println!("Account: {}, Balance: {:?}", account.id, account.balance);
    }
    println!("{:#?}", my_bank);
}

#[derive(Debug)]
struct Bank {
    accounts: HashMap<String, Account>,
}
impl Bank {
    fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }
    fn add_account(&mut self, id: &str, balance: u32) {
        let account = Account::new(id.to_string(), balance);
        self.accounts.insert(id.to_string(), account);
    }
    fn access_account(&mut self, id: &str) -> Option<&mut Account> {
        self.accounts.get_mut(id)
    }
}

#[derive(Debug)]
struct Account {
    id: String,
    balance: u32,
}
impl Account {
    fn new(id: String, balance:u32) -> Self {
        Self {
            id,
            balance,
        }
    }
    fn sufficient_funds(&self, funds:u32) -> bool {
        self.balance >= funds
    }
    fn withdraw(&mut self, funds: u32) {
        if self.sufficient_funds(funds) {
            self.balance -= funds
        }
    }
    fn deposit(&mut self, funds:u32) {
        self.balance += funds
    }
}
