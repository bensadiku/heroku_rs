extern crate heroku_rs;
use super::print_response;
use heroku_rs::endpoints::custom;
use heroku_rs::framework::apiclient::HerokuApiClient;
use heroku_rs::framework::endpoint::Method;
use serde::Serialize;

pub fn run<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let app_name = String::from("heroku-rs-tests");

    get_app_custom(api_client, app_name);
    // create_app_custom(api_client, app_name);
    // delete_app_custom(api_client, app_name); //Careful here :)
}

// Delete an app
fn delete_app_custom<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let query = format!("{}{}", "apps/", app_id);
    let method = Method::Delete;
    let response = api_client.request(&custom::CustomEndpointSimple {
        query: query,
        method: method,
    });
    print_response(response);
}

// get info about a slug
fn create_app_custom<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let query = String::from("apps");
    let param = AppCreateParam {
        name: Some(app_id),
        region: None,
        stack: None,
    };
    let method = Method::Post;
    let response = api_client.request(&custom::CustomEndpoint {
        query: query,
        method: method,
        params: param,
    });
    print_response(response);
}

#[derive(Serialize, Clone, Debug)]
pub struct AppCreateParam {
    /// name of app. pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
    pub name: Option<String>,
    /// unique identifier or name of region
    pub region: Option<String>,
    /// unique name or identifier of stack
    pub stack: Option<String>,
}

// get info about a app
fn get_app_custom<T: HerokuApiClient>(api_client: &T, app_id: String) {
    let query = format!("{}{}", "apps/", app_id);
    let method = Method::Get;
    let response = api_client.request(&custom::CustomEndpointSimple::new(query, method));
    print_response(response);
}
