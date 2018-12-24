use helium;

fn main() {
    let node = helium::Node::new("localhost", 4001);
    let blocks = node.list_blocks(None).unwrap();
    for block in blocks {
        println!("Block: {:?}", block);
    }
}
