extern crate ansible;

use ansible::{Config, Client, FromConfig};
use std::thread;
use std::time::Duration;

fn main() {
    let cfg = Config::load();
    let client = Client::from_config(&cfg);

    loop {
        thread::sleep(Duration::from_millis(10_000));
        client.update_server();
    }
}
