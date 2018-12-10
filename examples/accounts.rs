extern crate helium;


fn main() {
    let client = helium::Client::new("localhost", 4001);
    let accounts = client.list_accounts().unwrap();
    print!("found: {:?}", accounts);

    if accounts.len() > 0 {
        let account = accounts.first().unwrap();
        print!("\nAccount: {}\nBalance: {}\n", account.name, account.balance);
    }
}