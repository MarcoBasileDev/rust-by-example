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

    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: i32) {
        self.balance -= amount;
    }

    fn summary(&self) -> String {
        format!("{} has a balance {}", self.holder, self.balance)
    }
}

fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("test"));

    account.deposit(500);
    account.withdraw(250);

    println!("{}", account.summary());

    bank.add_account(account);
    println!("{:#?}",bank);
}
