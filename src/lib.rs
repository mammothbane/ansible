#[macro_use] extern crate hyper;

const PUSH_KEY: &'static str = "X-Ansible-PushToken";
const PULL_KEY: &'static str = "X-Ansible-PullToken";

header! { (PushToken, PUSH_KEY) => [usize] }
header! { (PullToken, PULL_KEY) => [usize] }

pub struct Update {
    elem: usize
}
