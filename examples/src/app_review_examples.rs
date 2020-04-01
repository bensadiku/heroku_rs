extern crate heroku_rs;
use super::print_response;

use heroku_rs::endpoints::review;
use heroku_rs::framework::apiclient::HerokuApiClient;

pub fn run<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    // create_app_review(api_client);
    get_app_review(api_client);
    // delete_app_review(api_client);   // Careful here :)
    // get_app_review_by_app(api_client)
    // get_app_review_list_by_pipeline(api_client)
}


// Delete app review list by review_id
fn delete_app_review<T: HerokuApiClient>(api_client: &T) {
    let review_id = String::from("REVIEW_ID");
    let response = api_client.request(&review::ReviewAppDelete { review_id });
    print_response(response);
}

// Get app review list by pipeline_id
fn get_app_review_list_by_pipeline<T: HerokuApiClient>(api_client: &T) {
    let pipeline_id = String::from("PIPELINE_ID");
    let response = api_client.request(&review::ReviewAppByPipelineList { pipeline_id });
    print_response(response);
}

// Get app review by app_id
fn get_app_review_by_app<T: HerokuApiClient>(api_client: &T) {
    let app_id = String::from("APP_ID");
    let response = api_client.request(&review::ReviewAppByAppDetails { app_id });
    print_response(response);
}

// Get app review
fn get_app_review<T: HerokuApiClient>(api_client: &T) {
    let review_id = String::from("REVIEW_ID");
    let response = api_client.request(&review::ReviewAppDetails { review_id });
    print_response(response);
}

// Create app review
fn create_app_review<T: HerokuApiClient>(api_client: &T) {
    let branch = String::from("master");
    let pipeline = String::from("PIPELINE_ID");
    let source_blob_url =
        String::from("https://nodejs.org/dist/v0.10.20/node-v0.10.20-linux-x64.tar.gz");
    let response = api_client.request(&review::ReviewAppCreate {
        params: review::ReviewAppCreateParams::new(
            branch,
            pipeline,
            source_blob_url,
            None,
            None,
            None,
        ),
    });
    print_response(response);
}
