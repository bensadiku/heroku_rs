extern crate heroku_rs;
use super::print_response;
use heroku_rs::endpoints::logs;
use heroku_rs::framework::apiclient::HerokuApiClient;

pub fn run<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let app_name = "mycoolherokuappname";

    // create_log_drain(api_client, app_name);
    get_log_drains(api_client, app_name);
    // get_log_drain(api_client, app_name);
    // get_log_drains_by_addon(api_client);
    // update_addon_log_drain(api_client);
    // delete_log_drain(api_client, app_name); // Careful here :)

    // create_log_session(api_client, app_name);
}

// creat log session
fn create_log_session<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    // `create` method takes only the required parameters
    // see `new` to pass optional parameters too
    let response = api_client.request(&logs::LogSessionCreate::create(app_id));
    print_response(response);
}

// delete log drain
fn delete_log_drain<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let drain_id = "DRAIN_ID_URL_OR_TOKEN";
    let response = api_client.request(&logs::LogDrainDelete::new(app_id, drain_id));
    print_response(response);
}

// get a list of log drains by addon id
fn get_log_drains_by_addon<T: HerokuApiClient>(api_client: &T) {
    let addon_id = "ADD_ON_ID";
    let response = api_client.request(&logs::LogDrainListByAddon::new(addon_id));
    print_response(response);
}

// update a add-on owned log drain
fn update_addon_log_drain<T: HerokuApiClient>(api_client: &T) {
    let drain_id = "DRAIN_ID_URL_OR_TOKEN";
    let addon_id = "ADD_ON_ID";
    let url = "https://mycoolherokuappname.herokuapp.com/";
    let response = api_client.request(&logs::LogDrainUpdate::new(addon_id, drain_id, url));
    print_response(response);
}

// get info about a specific log drain
fn get_log_drain<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let drain_id = "DRAIN_ID_URL_OR_TOKEN";
    let response = api_client.request(&logs::LogDrainDetails::new(app_id, drain_id));
    print_response(response);
}

// get a list of log drain
fn get_log_drains<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let response = api_client.request(&logs::LogDrainList::new(app_id));
    print_response(response);
}

// create a new log drain
fn create_log_drain<T: HerokuApiClient>(api_client: &T, app_id: &str) {
    let url = "https://mycoolherokuappname.herokuapp.com/";
    let response = api_client.request(&logs::LogDrainCreate::new(app_id, url));
    print_response(response);
}
