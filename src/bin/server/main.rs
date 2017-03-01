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
use std::sync::Mutex;

use rocket::config::Config as RConfig;
use rocket::config::Environment;
use rocket::State;
use rocket::http::Status;
use rocket::request::Request;

use rocket_contrib::JSON;

#[get("/")]
fn index(cfg: State<Config>, tok: Option<PullToken>) -> Result<String, Status> {
    match tok {
        Some(ref x) if *x == cfg.pull_key => (),
        _ => return Err(Status::Unauthorized),
    }

    unsafe {
        let adr: Option<SocketAddr> = addr.lock().expect("Failed to lock on socket address.").inner();
        match adr {
            Some(x) => Ok(x.to_string()),
            None => Err(Status::NotFound),
        }        
    }
}

#[post("/update", rank = 1, data = "<newAddr>")]
fn update_json(newAddr: JSON<Update>, tok: Option<PushToken>, cfg: State<Config>) -> Result<(), Status> {
    match tok {
        Some(ref x) if *x == cfg.push_key => (),
        _ => return Err(Status::Unauthorized)
    }

    unsafe { addr.lock().expect("unable to lock address").load(newAddr.addr()); }
    Ok(())
}

#[post("/update", rank = 2)]
fn update(remote: SocketAddr, tok: Option<PushToken>, cfg: State<Config>) -> Result<(), Status> {
    match tok {
        Some(ref x) if *x == cfg.push_key => (),
        _ => return Err(Status::Unauthorized),
    }

    unsafe { addr.lock().expect("unable to lock address").load(remote); }
    Ok(())
}

#[error(401)]
fn unauthorized() -> String {
    "You are not authorized to do that.".to_owned()
}

static mut addr: Mutex<BroadcastAddr> = Mutex::new(BroadcastAddr::new());

fn main() {
    let cfg = Config::load();
    println!("{:?}", cfg);

    let rcfg = RConfig::build(Environment::Development)
        .address("0.0.0.0")
        .port(cfg.port)
        .unwrap();

    rocket::custom(rcfg, true)
        .mount("/", routes![index, update])
        .manage(unsafe { addr })
        .manage(cfg)
        .catch(errors![unauthorized])
        .launch();
}
