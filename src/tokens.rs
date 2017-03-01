use std::str::FromStr;

pub const PUSH_KEY: &'static str = "X-Ansible-PushToken";
pub const PULL_KEY: &'static str = "X-Ansible-PullToken";

header! { (PushToken, PUSH_KEY) => [String] }
header! { (PullToken, PULL_KEY) => [String] }

impl FromStr for PushToken {
    type Err = ();

    fn from_str(s: &str) -> Result<PushToken, ()> {
        Ok(PushToken(s.to_string()))
    }
}

impl FromStr for PullToken {
    type Err = ();

    fn from_str(s: &str) -> Result<PullToken, ()> {
        Ok(PullToken(s.to_string()))
    }
}
