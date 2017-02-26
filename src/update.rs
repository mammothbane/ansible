use std::net::SocketAddr;
use std::ops::Deref;

#[derive(Serialize, Deserialize)]
pub struct Update(SocketAddr);

impl Update {
    pub fn addr(&self) -> SocketAddr {
        self.0
    }
}

impl Deref for Update {
    type Target = SocketAddr;

    fn deref<'a>(&'a self) -> &'a SocketAddr {
        &self.0
    }
}
