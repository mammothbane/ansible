extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate config;
extern crate ansible;
extern crate rustc_serialize;

mod error;

use iron::prelude::*;
use iron::status;
use router::Router;
use config::reader::from_file;
use rustc_serialize::hex::FromHex;

use std::path::Path;
use std::net::SocketAddr;

use ansible::{PushToken, PullToken, Update};
use error::ServerError;

static mut ADDRESS: Option<SocketAddr> = None;

fn update_handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "update")))
}

fn retrieve_handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "repsonse")))
}

fn main() {
    let cfg = from_file(Path::new("ansible.conf")).expect("Failed to get config file.");
    let sock_str = format!("0.0.0.0:{}", cfg.lookup_integer32("port").unwrap());
    let mut router = Router::new();

    let push_token = cfg.lookup_str("push_secret").unwrap().to_owned();
    let pull_token = cfg.lookup_str("pull_secret").unwrap().to_owned();


    let mut upd = Chain::new(update_handler);
    upd.link_before(move |r: &mut Request| {
        match r.headers.get::<PushToken>() {
            Some(x) if x.0 == push_token => Ok(()),
            _ => Err(IronError::new(ServerError("Invalid push token.".to_owned()), status::Unauthorized))
        }
    });

    let mut ret = Chain::new(retrieve_handler);
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
