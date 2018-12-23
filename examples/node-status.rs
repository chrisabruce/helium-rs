fn main() {
    let node = helium::Node::new("localhost", 4001);
    let status = node.status().unwrap();
    print!("{:?}", status);
}
