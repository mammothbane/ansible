extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate config;

use iron::prelude::*;
use iron::status;
use router::Router;
use config::reader::from_file;

use self::{PushToken, PullToken};

use std::path::Path;
use std::net::SocketAddr;

static mut ADDRESS: Option<SocketAddr> = None;

fn update_handler(req: &mut Request) -> IronResult<Response> {
    let auth_header = req.headers.get<PushToken>();
    let body = req.get::<bodyparser::Json>();
    println!("{:?}", body);
    Ok(Response::with((status::Ok, "post")))
}

fn retrieve_handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "repsonse")))
}

fn main() {
    let cfg = from_file(Path::new("ansible.conf")).unwrap();
    let sock_str = format!("0.0.0.0:{}", cfg.lookup_integer32("port").unwrap());
    let mut router = Router::new();

    router.post("/update", update_handler, "update ip");
    router.get("/", retrieve_handler, "ip response");

    Iron::new(router).http(&sock_str[..]).unwrap();
}
