#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate ansible;
extern crate rustc_serialize;
extern crate rocket_contrib;

mod error;
mod broadcast_addr;

use broadcast_addr::BroadcastAddr;

use ansible::{Config, PullToken, PushToken, Update};

use std::net::SocketAddr;

use rocket::config::Config as RConfig;
use rocket::config::Environment;
use rocket::State;
use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::response::status::Custom;

use rocket_contrib::JSON;

struct StatusResponder(u16);

impl StatusResponder {
    pub fn new(v: u16) -> StatusResponder {
        StatusResponder(v)
    }

    pub fn err<T>(v: u16) -> Result<T, StatusResponder> {
        Err(StatusResponder::new(v))
    }
}

impl<'r> Responder<'r> for StatusResponder {
    fn respond(self) -> Result<Response<'r>, Status> {
        Err(Status::from_code(self.0).unwrap())
    }
}

#[get("/")]
fn index(cfg: State<Config>, addr: State<BroadcastAddr>, tok: Option<PullToken>) -> Result<String, StatusResponder> {
    match tok {
        Some(ref x) if *x == cfg.pull_key => (),
        _ => return StatusResponder::err(401),
    }

    match (*addr).inner() {
        Some(x) => Ok(x.to_string()),
        None => StatusResponder::err(404),
    }
}

#[post("/update", rank = 1, data = "<new_addr>")]
fn update_json(new_addr: JSON<Update>, tok: Option<PushToken>, addr: State<BroadcastAddr>, cfg: State<Config>) -> Result<(), StatusResponder> {
    match tok {
        Some(ref x) if *x == cfg.push_key => (),
        _ => return StatusResponder::err(401)
    }

    (*addr).load(**new_addr);
    Ok(())
}

#[post("/update", rank = 2)]
fn update(remote: SocketAddr, addr: State<BroadcastAddr>, tok: Option<PushToken>, cfg: State<Config>) -> Result<(), StatusResponder> {
    match tok {
        Some(ref x) if *x == cfg.push_key => (),
        _ => return StatusResponder::err(401),
    }

    (*addr).load(remote);
    Ok(())
}

#[error(401)]
fn unauthorized() -> String {
    "You are not authorized to do that.".to_owned()
}

fn main() {
    let cfg = Config::load();
    let addr = BroadcastAddr::new();
    println!("{:?}", cfg);

    let rcfg = RConfig::build(Environment::Development)
        .address("0.0.0.0")
        .port(cfg.port)
        .unwrap();

    rocket::custom(rcfg, true)
        .mount("/", routes![index, update, update_json])
        .manage(addr)
        .manage(cfg)
        .catch(errors![unauthorized])
        .launch();
}
