#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate config;
extern crate ansible;
extern crate rustc_serialize;

mod error;

use config::reader::from_file;
use rustc_serialize::hex::FromHex;

use std::path::Path;
use std::net::SocketAddr;

use error::ServerError;

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

#[get("/")]
fn index() -> &'static str {
    "HELLO"
}


fn main() {
    let cfg = from_file(Path::new("ansible.conf")).expect("Failed to get config file.");
    let sock_str = format!("0.0.0.0:{}", cfg.lookup_integer32("port").unwrap());

    let push_token = cfg.lookup_str("push_secret").unwrap().to_owned();
    let pull_token = cfg.lookup_str("pull_secret").unwrap().to_owned();

    rocket::ignite().mount("/", routes![index]).launch();
}
