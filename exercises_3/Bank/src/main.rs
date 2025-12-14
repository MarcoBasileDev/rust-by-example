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
    let bank = Bank::new();
    let account = Account::new(1, String::from("Me"));

    println!("Bank {:#?}", bank);
    println!("Account {:#?}", account);
}
