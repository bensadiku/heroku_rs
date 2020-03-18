extern crate heroku_rs;

use heroku_rs::endpoints::apps;
use heroku_rs::framework::{
    apiclient::HerokuApiClient,
    auth::Credentials,
    response::{ApiResult, ApiResponse},
    ApiEnvironment, HttpApiClient, HttpApiClientConfig,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = Some("API_KEY_TOKEN");

    let credentials: Credentials = if let Some(token) = token {
        Credentials::UserAuthToken {
            token: token.to_string(),
        }
    } else {
        panic!("No token provided")
    };

    let api_client = HttpApiClient::new(
        credentials,
        HttpApiClientConfig::default(),
        ApiEnvironment::Production,
    )?;

    list_apps(&api_client);

    Ok(())
}

fn list_apps<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let id = String::from("heroku-rs-tests");

    let response = api_client.request(&apps::AppDetails { identifier: id });
    print_response(response);
}

fn print_response<T: ApiResult>(response: ApiResponse<T>) {
    match response {
        Ok(success) => println!("Success: {:#?}", success),
        Err(e) => println!("Error: {}", e),
    }
}
