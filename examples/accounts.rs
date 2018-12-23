use helium;

fn main() {
    let node = helium::Node::new("localhost", 4001);
    let accounts = node.list_accounts().unwrap();
    //print!("found: {:?}", accounts);
    for account in accounts {
        print!(
            "\nAccount: {}({})\nBalance: {}\n",
            account.name.unwrap_or("Unnamed Account".to_string()),
            account.address,
            account.balance
        );
    }
}
