use std::cell::RefCell;
use std::net::SocketAddr;
use std::rc::Rc;

use iron::typemap::Key;

#[derive(Debug, Clone)]
pub struct Addr(Rc<RefCell<Option<SocketAddr>>>);

unsafe impl Sync for Addr {}
unsafe impl Send for Addr {}

impl Addr {
    pub fn new() -> Addr {
        Addr(Rc::new(RefCell::new(None)))
    }

    pub fn update(&mut self, addr: SocketAddr) {
        *self.0.borrow_mut() = Some(addr)
    }

    pub fn address(&self) -> Option<SocketAddr> {
        *self.0.borrow()
    }
}

impl Key for Addr {
    type Value = Addr;
}
