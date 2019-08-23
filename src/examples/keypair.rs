use helium;

fn main() {
   let (p, s) = helium::crypto::generate_keypair("test");
   println!("{} - {}", p, s);
}