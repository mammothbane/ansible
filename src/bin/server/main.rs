extern crate iron;
extern crate router;
extern crate persistent;
extern crate bodyparser;
extern crate config;
extern crate ansible;
extern crate rustc_serialize;

mod error;
mod addr;

use addr::Addr;

use iron::prelude::*;
use iron::status;

use router::Router;
use config::reader::from_file;
use persistent::State;

use std::path::Path;

use ansible::{PushToken, PullToken};
use error::ServerError;

fn main() {
    let cfg = from_file(Path::new("ansible.conf")).expect("Failed to get config file.");
    let sock_str = format!("0.0.0.0:{}", cfg.lookup_integer32("port").unwrap());
    let mut router = Router::new();

    let push_token = cfg.lookup_str("push_secret").unwrap().to_owned();
    let pull_token = cfg.lookup_str("pull_secret").unwrap().to_owned();

    let adr = Addr::new();

    let mut ret = Chain::new(move |r: &mut Request| {
        let st = r.get::<State<Addr>>().unwrap();
        let addr = st.read().unwrap();

        match addr.address() {
            Some(addr) => Ok(Response::with((status::Ok, format!("{:?}", addr)))),
            None => Ok(Response::with((status::Ok, "no remote address present"))),
        }
    });

    let mut upd = Chain::new(move |r: &mut Request| {
        let mutex = r.get::<State<Addr>>().unwrap();
        let mut addr = mutex.write().unwrap();

        println!("updating with {:?}", r.remote_addr);

        (*addr).update(r.remote_addr);
        Ok(Response::with((status::Ok, "updated!")))
    });

    ret.link(State::<Addr>::both(adr.clone()));
    upd.link(State::<Addr>::both(adr.clone()));

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
