use std::str::FromStr;

use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};
use rustc_serialize::hex::FromHex;

pub struct PushToken(Vec<u8>);
pub struct PullToken(Vec<u8>);

impl<'a, 'r> FromRequest<'a, 'r> for PushToken {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<PushToken, ()> {
        let keys: Vec<_> = req.headers().get("x-ansible-pushtoken").collect();
        Outcome::Success(PushToken::from_str(keys.first().unwrap()).unwrap())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for PullToken {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<PullToken, ()> {
        let keys: Vec<_> = req.headers().get("x-ansible-pulltoken").collect();
        Outcome::Success(PullToken::from_str(keys.first().unwrap()).unwrap())
    }
}


impl FromStr for PushToken {
    type Err = ();

    fn from_str(s: &str) -> Result<PushToken, ()> {
        let push_key = s.from_hex().unwrap();
        assert!(push_key.len() >= 8, "Push key too short!");

        Ok(PushToken(push_key))
    }
}

impl FromStr for PullToken {
    type Err = ();

    fn from_str(s: &str) -> Result<PullToken, ()> {
        let pull_key = s.from_hex().unwrap();
        assert!(pull_key.len() >= 8, "Pull key too short!");

        Ok(PullToken(pull_key))
    }
}
