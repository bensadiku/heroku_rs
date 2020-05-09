extern crate heroku_rs;
use super::print_response;
use heroku_rs::endpoints::misc;
use heroku_rs::framework::apiclient::HerokuApiClient;

pub fn run<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    // get_regions(api_client);
    // get_region(api_client);

    get_rate_limit(api_client);

    // get_stack(api_client);
    // get_stacks(api_client);

    // create_sources(api_client);
}

/// Create sources
fn create_sources<T: HerokuApiClient>(api_client: &T) {
    let response = api_client.request(&misc::SourceCreate {});
    print_response(response);
}

/// Get a specific stack
fn get_stack<T: HerokuApiClient>(api_client: &T) {
    let stack_id = "69bee368-352b-4bd0-9b7c-819d860a2588"; // heroku-18 stack
    let response = api_client.request(&misc::StackDetails { stack_id });
    print_response(response);
}

/// Get stacks
fn get_stacks<T: HerokuApiClient>(api_client: &T) {
    let response = api_client.request(&misc::StackList {});
    print_response(response);
}

/// Get rate limit
fn get_rate_limit<T: HerokuApiClient>(api_client: &T) {
    let response = api_client.request(&misc::RatelimitDetails {});
    print_response(response);
}

/// Get regions
fn get_regions<T: HerokuApiClient>(api_client: &T) {
    let response = api_client.request(&misc::RegionList {});
    print_response(response);
}

/// Get specific region
fn get_region<T: HerokuApiClient>(api_client: &T) {
    let region_id = "6f2b2ec9-b087-4976-8ec9-5d2f62276aeb"; // Dublin - Ireland
    let response = api_client.request(&misc::RegionDetails { region_id });
    print_response(response);
}
