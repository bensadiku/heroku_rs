use heroku_rs::client::{Executor, Heroku};
use serde_json::Value;

pub fn run() {
    get_apps();
    post_apps();
}

fn get_apps() {
    let client = Heroku::new("API_KEY").unwrap();
    let me = client.get()
    .apps() // gets all apps in this account
    .app_name("APP_NAME") // get a specific app by name OR can by id: app_id("APP_ID")
    .app_features() // gets all features in that app
    .feature_name("web-auto-scaling") // get a specific feature by name OR can by id: feature_id("47d1998e-f8f4-432d-b4cc-f105f4d76407")
    .execute::<Value>();
    match me {
        Ok((headers, status, json)) => {
            println!("Headers: {:#?}", headers);
            println!("Status: {}", status);
            if let Some(json) = json {
                println!("Response: {}", json);
            }
        }
        Err(e) => println!("Err {}", e),
    }
}

fn post_apps() {
    //TODO
}