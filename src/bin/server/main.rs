#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate ansible;
extern crate rustc_serialize;

mod error;

use ansible::{Config, PullToken, PushToken};

use std::io::Cursor;
use std::net::SocketAddr;

use rocket::config::Config as RConfig;
use rocket::config::Environment;
use rocket::State;
use rocket::http::Status;
use rocket::response::Response;
use rocket::request::{Request, FromRequest};

#[derive(Debug, Clone, Copy)]
struct BroadcastAddr(Option<SocketAddr>);

#[get("/")]
fn index<'a>(addr: State<'a, BroadcastAddr>, cfg: State<Config>, tok: PullToken) -> Response<'a> {
    if tok != cfg.pull_key {
        return Response::build().status(Status::Unauthorized).finalize()
    }

    match addr.0 {
        Some(x) => Response::build()
            .sized_body(Cursor::new(x.to_string()))
            .status(Status::Ok)
            .finalize(),
        None => Response::build()
            .status(Status::NotFound)
            .finalize(),
    }
}

#[post("/update")]
fn update(tok: Option<PushToken>, cfg: State<Config>) {

}

fn main() {
    let cfg = Config::load();
    println!("{:?}", cfg);

    let rcfg = RConfig::build(Environment::Development)
        .address("0.0.0.0")
        .port(cfg.port)
        .unwrap();

    rocket::custom(rcfg, true)
        .mount("/", routes![index])
        .manage(BroadcastAddr(None))
        .manage(cfg)
        .launch();
}
