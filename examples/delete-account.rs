use helium;

fn main() {
    let node = helium::Node::new("localhost", 4001);
    node.list_accounts()
        .unwrap()
        .iter()
        .filter(|&a| a.balance < 1)
        .for_each(|a| {
            println!("Deleting: {}", a.address);
            node.delete_account(&a.address).unwrap();
        });
}
