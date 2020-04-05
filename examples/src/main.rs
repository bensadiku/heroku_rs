#![allow(dead_code)] //Some methods may not be "used" because they're showcase examples and not called all at once
extern crate heroku_rs;
use dotenv;
use heroku_rs::framework::{
    auth::Credentials,
    response::{ApiResponse, ApiResult},
    ApiEnvironment, HttpApiClient, HttpApiClientConfig,
};

mod account_examples;
mod apps_examples;
mod misc_examples;
mod oauth_examples;
mod pipeline_examples;
mod app_review_examples;
mod config_var_examples;
mod custom_examples;
mod teams_examples;
mod collaborators_examples;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = "API_KEY";
    let token = dotenv::var(key).unwrap();

    // This will use the default headers & timeout for the HttpApiClientConfig
    // and will use the default production heroku endpoint
    let api_client = HttpApiClient::create(&token)?;

    // If you want to costumize the HttpApiClientConfig and ApiEnvironment,
    // see the create_custom_client and uncomment this to use it
    // let api_client = create_custom_client(token)?;

    apps_examples::run(&api_client);
    // collaborators_examples::run(&api_client);
    // custom_examples::run(&api_client);
    // app_review_examples::run(&api_client);
    // config_var_examples::run(&api_client);
    // account_examples::run(&api_client);
    // oauth_examples::run(&api_client);
    // pipeline_examples::run(&api_client);
    // misc_examples::run(&api_client);
    // teams_examples::run(&api_client);

    Ok(())
}

fn create_custom_client(token: String) -> Result<HttpApiClient, Box<dyn std::error::Error>> {
    let credentials = Credentials::UserAuthToken {
        token: token.to_string(),
    };
    let api_client = HttpApiClient::new(
        credentials,
        HttpApiClientConfig::default(),
        ApiEnvironment::Production,
    )?;

    Ok(api_client)
}

fn print_response<T: ApiResult>(response: ApiResponse<T>) {
    match response {
        Ok(success) => println!("Success: {:#?}", success),
        Err(e) => println!("Error: {}", e),
    }
}
