extern crate iron;
extern crate router;
extern crate persistent;
extern crate bodyparser;
extern crate config;
extern crate ansible;
extern crate rustc_serialize;

mod error;

use iron::prelude::*;
use iron::status;
use iron::Handler;
use iron::typemap::Key;
use iron::headers::Header;

use router::Router;
use config::reader::from_file;
use persistent::{Read, Write, State};
use rustc_serialize::hex::FromHex;

use std::boxed::Box;
use std::path::Path;
use std::net::SocketAddr;

use ansible::{PushToken, PullToken, Update};
use error::ServerError;

struct Addr(Option<SocketAddr>);

impl Addr {
    fn new() -> Addr {
        Addr(None)
    }

    fn update(&mut self, addr: SocketAddr) {
        self.0 = Some(addr)
    }

    fn address(&self) -> Option<SocketAddr> {
        self.0
    }
}

impl Key for Addr {
    type Value = Addr;
}

fn main() {
    let cfg = from_file(Path::new("ansible.conf")).expect("Failed to get config file.");
    let sock_str = format!("0.0.0.0:{}", cfg.lookup_integer32("port").unwrap());
    let mut router = Router::new();

    let push_token = cfg.lookup_str("push_secret").unwrap().to_owned();
    let pull_token = cfg.lookup_str("pull_secret").unwrap().to_owned();

    let adr = Box::new(Addr::new());

    let mut ret = Chain::new(move |r: &mut Request| {
        let st = r.get::<State<Addr>>().unwrap().read().unwrap();
        match st.address() {
            Some(addr) => Ok(Response::with((status::Ok, format!("{:?}", addr)))),
            None => Ok(Response::with((status::Ok, "FACK"))),
        }
    });

    let mut upd = Chain::new(move |r: &mut Request| {
        let mutex = r.get::<State<Addr>>().unwrap();
        let mut addr = mutex.write().unwrap();

        (*addr).update(r.remote_addr);
        Ok(Response::with((status::Ok, "updated!")))
    });

    ret.link(State::<Box<Addr>>::both(adr));
    upd.link(State::<Box<Addr>>::both(adr.clone()));

    upd.link_before(move |r: &mut Request| {
        match r.headers.get::<PushToken>() {
            Some(x) if x.0 == push_token => Ok(()),
            _ => Err(IronError::new(ServerError("Invalid push token.".to_owned()), status::Unauthorized))
        }
    });

    ret.link_before(move |r: &mut Request| {
        match r.headers.get::<PullToken>() {
            Some(x) if x.0 == pull_token => Ok(()),
            _ => Err(IronError::new(ServerError("Invalid pull token.".to_owned()), status::Unauthorized))
        }
    });

    router.get("/update", upd, "ping with updated ip");
    router.get("/", ret, "retrieve current ip");


    Iron::new(router).http(&sock_str[..]).unwrap();
}
