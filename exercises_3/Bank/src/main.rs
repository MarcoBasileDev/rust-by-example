#[derive(Debug)]
struct Account {
    balance: i32,
    id: u32,
    holder: String
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: Vec::new() }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

fn main() {
    let mut bank = Bank::new();
    let account = Account::new(1, String::from("test"));

    bank.add_account(account);
    println!("{:?}", bank);

}
