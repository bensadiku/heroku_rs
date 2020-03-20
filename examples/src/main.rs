#![allow(dead_code)] //Some methods may not be "used" because they're showcase examples and not called all at once
extern crate heroku_rs;
use heroku_rs::framework::{
    auth::Credentials,
    response::{ApiResponse, ApiResult},
    ApiEnvironment, HttpApiClient, HttpApiClientConfig,
};
use dotenv;

mod apps_examples;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = "API_KEY";
    let token = dotenv::var(key).unwrap();

    let credentials = Credentials::UserAuthToken {
        token: token.to_string(),
    };
    let api_client = HttpApiClient::new(
        credentials,
        HttpApiClientConfig::default(),
        ApiEnvironment::Production,
    )?;

    apps_examples::run(&api_client);

    Ok(())
}

fn print_response<T: ApiResult>(response: ApiResponse<T>) {
    match response {
        Ok(success) => println!("Success: {:#?}", success),
        Err(e) => println!("Error: {}", e),
    }
}
