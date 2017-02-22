#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate config;
extern crate ansible;
extern crate rustc_serialize;

mod error;

use config::reader::from_file;

use std::io::Cursor;
use std::path::Path;
use std::net::SocketAddr;

use ansible::Config;
use rocket::config::Config as RConfig;
use rocket::config::Environment;
use rocket::State;
use rocket::http::Status;
use rocket::response::{self, Response, Responder};

// static mut ADDRESS: Option<SocketAddr> = None;
//
// fn update_handler(req: &mut Request) -> IronResult<Response> {
//     unsafe {
//         ADDRESS = Some(req.remote_addr.clone());
//     }
//
//     Ok(Response::with((status::Ok, "update")))
// }
//
// fn retrieve_handler(req: &mut Request) -> IronResult<Response> {
//     unsafe {
//         Ok(Response::with((status::Ok, format!("{:?}", ADDRESS))))
//     }
// }

struct BroadcastAddr(Option<SocketAddr>);

#[get("/")]
fn index(addr: State<BroadcastAddr>) -> &BroadcastAddr {
    addr.inner()
}

impl<'a> Responder<'a> for BroadcastAddr {
    fn respond(self) -> response::Result<'a> {
        (match self.0 {
            Some(x) => Response::build()
                .sized_body(Cursor::new(x.to_string()))
                .status(Status::Ok),
            None => Response::build()
                .status(Status::NotFound),
        }).ok()
    }
}

fn main() {
    let cfg = Config::load();

    from_file(Path::new("ansible.conf")).expect("Failed to get config file.");
    let rcfg = RConfig::build(Environment::Development)
        .address("0.0.0.0")
        .port(cfg.port)
        .unwrap();

    rocket::custom(rcfg, true)
        .mount("/", routes![index])
        .manage(BroadcastAddr(None))
        .launch();
}
