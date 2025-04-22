use std::collections::HashMap;
use crate::account::Account;

#[derive(Debug)]
pub struct Bank {
    pub accounts: HashMap<String, Account>,
}
impl Bank {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }
    pub fn add_account(&mut self, id: &str, balance: u32) {
        let account = Account::new(id.to_string(), balance);
        self.accounts.insert(id.to_string(), account);
    }
    pub fn access_account(&mut self, id: &str) -> Option<&mut Account> {
        self.accounts.get_mut(id)
    }
}