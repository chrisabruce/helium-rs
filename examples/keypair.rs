use helium::crypto;

fn main() {
   let (p, s) = crypto::generate_keypair("cliff crater normal poet canal wool birth omit whip early kid reduce");
   println!("{:?} - {:?}", p, s);
}