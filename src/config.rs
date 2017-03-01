use std::fs::File;
use std::str::FromStr;

use tokens::{PushToken, PullToken};
use serde_json;

#[derive(Deserialize)]
struct TConfig {
    push_secret: String,
    pull_secret: String,
    port: u16,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub push_key: PushToken,
    pub pull_key: PullToken,
    pub port: u16,
}

impl Config {
    pub fn load() -> Config {
        let load: TConfig = serde_json::from_reader(File::open("ansible.json").unwrap()).unwrap();

        Config {
            push_key: PushToken::from_str(&load.push_secret).unwrap(),
            pull_key: PullToken::from_str(&load.pull_secret).unwrap(),
            port: load.port
        }
    }

    pub fn port_str(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
