extern crate heroku_rs;
use super::print_response;
use heroku_rs::endpoints::account;
use heroku_rs::framework::apiclient::HerokuApiClient;

pub fn run<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    // get_account(api_client);
    // patch_account(api_client);
    // delete_account(api_client); //Careful here :)
    // get_user_account(api_client);
    // patch_user_account(api_client);
    // delete_user_account(api_client); //Careful here :)
}

// Delete heroku user account. NOTE that this action cannot be undone.
fn delete_user_account<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let account_id = String::from("USER_ID_OR_EMAIL");

    let response = api_client.request(&account::UserAccountDelete {
        account_identifier: account_id,
    });
    print_response(response);
}

// Patch heroku user account
fn patch_user_account<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let account_id = String::from("USER_ID_OR_EMAIL");
    let response = api_client.request(&account::UserAccountUpdate {
        account_identifier: account_id,
        params: account::UserAccountUpdateParams {
            allow_tracking: Some(true),
            beta: Some(false),
            name: Some(String::from("Heroku-testing")),
        },
    });
    print_response(response);
}

// Get heroku user account by email or id
fn get_user_account<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let account_id = String::from("USER_ID_OR_EMAIL");
    let response = api_client.request(&account::UserAccountDetails {
        account_identifier: account_id,
    });
    print_response(response);
}

// Delete heroku account. NOTE that this action cannot be undone.
fn delete_account<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AccountDelete {});
    print_response(response);
}

// Patch heroku account
fn patch_account<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AccountUpdate {
        params: account::AccountUpdateParams {
            allow_tracking: Some(true),
            beta: Some(false),
            name: Some(String::from("Heroku-test")),
        },
    });
    print_response(response);
}

// Get heroku account
fn get_account<ApiClientType: HerokuApiClient>(api_client: &ApiClientType) {
    let response = api_client.request(&account::AccountDetails {});
    print_response(response);
}
