use entity::Event;
use github_rs::client::{Executor, Github};

pub fn fetch_github_user_events(token: String, user: String) {
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