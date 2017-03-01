use addr::Addr;

use iron::prelude::*;
use iron::status;

use persistent::State;

pub fn index(r: &mut Request) -> IronResult<Response> {
    let st = r.get::<State<Addr>>().unwrap();
    let addr = st.read().unwrap();

    match addr.address() {
        Some(addr) => Ok(Response::with((status::Ok, format!("{:?}", addr)))),
        None => Ok(Response::with((status::Ok, "no remote address present"))),
    }
}

pub fn update(r: &mut Request) -> IronResult<Response> {
    let mutex = r.get::<State<Addr>>().unwrap();
    let mut addr = mutex.write().unwrap();

    println!("updating with {:?}", r.remote_addr);

    (*addr).update(r.remote_addr);
    Ok(Response::with((status::Ok, "updated!")))
}
