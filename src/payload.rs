use std::net::SocketAddr;
use std::ops::Deref;

use iron::typemap::Key;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payload(SocketAddr);

impl Key for Payload {
    type Value = SocketAddr;
}

impl Payload {
    pub fn new(addr: SocketAddr) -> Self {
        Payload(addr)
    }
}

impl Deref for Payload {
    type Target = SocketAddr;

    fn deref(&self) -> &SocketAddr {
        &self.0
    }
}
