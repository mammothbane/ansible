use iron::prelude::*;
use iron::middleware::BeforeMiddleware;

use std::marker::{Send, Sync};

use error::ServerError;

pub struct Auth<'a, T> where T: 'a {
    key: &'a T
}

impl<'a, T> Auth<'a, T> {
    fn new(key: T) -> Auth<'a, T> {
        Auth{
            key: key
        }
    }
}

impl<'a, T> BeforeMiddleware for Auth<'a, T> where T: 'a + Send + Sync {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        match req.headers.get::<T>() {
            Ok(x) if x == self.key => Ok(()),
            _ => Err(IronError::new(ServerError("Invalid auth token."), 401))
        }
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<()> {
        Err(err)
    }
}
