use helium;

fn main() {
    let client = helium::Client::new("localhost", 4001);
    let accounts = client.list_accounts().unwrap();
    //print!("found: {:?}", accounts);
    for account in accounts {
        print!(
            "\nAccount: {}({})\nBalance: {}\nNonce: {}\n",
            account.name, account.address, account.balance, account.nonce
        );
    }
}
