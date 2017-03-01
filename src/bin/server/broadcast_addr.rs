use std::net::SocketAddr;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct BroadcastAddr(Rc<RefCell<Option<SocketAddr>>>);

impl BroadcastAddr {
    pub fn new() -> BroadcastAddr {
        BroadcastAddr(Rc::new(RefCell::new(None)))
    }

    pub fn load(&self, s: SocketAddr) {
        *self.0.borrow_mut() = Some(s)
    }

    pub fn inner(&self) -> Option<SocketAddr> {
        *self.0.borrow()
    }
}

unsafe impl Sync for BroadcastAddr {}
unsafe impl Send for BroadcastAddr {}
