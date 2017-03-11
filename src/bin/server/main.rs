extern crate iron;
extern crate router;
extern crate persistent;
extern crate bodyparser;
extern crate ansible;

extern crate hyper_native_tls;

#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rpassword;

mod error;
mod addr;
mod routes;
mod server_config;

use addr::Addr;
use routes::{index, update};
use server_config::ServerConfig;

use iron::prelude::*;
use iron::status;
use persistent::State;
use router::Router;
use hyper_native_tls::NativeTlsServer;

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
    router.get("/pull", ret, "retrieve current ip");
    router.get("/", move |_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello.")))
    }, "index");

    router
}

fn main() {
    let cfg = Config::load();
    let server_cfg = ServerConfig::load();
    let port_str = cfg.port_str();

    let cert_pass = &server_cfg.cert_pass[..];
    let identity_file = &server_cfg.identity_file[..];

    let ssl = NativeTlsServer::new(identity_file, cert_pass).expect("Unable to create TLS server with provided identity and password.");

    println!("Starting server.");
    Iron::new(router(cfg)).https(port_str, ssl).unwrap();
}
