#[macro_use] extern crate hyper;
extern crate serde_json;
extern crate iron;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

use iron::typemap::Key;

const PUSH_KEY: &'static str = "X-Ansible-PushToken";
const PULL_KEY: &'static str = "X-Ansible-PullToken";

header! { (PushToken, PUSH_KEY) => [String] }
header! { (PullToken, PULL_KEY) => [String] }

impl Key for Update {
    type Value = std::net::IpAddr;
}

impl Update {
    pub fn new(addr: std::net::IpAddr) -> Self {
        Update{address: addr}
    }
}
