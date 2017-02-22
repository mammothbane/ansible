#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate ansible;
extern crate rustc_serialize;

mod error;

use std::io::Cursor;
use std::net::SocketAddr;

use ansible::{Config, PullToken, PushToken};

use rocket::config::Config as RConfig;
use rocket::config::Environment;
use rocket::State;
use rocket::http::Status;
use rocket::response::Response;
use rocket::request::{Request, FromRequest};

#[derive(Debug)]
struct BroadcastAddr(Option<SocketAddr>);

#[get("/")]
fn index(addr: State<BroadcastAddr>, tok: PullToken) -> Response {
    match addr.0 {
        Some(x) => Response::build()
            .sized_body(Cursor::new(x.to_string()))
            .status(Status::Ok)
            .finalize(),
        None => Response::build()
            .status(Status::NotFound)
            .finalize()
    }
}

#[post("/update")]
fn update(tok: PushToken) {

}

fn main() {
    let cfg = Config::load();

    let rcfg = RConfig::build(Environment::Development)
        .address("0.0.0.0")
        .port(cfg.port)
        .unwrap();

    rocket::custom(rcfg, true)
        .mount("/", routes![index])
        .manage(BroadcastAddr(None))
        .launch();
}
