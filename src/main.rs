use heroku_rs::client::{Executor, Heroku};
use serde_json::Value;

fn main() {
    println!("Hello, heroku!");
    let client = Heroku::new("API_KEY_HERE").unwrap();
    let me = client.get()
    .apps()
    .app_name("APP_NAME_HERE")
    .execute::<Value>();
    match me {
        Ok((headers, status, json)) => {
            println!("h {:#?}", headers);
            println!("s {}", status);
            if let Some(json) = json {
                println!("{}", json);
            }
        }
        Err(e) => println!("Err {}", e),
    }
}
