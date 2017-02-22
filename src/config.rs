use std::fs::File;
use rustc_serialize::hex::FromHex;
use serde_json;

#[derive(Deserialize)]
struct TConfig {
    push_key: String,
    pull_key: String,
    port: u16,
}

pub struct Config {
    pub push_key: Vec<u8>,
    pub pull_key: Vec<u8>,
    pub port: u16,
}

impl Config {
    pub fn load() -> Config {
        let load: TConfig = serde_json::from_reader(File::open("ansible.json").unwrap()).unwrap();
        let push_key = load.push_key.from_hex().unwrap();
        let pull_key = load.pull_key.from_hex().unwrap();

        assert!(push_key.len() >= 8, "Push key too short!");
        assert!(pull_key.len() >= 8, "Pull key too short!");

        Config {
            push_key: load.push_key.from_hex().unwrap(),
            pull_key: load.pull_key.from_hex().unwrap(),
            port: load.port
        }
    }
}
