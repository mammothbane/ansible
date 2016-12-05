extern crate ansible;
extern crate hyper;
extern crate config;
extern crate serde_json;

use config::reader::from_file;
use hyper::client::Client;

use std::path::Path;

fn main() {
    let cfg = from_file(Path::new("ansible_push.conf")).expect("unable to load config file");
    let server_addr = cfg.lookup_str("server").expect("server not present in config.");

    let client = Client::new();
    let res = client.get(&server_addr[..]).send().unwrap();
}
