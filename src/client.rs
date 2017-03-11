use PullToken;
use PushToken;
use Payload;
use Config;
use FromConfig;

use serde_json;
use hyper::client::Client as HClient;

use std::default::Default;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

#[derive(Debug)]
pub struct Client {
    push_token: Option<PushToken>,
    pull_token: Option<PullToken>,
    client: HClient,
    server: SocketAddr,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            push_token: None,
            pull_token: None,
            client: HClient::new(),
            server: SocketAddr::from_str("0.0.0.0:10000").unwrap(),
        }
    }
}

unsafe impl Sync for Client {}
unsafe impl Send for Client {}

#[allow(dead_code)]
impl Client {
    pub fn new(server: SocketAddr, push_token: Option<PushToken>, pull_token: Option<PullToken>) -> Client {
        Client {
            push_token: push_token,
            pull_token: pull_token,
            client: HClient::new(),
            server: server,
        }
    }

    #[inline]
    fn server_str(&self) -> String {
        format!("https://{}:{}/", self.server.ip(), self.server.port())
    }

    pub fn update_server(&self) {
        assert!(self.push_token != None);
        println!("{}", self.server_str());

        self.client
            .post(&(self.server_str() + "update"))
            .header(self.push_token.clone().unwrap())
            .send()
            .unwrap();
    }

    pub fn pull(&self) -> SocketAddr {
        assert!(self.pull_token != None);
        let res = self.client
            .get(&self.server_str())
            .header(self.pull_token.clone().unwrap())
            .send()
            .unwrap();

        *serde_json::from_reader::<_, Payload>(res).unwrap()
    }
}

impl FromConfig for Client {
    fn from_config(config: &Config) -> Client {
        Client {
            push_token: Some(config.push_key.clone()),
            pull_token: Some(config.pull_key.clone()),
            client: HClient::new(),
            server: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 0)), config.port)
        }
    }
}
