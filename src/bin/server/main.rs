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
use router::Router;
use config::reader::from_file;
use persistent::{Read, Write};
use rustc_serialize::hex::FromHex;

use std::path::Path;
use std::net::SocketAddr;

use ansible::{PushToken, PullToken, Update};
use error::ServerError;

struct State {
    address: Option<SocketAddr>
}

impl State {
    fn new() -> State {
        State{address: None}
    }

    fn update(&mut self, addr: Option<SocketAddr>) {
        self.address = Some(addr.clone())
    }

    fn address(&self) {
        self.address
    }

    // fn update_handler(&mut self, req: &mut Request) -> IronResult<Response> {
    //     self.address = Some(req.remote_addr.clone());
    //     Ok(Response::with((status::Ok, "updated")))
    // }
    //
    // fn get_handler(&self, req: &mut Request) -> IronResult<Response> {
    //     Ok(Response::with((status::Ok, format!("{:?}", self.address))))
    // }
}

// static mut ADDRESS: Option<SocketAddr> = None;
//
// fn update_handler(req: &mut Request) -> IronResult<Response> {
//     unsafe {
//         ADDRESS = Some(req.remote_addr.clone());
//     }
//
//     Ok(Response::with((status::Ok, "update")))
// }
//
// fn retrieve_handler(req: &mut Request) -> IronResult<Response> {
//     unsafe {
//         Ok(Response::with((status::Ok, format!("{:?}", ADDRESS))))
//     }
// }

fn main() {
    let cfg = from_file(Path::new("ansible.conf")).expect("Failed to get config file.");
    let sock_str = format!("0.0.0.0:{}", cfg.lookup_integer32("port").unwrap());
    let mut router = Router::new();

    let push_token = cfg.lookup_str("push_secret").unwrap().to_owned();
    let pull_token = cfg.lookup_str("pull_secret").unwrap().to_owned();

    let mut state = State::new();

    let mut ret = Chain::new(move |r: &mut Request| {
        Ok(Response::with((status::Ok, format!("{:?}", state.address()))))
    });
    let mut upd = Chain::new(move |r: &mut Request| {
        let mutex = request.get::<Read<State>>;
        let addr = mutex.lock().unwrap();

        state.update_handler(r)
    });

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
