#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new (id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct  Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn print_account(account: Account) -> Account{
    println!("{:#?}", account);
    account // return account (binding) allows me to use it again
}



fn main() {
   // let bank  = Bank::new();
    let  mut account = Account::new(1,String::from("me"));

    //println!("{:#?}", bank);
    account = print_account(account); ///// ownership moved to the function but returned, so it can be used again

    //print_account(account);
    account = print_account(account);

    println!("{:#?}",account)


}
