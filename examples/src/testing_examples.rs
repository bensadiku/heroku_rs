extern crate heroku_rs;
use super::print_response;
use heroku_rs::endpoints::testing;
use heroku_rs::framework::apiclient::HerokuApiClient;

pub fn run<T: HerokuApiClient>(api_client: &T) {
    // create_test_run(api_client);
    get_test_case_list(api_client);
    // get_test_node_list(api_client);

    // get_test_run(api_client);
    // get_test_run_list(api_client);
    // get_test_run_by_pipeline(api_client);
    // update_test_run(api_client);
}

fn update_test_run<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let message = "message id";
    let status = "succeeded";
    let run_id = "id";
    let response = api_client.request(&testing::TestRunUpdate::new(run_id, Some(message), status));
    print_response(response);
}

fn get_test_run_by_pipeline<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let test_case_pipeline_id = "id";
    let test_case_run_id = "id";
    let response = api_client.request(&testing::TestRunDetailsByPipeline::new(
        test_case_pipeline_id,
        test_case_run_id,
    ));
    print_response(response);
}

fn get_test_run_list<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let test_case_pipeline_id = "pipline id";
    let response = api_client.request(&testing::TestRunList::new(test_case_pipeline_id));
    print_response(response);
}

fn get_test_run<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let test_case_run_id = "id";
    let response = api_client.request(&testing::TestRunDetails::new(test_case_run_id));
    print_response(response);
}

fn get_test_node_list<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let test_case_run_id = "b226996a-c2f0-44eb-ab5d-c1fa252bb0ed";
    let response = api_client.request(&testing::TestNodeList::new(test_case_run_id));
    print_response(response);
}

fn get_test_case_list<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let test_case_run_id = "id";
    let response = api_client.request(&testing::TestCaseList::new(test_case_run_id));
    print_response(response);
}

fn create_test_run<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let commit_branch = "master";
    let commit_message = "the message for the commit under test";
    let commit_sha = "SHA";
    let pipeline = "97241b2c-e214-4842-a633-52e7f3ddd17c";
    let source_blob_url = "https://s3-external-1.amazonaws.com/heroku-slugs-us/2dbc/2dbce013-4be8-44e1-8221-c9c74e45949c.tar.gz?AWSAccessKeyId=AKIAIO4SD3DCRO7W6IJQ&Signature=pTvmgwH0T7irx2q6CQqDl7wxcag%3D&Expires=1585394704";
    let response = api_client.request(&testing::TestRunCreate::new(
        commit_branch,
        commit_message,
        commit_sha,
        pipeline,
        source_blob_url,
    ));
    print_response(response);
}
