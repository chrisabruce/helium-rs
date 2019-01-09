use helium;

fn main() {
    let node = helium::Node::new_with_timeout("localhost", 4001, 240);
    let accounts = node.list_accounts().unwrap();
    println!("found: {} accounts", accounts.len());

    let account = node.create_account().unwrap();
    println!("New Master Account: {}", account.address);

    for a in accounts {
        node.pay(&a.address, &account.address, a.balance).unwrap();
        //node.delete_account(&a.address).unwrap();
    }
}
