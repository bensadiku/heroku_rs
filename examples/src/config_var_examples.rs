extern crate heroku_rs;
use super::print_response;
use heroku_rs::endpoints::config_vars;
use heroku_rs::framework::apiclient::HerokuApiClient;
use std::collections::HashMap;

pub fn run<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let app_name = "heroku-rs-tests";
    get_app_config_vars(api_client, app_name);
    // get_release_config_vars(api_client, app_name);
    // update_app_config_vars(api_client, app_name);
    // delete_app_config_vars(api_client, app_name);

    // get_pipeline_config_vars(api_client);
    // update_pipeline_config_vars(api_client);
    // delete_pipeline_config_vars(api_client);
}

// delete config vars for a pipeline
fn delete_pipeline_config_vars<T: HerokuApiClient>(api_client: &T) {
    let pipeline_id = "PIPELINE_ID";
    let stage_id = "test";

    let mut cvar = HashMap::new();
    let cvar_key = String::from("config_var_key"); // config var value key
    let cvar_value = None; // config var value to delete

    cvar.insert(cvar_key, cvar_value);

    let response = api_client.request(&config_vars::PipelineConfigVarDelete {
        pipeline_id,
        stage_id,
        params: cvar,
    });
    print_response(response);
}

// update config vars for a pipeline
fn update_pipeline_config_vars<T: HerokuApiClient>(api_client: &T) {
    let pipeline_id = "PIPELINE_ID";
    let stage_id = "test";

    let mut cvar = HashMap::new();
    let cvar_key = String::from("config_var_key"); // config var value key
    let cvar_value = String::from("updated_config_var_value"); // config var value to update

    cvar.insert(cvar_key, cvar_value);

    let response = api_client.request(&config_vars::PipelineConfigVarUpdate::new(
        pipeline_id,
        stage_id,
        cvar,
    ));
    print_response(response);
}

// get config vars for a pipeline
fn get_pipeline_config_vars<T: HerokuApiClient>(api_client: &T) {
    let pipeline_id = "PIPELINE_ID";
    let stage_id = "test";
    let response = api_client.request(&config_vars::PipelineConfigVarDetails {
        pipeline_id,
        stage_id,
    });
    print_response(response);
}

// delete config vars for an app
fn delete_app_config_vars<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let mut cvar = HashMap::new();
    // config var value key
    let cvar_key = String::from("config_var_key");
    // This parameter is Option because it is used as a `DELETE` request.
    // If you pass `None` it will actually delete the config var.
    let cvar_value = None;
    cvar.insert(cvar_key, cvar_value);

    let response = api_client.request(&config_vars::AppConfigVarDelete {
        app_id,
        params: cvar,
    });
    print_response(response);
}

// update config vars for an app
fn update_app_config_vars<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let mut cvar = HashMap::new();
    let cvar_key = String::from("config_var_key"); // config var value key
    let cvar_value = String::from("updated_config_var_value"); // config var value to update

    cvar.insert(cvar_key, cvar_value);

    let response = api_client.request(&config_vars::AppConfigVarUpdate {
        app_id,
        params: cvar,
    });
    print_response(response);
}

// get app release config vars
fn get_release_config_vars<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    // release version or id
    let release_id = "1";
    let response = api_client.request(&config_vars::ReleaseConfigVarDetails { app_id, release_id });
    print_response(response);
}

// get config vars for an app
fn get_app_config_vars<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let response = api_client.request(&config_vars::AppConfigVarDetails { app_id });
    print_response(response);
}
