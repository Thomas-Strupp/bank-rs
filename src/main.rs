// Vecs and Hashmaps
// Modularity for code layouts, and importing code
// Custom types (Enums, Generics, and Traits)
use bank::Bank;
use bank::Account;

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



