#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate ansible;
extern crate rustc_serialize;
#[macro_use] extern crate rocket_contrib;

mod error;
mod broadcast_addr;

use broadcast_addr::BroadcastAddr;

use ansible::{Config, PullToken, PushToken, Update};

use std::net::SocketAddr;
use std::ops::Deref;

use rocket::config::Config as RConfig;
use rocket::config::Environment;
use rocket::State;
use rocket::http::Status;
use rocket::request::Request;

use rocket_contrib::JSON;

#[get("/")]
fn index(addr: State<BroadcastAddr>, cfg: State<Config>, tok: Option<PullToken>) -> Result<String, Status> {
    match tok {
        Some(ref x) if *x == cfg.pull_key => (),
        _ => return Err(Status::Unauthorized),
    }

    match *addr.inner().deref() {
        Some(x) => Ok(x.to_string()),
        None => Err(Status::NotFound),
    }
}

#[post("/update", rank = 1, data = "<newAddr>")]
fn update_json(newAddr: JSON<Update>, mut addr: State<BroadcastAddr> , tok: Option<PushToken>, cfg: State<Config>) -> Result<(), Status> {
    match tok {
        Some(ref x) if *x == cfg.push_key => (),
        _ => return Err(Status::Unauthorized)
    }

    addr.load(newAddr.addr());
    Ok(())
}

#[post("/update", rank = 2)]
fn update(remote: SocketAddr, mut addr: State<BroadcastAddr>, tok: Option<PushToken>, cfg: State<Config>) -> Result<(), Status> {
    match tok {
        Some(ref x) if *x == cfg.push_key => (),
        _ => return Err(Status::Unauthorized),
    }

    addr.load(remote);
    Ok(())
}

#[error(401)]
fn unauthorized() -> String {
    "You are not authorized to do that.".to_owned()
}

fn main() {
    let cfg = Config::load();
    println!("{:?}", cfg);

    let rcfg = RConfig::build(Environment::Development)
        .address("0.0.0.0")
        .port(cfg.port)
        .unwrap();

    rocket::custom(rcfg, true)
        .mount("/", routes![index, update])
        .manage(BroadcastAddr::new())
        .manage(cfg)
        .catch(errors![unauthorized])
        .launch();
}
