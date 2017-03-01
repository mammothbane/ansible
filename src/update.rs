use std::net::SocketAddr;
use std::ops::Deref;

use iron::typemap::Key;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Update(SocketAddr);

impl Key for Update {
    type Value = SocketAddr;
}

impl Update {
    pub fn new(addr: SocketAddr) -> Self {
        Update(addr)
    }
}

impl Deref for Update {
    type Target = SocketAddr;

    fn deref(&self) -> &SocketAddr {
        &self.0
    }
}
