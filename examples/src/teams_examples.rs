extern crate heroku_rs;
use super::print_response;
use heroku_rs::endpoints::teams;
use heroku_rs::framework::apiclient::HerokuApiClient;

pub fn run<T: HerokuApiClient>(api_client: &T) {
    // create_team(api_client);
    // create_team_in_enterprise_account(api_client);
    // get_team(api_client);
    get_teams(api_client);
    // get_enterprise_account_teams(api_client);
    // update_team(api_client);
    // delete_team(api_client);    // Careful here :)

    // create_team_app(api_client);
    // get_team_app(api_client);
    // update_team_app(api_client);
    // transfer_team_app(api_client);
    // team_app_list(api_client);

    // team_permissions(api_c   lient);
}

// get team permissions
fn team_permissions<T: HerokuApiClient>(api_client: &T) {
    let response = api_client.request(&teams::TeamAppPermissionList {});
    print_response(response);
}

// get team app list
fn team_app_list<T: HerokuApiClient>(api_client: &T) {
    let team_id = "123";
    let response = api_client.request(&teams::TeamAppList { team_id });
    print_response(response);
}

/// Transfer a team app
/// Owner id can be the account id, in that case it will transfer the app to an account
/// Owner id can be the teamm id, in that case it will transfer the app to a team.
fn transfer_team_app<T: HerokuApiClient>(api_client: &T) {
    let team_id = "123";
    let owner_id = "1234";
    let response = api_client.request(&teams::TeamAppTransfer::new(team_id, owner_id));
    print_response(response);
}

/// Update a team app lock
fn update_team_app<T: HerokuApiClient>(api_client: &T) {
    let team_id = "123";
    let locked = true;

    // verbose

    // let response = api_client.request(&teams::TeamAppUpdateLocked {
    //     team_id,
    //     params: teams::TeamAppUpdateLockedParams { locked },
    // });

    // simple
    let response = api_client.request(&teams::TeamAppUpdateLocked::new(team_id, locked));
    print_response(response);
}

/// Create a new team app
fn update_team<T: HerokuApiClient>(api_client: &T) {
    let team_id = "123";
    // let response = api_client.request(&teams::TeamUpdate {
    //     team_id,
    //     params_optional: None,
    // });

    let response = api_client.request(&teams::TeamUpdate::new(team_id, None, None));
    print_response(response);
}

/// Create a new team app
fn get_team_app<T: HerokuApiClient>(api_client: &T) {
    let app_id = "123";
    let response = api_client.request(&teams::TeamAppDetails { app_id });
    print_response(response);
}

/// Create a new team app
fn create_team_app<T: HerokuApiClient>(api_client: &T) {
    // let response = api_client.request(&teams::TeamAppCreate { params: None });
    let response = api_client.request(&teams::TeamAppCreate::create());
    print_response(response);
}

// delete team
fn delete_team<T: HerokuApiClient>(api_client: &T) {
    let team_id = "123";
    let response = api_client.request(&teams::TeamDelete { team_id });
    print_response(response);
}

// get team list by account id
fn get_enterprise_account_teams<T: HerokuApiClient>(api_client: &T) {
    let account_id = "123";
    let response = api_client.request(&teams::TeamListByEA { account_id });
    print_response(response);
}

// get all teams
fn get_teams<T: HerokuApiClient>(api_client: &T) {
    let response = api_client.request(&teams::TeamList {});
    print_response(response);
}

// get a specific team by id
fn get_team<T: HerokuApiClient>(api_client: &T) {
    let team_id = "123"; // team identifier
    let response = api_client.request(&teams::TeamDetails { team_id });
    print_response(response);
}

/// Create a new team
/// Parameters specified with None will not be sent
fn create_team<T: HerokuApiClient>(api_client: &T) {
    // team name
    let name = "herokursteam2020";

    // let response = api_client.request(&teams::TeamCreate {
    //     params: teams::TeamCreateParams {
    //         name: name,
    //         params_optional: Some(teams::TeamCreateOptionalParams {
    //             address_1: None,
    //             address_2: Some("test"),
    //             card_number: None,
    //             city: None,
    //             country: None,
    //             cvv: None,
    //             expiration_month: None,
    //             expiration_year: None,
    //             first_name: None,
    //             last_name: None,
    //             other: None,
    //             postal_code: None,
    //             state: None,
    //             nonce: None,
    //             device_data: None,
    //         }),
    //     },
    // });

    // `create` method has only the required parameters
    // use `new` method if you want to pass optional parameters
    let response = api_client.request(&teams::TeamCreate::create(name));
    print_response(response);
}

/// Create a new team in an enterprise account
/// This sends `{"name":"herokursteam2020}` to Heroku
fn create_team_in_enterprise_account<T: HerokuApiClient>(api_client: &T) {
    // team name
    let name = "herokursteam2020";
    // account id
    let account_id = "123";

    // let response = api_client.request(&teams::TeamCreateByEA {
    //     account_id,
    //     params: teams::TeamCreateByEAParams { name },
    // });

    let response = api_client.request(&teams::TeamCreateByEA::new(account_id, name));
    print_response(response);
}
