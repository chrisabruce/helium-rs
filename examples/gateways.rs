use helium;

fn main() {
    let node = helium::Node::new("localhost", 4001);
    let gateways_raw = node.list_gateways_raw(None, None).unwrap();
    for gateway in gateways_raw.entries {
        println!("Gateway Address: {}", gateway.address);
    }
}
