extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate config;
extern crate ansible;
extern crate rustc_serialize;

mod auth;
mod error;

use iron::prelude::*;
use iron::status;
use router::Router;
use config::reader::from_file;
use rustc_serialize::hex::FromHex;

use std::path::Path;
use std::net::SocketAddr;

use ansible::{PushToken, PullToken, Update};
use auth::Auth;

static mut ADDRESS: Option<SocketAddr> = None;

fn update_handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "update")))
}

fn retrieve_handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "repsonse")))
}

fn main() {
    let cfg = from_file(Path::new("ansible.conf")).expect("Failed to get config file.");

    let push_tokens = cfg.lookup_str("push_secret").unwrap();
    let pull_tokens = cfg.lookup_str("pull_secret").unwrap();

    let push_token = push_tokens.from_hex().unwrap() as usize;
    let pull_token = pull_tokens.from_hex().unwrap() as usize;

    let sock_str = format!("0.0.0.0:{}", cfg.lookup_integer32("port").unwrap());
    let mut router = Router::new();

    let upd = Chain::new(update_handler).link_before(|r: &mut Request| {
        match req.headers.get::<PushToken>() {
            Ok(x) if x == self.key => Ok(()),
            _ => Err(IronError::new(ServerError("Invalid auth token."), 401))
        }
    });
    let ret = Chain::new(retrieve_handler).link_before(Auth::new(PullToken(pull_token)));

    router.get("/update", upd, "update ip");
    router.get("/", ret, "ip response");

    Iron::new(router).http(&sock_str[..]).unwrap();
}
