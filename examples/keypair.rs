use helium::crypto;

fn main() {
    //let phrase = "park remain person kitchen mule spell knee armed position rail grid ankle";
    let phrase = "cliff crater normal poet canal wool birth omit whip early kid reduce";
    let (p, s) = crypto::generate_keypair(phrase);
    println!("{:?} - {:?}", p, s);
}