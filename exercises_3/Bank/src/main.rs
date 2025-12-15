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

fn print_account(account: &Account){
    println!("Here's your account: {:#?}", account)
}

fn change_account_balance(account: &mut Account) {
    account.balance = 10;
    println!("Here's your new balance: {:#?}", account)
}

fn add_account(bank: &mut Bank, account: Account) {
    bank.accounts.push(account);
}

fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("Me"));

    print_account(&account);

    change_account_balance(&mut account);

    add_account(&mut bank, account);
    println!("Bank {:#?}", bank);
}
