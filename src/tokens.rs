use std::str::FromStr;

use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rustc_serialize::hex::FromHex;

#[derive(Clone, Debug)]
pub struct PushToken(Vec<u8>);
#[derive(Clone, Debug)]
pub struct PullToken(Vec<u8>);

impl PartialEq for PushToken {
    fn eq(&self, other: &PushToken) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for PullToken {
    fn eq(&self, other: &PullToken) -> bool {
        self.0 == other.0
    }
}

impl Eq for PushToken {}
impl Eq for PullToken {}

impl<'a, 'r> FromRequest<'a, 'r> for PushToken {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<PushToken, ()> {
        let keys: Vec<_> = req.headers().get("x-ansible-pushtoken").collect();

        match keys.first() {
            Some(x) => match PushToken::from_str(x).ok() {
                Some(tok) => Outcome::Success(tok),
                None => Outcome::Failure((Status::BadRequest, ())),
            },
            _ => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for PullToken {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<PullToken, ()> {
        let keys: Vec<_> = req.headers().get("x-ansible-pulltoken").collect();

        match keys.first() {
            Some(x) => match PullToken::from_str(x).ok() {
                Some(tok) => Outcome::Success(tok),
                None => Outcome::Failure((Status::BadRequest, ())),
            },
            _ => Outcome::Failure((Status::Unauthorized, ())),
        }
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
