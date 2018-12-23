use helium;

fn main() {
    let node = helium::Node::new("localhost", 4001);
        let account = node.create_account().unwrap();
        println!("Account Created: {:?}", account);
}
