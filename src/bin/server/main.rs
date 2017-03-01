extern crate iron;
extern crate router;
extern crate persistent;
extern crate bodyparser;
extern crate ansible;

extern crate hyper_native_tls;

extern crate serde_json;
extern crate rustc_serialize;
extern crate rpassword;

mod error;
mod addr;
mod routes;

use addr::Addr;
use routes::{index, update};

use iron::prelude::*;
use iron::status;
use persistent::State;
use router::Router;
use hyper_native_tls::NativeTlsServer;

use rpassword::prompt_password_stderr;

use ansible::{PushToken, PullToken, Config};
use error::ServerError;

fn router(cfg: Config) -> Router {
    let mut router = Router::new();

    let mut ret = Chain::new(index);
    let mut upd = Chain::new(update);

    let adr = Addr::new();

    ret.link(State::<Addr>::both(adr.clone()));
    upd.link(State::<Addr>::both(adr.clone()));

    let pull: PullToken = cfg.pull_key;
    let push: PushToken = cfg.push_key;

    ret.link_before(move |r: &mut Request| {
        match r.headers.get::<PullToken>() {
            Some(x) if *x == pull => Ok(()),
            _ => Err(IronError::new(ServerError("Invalid pull token.".to_owned()), status::Unauthorized))
        }
    });

    upd.link_before(move |r: &mut Request| {
        match r.headers.get::<PushToken>() {
            Some(x) if *x == push => Ok(()),
            _ => Err(IronError::new(ServerError("Invalid push token.".to_owned()), status::Unauthorized))
        }
    });

    router.get("/update", upd, "ping with updated ip");
    router.get("/", ret, "retrieve current ip");

    router
}

fn main() {
    let cfg = Config::load();
    let port_str = cfg.port_str();

    let pass = prompt_password_stderr("SSL Cert Password: ").unwrap();
    let ssl = NativeTlsServer::new("identity.p12", &pass[..]).unwrap();

    println!("Starting server.");
    Iron::new(router(cfg)).https(port_str, ssl).unwrap();
}
