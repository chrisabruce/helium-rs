use helium;

use std::time::Duration;
use tokio::prelude::*;
use tokio::timer::Interval;

const POLL_INTERVAL: u64 = 10;

fn main() {
    let node = helium::Node::new("localhost", 4001);
    let mut last_height = node.status().unwrap().chain_height;
    let accounts = node.list_accounts().unwrap();
    seed_account = accounts.iter.filter(|a| a.name.unwrap_or("empty") == "Main");

    let interval = Duration::new(POLL_INTERVAL, 0);
    let task = Interval::new_interval(interval)
        .for_each(move |_| {
            print!("Checking...\n");
            let cur_height = node.status().unwrap().node_height; // want to make sure node is current
            if cur_height > last_height {
                print!("New height: {}\n", cur_height);
                last_height = cur_height;
            }
            Ok(())
        })
        .map_err(|e| print!("interval errored; err={:?}\n", e));

    tokio::run(task);
}
