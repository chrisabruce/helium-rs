use helium;

use rand::prelude::*;
use std::time::Duration;
use tokio::prelude::*;
use tokio::timer::Interval;

const PAY_INTERVAL: u64 = 10;

fn main() {
    let client = helium::Client::new("localhost", 4001);
    let accounts = client.list_accounts().unwrap();
    if accounts.len() < 1 {
        panic!("Requires two existing accounts.");
    }

    let five_seconds = Duration::new(PAY_INTERVAL, 0);
    let task = Interval::new_interval(five_seconds)
        .for_each(move |_| {
            //let client = helium::Client::new("localhost", 4001);
            //let accounts = match client.list_accounts().unwrap();

            let mut rng = rand::thread_rng();
            let amt: u64 = rng.gen_range(10_000_000_000, 100_000_000_000);

            print!("Paying: {}\n", amt);
            client
                .pay(&accounts[0].address, &accounts[1].address, amt)
                .unwrap();
            client
                .pay(&accounts[1].address, &accounts[0].address, amt)
                .unwrap();

            Ok(())
        })
        .map_err(|e| print!("interval errored; err={:?}\n", e));

    tokio::run(task);
}
