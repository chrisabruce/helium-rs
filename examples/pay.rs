use helium;

use rand::prelude::*;

fn main() {
    let client = helium::Client::new("localhost", 4001);
    let accounts = client.list_accounts().unwrap();

    if accounts.len() > 1 {
        //loop {
        let mut rng = rand::thread_rng();
        let amt: u64 = rng.gen_range(100, 10_000_000);

        print!("Paying: {}", amt);
        client
            .pay(&accounts[0].address, &accounts[1].address, amt)
            .unwrap();

        //}
    }
}
