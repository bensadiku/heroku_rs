use heroku_rs::client::Heroku;

mod apps;
mod teams;

fn main() {
    println!("Hello, heroku!");
    let client = Heroku::new("API_KEY").unwrap();
    apps::run(client);
    // teams::run(client);
}
