extern crate heroku_rs;

use heroku_rs::endpoints::apps;
use heroku_rs::endpoints::dynos;
use heroku_rs::framework::{
    apiclient::HerokuApiClient,
    auth::Credentials,
    response::{ApiResponse, ApiResult},
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

    // get_app(&api_client);
    // list_apps(&api_client);
    // get_dyno(&api_client);
    // list_dynos(&api_client);
    // restart_dyno(&api_client);
    restart_all_dynos(&api_client);

    Ok(())
}

fn get_app<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let id = String::from("heroku-rs-tests");

    let response = api_client.request(&apps::AppDetails { identifier: id });
    print_response(response);
}

fn list_apps<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let resp = api_client.request(&apps::AppList {});
    print_response(resp);
}

fn get_dyno<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let application_id = String::from("heroku-rs-tests");
    let dyno_id = String::from("web.1");

    let response = api_client.request(&dynos::DynoDetails { app_identifier: application_id, identifier: dyno_id });
    print_response(response);
}

fn list_dynos<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let application_id = String::from("testing-nell-bot");

    let resp = api_client.request(&dynos::DynoList { app_identifier: application_id });
    print_response(resp);
}

fn restart_dyno<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let application_id = String::from("heroku-rs-tests");
    let dyno_id = String::from("web.1");

    let resp = api_client.request(&dynos::DynoRestart { app_identifier: application_id, identifier: dyno_id });
}

fn restart_all_dynos<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let application_id = String::from("heroku-rs-tests");

    let resp = api_client.request(&dynos::DynoAllRestart { app_identifier: application_id });
}

fn print_response<T: ApiResult>(response: ApiResponse<T>) {
    match response {
        Ok(success) => println!("Success: {:#?}", success),
        Err(e) => println!("Error: {}", e),
    }
}
