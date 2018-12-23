fn main() {
    let client = helium::Client::new("localhost", 4001);
    let status = client.status().unwrap();
    print!("{:?}", status);
}
