extern crate chrono;
extern crate github_rs;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
mod action;
mod entity;
use std::env;

// usage:
// cargo run -- $(pass github/grenade/token/memento-user-read) grenade
fn main() {
    action::fetch_github_user_events(
        env::args().nth(1).unwrap(),
        env::args().nth(2).unwrap())
}