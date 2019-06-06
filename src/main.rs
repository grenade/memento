extern crate chrono;
extern crate github_rs;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use chrono::{DateTime, Utc};
use github_rs::client::{Executor, Github};
//use serde_json::Value;
use std::env;

#[derive(Deserialize, Debug)]
struct Actor {
    avatar_url: String,
    display_login: String,
    gravatar_id: String,
    id: u32,
    login: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct Author {
    email: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Commit {
    author: Author,
    message: String,
    sha: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct Event {
    actor: Actor,
    created_at: DateTime<Utc>,
    id: String,
    org: Option<Org>,
    payload: Payload,
    #[serde(rename = "type")]
    action: String,
    repo: Repo,
}

#[derive(Deserialize, Debug)]
struct Org {
    avatar_url: String,
    gravatar_id: String,
    id: u32,
    login: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct Payload {
    descripttion: Option<String>,
    master_branch: Option<String>,
    #[serde(rename = "ref")]
    git_ref: Option<String>,
    ref_type: Option<String>,
    commits: Option<Vec<Commit>>,
}

#[derive(Deserialize, Debug)]
struct Repo {
    id: u32,
    name: String,
    url: String,
}

// usage:
// cargo run -- $(pass github/grenade/token/memento-user-read) grenade
fn main() {
    let args: Vec<String> = env::args().collect();
    let token = &args[1];
    let user = &args[2];

    let client = Github::new(token).unwrap();
    let endpoint = format!("users/{}/events", user);
    let response = client.get().custom_endpoint(&endpoint).execute::<Vec<Event>>();
    match response {
        Ok((headers, status, events)) => {
            println!("{:#?}", headers);
            println!("{}", status);
            

            for event in events.unwrap() {
                println!("{:?} {}\n", event.created_at, event.action);
            }
        },
        Err(e) => println!("{}", e)
    }
}