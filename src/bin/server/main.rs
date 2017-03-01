extern crate iron;
extern crate router;
extern crate persistent;
extern crate bodyparser;
extern crate config;
extern crate ansible;
extern crate rustc_serialize;

mod error;
mod addr;
mod routes;

use addr::Addr;
use routes::{index, update};

use iron::prelude::*;
use iron::status;
use persistent::State;

use router::Router;
use config::reader::from_file;

use std::path::Path;

use ansible::{PushToken, PullToken};
use error::ServerError;

fn router(push_tok: String, pull_tok: String) -> Router {
    let mut router = Router::new();

    let mut ret = Chain::new(index);
    let mut upd = Chain::new(update);

    let adr = Addr::new();

    ret.link(State::<Addr>::both(adr.clone()));
    upd.link(State::<Addr>::both(adr.clone()));

    upd.link_before(move |r: &mut Request| {
        match r.headers.get::<PushToken>() {
            Some(x) if x.0 == push_tok => Ok(()),
            _ => Err(IronError::new(ServerError("Invalid push token.".to_owned()), status::Unauthorized))
        }
    });

    ret.link_before(move |r: &mut Request| {
        match r.headers.get::<PullToken>() {
            Some(x) if x.0 == pull_tok => Ok(()),
            _ => Err(IronError::new(ServerError("Invalid pull token.".to_owned()), status::Unauthorized))
        }
    });

    router.get("/update", upd, "ping with updated ip");
    router.get("/", ret, "retrieve current ip");

    router
}

fn main() {
    let cfg = from_file(Path::new("ansible.conf")).expect("Failed to get config file.");
    let sock_str = format!("0.0.0.0:{}", cfg.lookup_integer32("port").unwrap());
    let push_token = cfg.lookup_str("push_secret").unwrap().to_owned();
    let pull_token = cfg.lookup_str("pull_secret").unwrap().to_owned();

    Iron::new(router(push_token, pull_token)).http(&sock_str[..]).unwrap();
}
