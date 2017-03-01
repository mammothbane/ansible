use std::net::SocketAddr;
use std::ops::Deref;

#[derive(Debug)]
pub struct BroadcastAddr {
    inner: Option<SocketAddr>,
}

impl BroadcastAddr {
    pub fn new() -> BroadcastAddr {
        BroadcastAddr {
            inner: None,
        }
    }

    pub fn load(&mut self, s: SocketAddr) {
        self.inner = Some(s)
    }

    pub fn inner(&self) -> Option<SocketAddr> {
        self.inner
    }
}

impl Deref for BroadcastAddr {
    type Target = Option<SocketAddr>;

    fn deref<'b>(&'b self) -> &'b Option<SocketAddr> {
        &self.inner
    }
}
