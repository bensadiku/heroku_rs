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

    // enable_app_review_configuration(api_client);
    // get_app_review_configuration(api_client);
    // update_app_review_configuration(api_client);
    // delete_app_review_configuration(api_client); // Careful here :)
}

// delete app review config by pipeline id
fn delete_app_review_configuration<T: HerokuApiClient>(api_client: &T) {
    let pipeline_id = "PIPELINE_ID";
    let response = api_client.request(&review::ReviewAppConfigDelete::new(pipeline_id));
    print_response(response);
}

// Update app review config
fn update_app_review_configuration<T: HerokuApiClient>(api_client: &T) {
    let pipeline_id = "PIPELINE_ID";
    let response = api_client.request(
        &review::ReviewAppConfigUpdate::new(pipeline_id)
            .automatic_review_apps(true)
            .base_name("cool-base-name-yo")
            .build(),
    );
    print_response(response);
}

// get app review config by pipeline id
fn get_app_review_configuration<T: HerokuApiClient>(api_client: &T) {
    let pipeline_id = "PIPELINE_ID";
    let response = api_client.request(&review::ReviewAppConfigDetails::new(pipeline_id));
    print_response(response);
}

// Enable app review config
fn enable_app_review_configuration<T: HerokuApiClient>(api_client: &T) {
    let pipeline_id = "PIPELINE_ID";
    let repo = "heroku/heroku-rs-test";
    // `new` method takes only the required parameters
    let response = api_client.request(
        &review::ReviewAppConfigEnable::new(pipeline_id, repo)
            .base_name("singular-app")
            .automatic_review_apps(true)
            .build(),
    );
    print_response(response);
}

// Delete app review list by review_id
fn delete_app_review<T: HerokuApiClient>(api_client: &T) {
    let review_id = "REVIEW_ID";
    let response = api_client.request(&review::ReviewAppDelete { review_id });
    print_response(response);
}

// Get app review list by pipeline_id
fn get_app_review_list_by_pipeline<T: HerokuApiClient>(api_client: &T) {
    let pipeline_id = "PIPELINE_ID";
    let response = api_client.request(&review::ReviewAppByPipelineList { pipeline_id });
    print_response(response);
}

// Get app review by app_id
fn get_app_review_by_app<T: HerokuApiClient>(api_client: &T) {
    let app_id = "APP_ID";
    let response = api_client.request(&review::ReviewAppByAppDetails { app_id });
    print_response(response);
}

// Get app review
fn get_app_review<T: HerokuApiClient>(api_client: &T) {
    let review_id = "REVIEW_ID";
    let response = api_client.request(&review::ReviewAppDetails { review_id });
    print_response(response);
}

// Create app review
fn create_app_review<T: HerokuApiClient>(api_client: &T) {
    let branch = "master";
    let pipeline = "PIPELINE_ID";
    let source_blob_url = "https://nodejs.org/dist/v0.10.20/node-v0.10.20-linux-x64.tar.gz";
    let response = api_client.request(
        &review::ReviewAppCreate::new(branch, pipeline, source_blob_url)
            .fork_repo_id(123)
            .build(),
    );
    print_response(response);
}
