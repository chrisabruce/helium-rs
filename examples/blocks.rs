use helium;

fn main() {
    let node = helium::Node::new("localhost", 4001);
    let blocks = node.list_blocks(Some(3003)).unwrap();
    for block in blocks {
        println!("Block: {:?}", block);
    }
}
