use std::fs::File;

use serde_json;

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub cert_pass: String,
    pub identity_file: String,
}

impl ServerConfig {
    pub fn load() -> ServerConfig {
        serde_json::from_reader(File::open("ansible_server.json").unwrap()).unwrap()
    }
}
