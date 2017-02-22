extern crate ansible;
extern crate config;

use config::reader::from_file;
use std::path::Path;

fn main() {
    let cfg = from_file(Path::new("ansible_push.conf")).expect("unable to load config file");
    let server_addr = cfg.lookup_str("server").expect("server not present in config.");

}
