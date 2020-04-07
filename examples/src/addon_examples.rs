extern crate heroku_rs;
use super::print_response;
use heroku_rs::endpoints::addons;
use heroku_rs::framework::apiclient::HerokuApiClient;

pub fn run<T: HerokuApiClient>(api_client: &T) {
    let addon_id = "123";

    // get_addons(api_client);
    get_addon(api_client, addon_id);
    // get_addon_by_app(api_client, addon_id);
    // get_addon_by_account(api_client);
    // get_addon_by_team(api_client);
    // delete_addon(api_client, addon_id); // Careful here :)
    // update_addon(api_client, addon_id);
    // create_addon(api_client);
    // create_addon_resolution(api_client, addon_id);
}

// Create heroku addon resolution
fn create_addon_resolution<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    // `create` method takes only the required parameters
    // see `new` to pass optional parameters too
    let response = api_client.request(&addons::AddonResolutionCreate::create(addon_id));
    print_response(response);
}

// Create heroku addon
fn create_addon<T: HerokuApiClient>(api_client: &T) {
    let app_id = "123";
    let plan = "heroku-postgresql:dev";
    // `create` method takes only the required parameters
    // see `new` to pass optional parameters too
    let response = api_client.request(&addons::AddonCreate::create(app_id, plan));
    print_response(response);
}

// Update heroku addon
fn update_addon<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let app_id = "123";
    let plan = "heroku-postgresql:dev";
    // `create` method takes only the required parameters
    // see `new` to pass optional parameters too
    let response = api_client.request(&addons::AddonUpdate::create(app_id, addon_id, plan));
    print_response(response);
}

// Delete heroku addon
fn delete_addon<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let app_id = "123";
    let response = api_client.request(&addons::AddonDelete::new(app_id, addon_id));
    print_response(response);
}

// Get heroku addons by team
fn get_addon_by_team<T: HerokuApiClient>(api_client: &T) {
    let team_id = "123";
    let response = api_client.request(&addons::AddonListByTeam::new(team_id));
    print_response(response);
}

// Get heroku addons by account
fn get_addon_by_account<T: HerokuApiClient>(api_client: &T) {
    let account_id = "123";
    let response = api_client.request(&addons::AddonListByAccount::new(account_id));
    print_response(response);
}

// Get heroku addon by app
fn get_addon_by_app<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let app_id = "123";
    let response = api_client.request(&addons::AddonDetailsByApp::new(app_id, addon_id));
    print_response(response);
}

// Get heroku addon
fn get_addon<T: HerokuApiClient>(api_client: &T, addon_id: &str) {
    let response = api_client.request(&addons::AddonDetails::new(addon_id));
    print_response(response);
}

// Get heroku addons
fn get_addons<T: HerokuApiClient>(api_client: &T) {
    let response = api_client.request(&addons::AddonList {});
    print_response(response);
}
