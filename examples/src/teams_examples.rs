extern crate heroku_rs;
use super::print_response;
use heroku_rs::endpoints::teams;
use heroku_rs::framework::apiclient::HerokuApiClient;

pub fn run<T: HerokuApiClient>(api_client: &T) {
    // create_team(api_client);
    // create_team_in_enterprise_account(api_client);
    get_team(api_client);
    // get_teams(api_client);
    // get_enterprise_account_teams(api_client);
    // update_team(api_client);
    // delete_team(api_client);    // Careful here :)

    // create_team_app(api_client);
    // get_team_app(api_client);
    // update_team_app(api_client);
    // transfer_team_app(api_client);
    // team_app_list(api_client);

    // team_permissions(api_client);

    // get_team_features(api_client);
    // get_team_feature(api_client);

    // get_team_invitations(api_client);
    // create_team_invitation(api_client);
    // revoke_team_invitation(api_client); // Careful here :)
    // get_team_invitation(api_client);
    // accept_team_invitation(api_client);

    // team_invoice_list(api_client);
    // team_invoice(api_client);

    // team_member_create_or_update(api_client);
    // team_member_create(api_client);
    // team_member_update(api_client);
    // team_member_delete(api_client); // Careful here :)
    // get_team_member_list(api_client);
    // get_team_member_app_list(api_client);

    // get_team_preferences(api_client);
    // update_team_preferences(api_client);
}

// update team preferences
fn update_team_preferences<T: HerokuApiClient>(api_client: &T) {
    let id = "123";
    let response = api_client.request(
        &teams::TeamPreferenceUpdate::new(id)
            .whitelisting_enabled(true)
            .build(),
    );
    print_response(response);
}

// get team preferences
fn get_team_preferences<T: HerokuApiClient>(api_client: &T) {
    let id = "123";
    let response = api_client.request(&teams::TeamPreferenceList::new(id));
    print_response(response);
}

// get team member apps
fn get_team_member_app_list<T: HerokuApiClient>(api_client: &T) {
    let member_id = "123@example.de";
    let team_id = "123";
    let response = api_client.request(&teams::TeamMemberAppsList::new(team_id, member_id));
    print_response(response);
}

// get team member list
fn get_team_member_list<T: HerokuApiClient>(api_client: &T) {
    let team_id = "123";
    let response = api_client.request(&teams::TeamMemberList::new(team_id));
    print_response(response);
}

// delete a team member
fn team_member_delete<T: HerokuApiClient>(api_client: &T) {
    let member_id = "123@example.de";
    let team_id = "123";
    let response = api_client.request(&teams::TeamMemberDelete::new(team_id, member_id));
    print_response(response);
}

// update a team member
fn team_member_update<T: HerokuApiClient>(api_client: &T) {
    let email = "123@example.de";
    let role = "admin";
    let team_id = "123";
    let response = api_client.request(
        &teams::TeamMemberUpdate::new(team_id, email, role)
            .federated(false)
            .build(),
    );
    print_response(response);
}

// create a team member
fn team_member_create<T: HerokuApiClient>(api_client: &T) {
    let email = "123@example.de";
    let role = "admin";
    let team_id = "123";
    let response = api_client.request(&teams::TeamMemberCreate::new(team_id, email, role));
    print_response(response);
}

// create or update a team member
fn team_member_create_or_update<T: HerokuApiClient>(api_client: &T) {
    let email = "123@example.de";
    let role = "admin";
    let team_id = "123";
    let response = api_client.request(&teams::TeamMemberCreateorUpdate::new(team_id, email, role));
    print_response(response);
}

// get a specific team invoice
fn team_invoice<T: HerokuApiClient>(api_client: &T) {
    let invoice_id = "123";
    let team_id = "123";
    let response = api_client.request(&teams::TeamInvoiceDetails {
        team_id,
        invoice_id,
    });
    print_response(response);
}

// get a list of existing invoices
fn team_invoice_list<T: HerokuApiClient>(api_client: &T) {
    let team_id = "123";
    let response = api_client.request(&teams::TeamInvoiceList { team_id });
    print_response(response);
}

// accept team invitation
fn accept_team_invitation<T: HerokuApiClient>(api_client: &T) {
    let token_id = "123";
    let response = api_client.request(&teams::TeamInvitationAccept { token_id });
    print_response(response);
}

// get team invitation
fn get_team_invitation<T: HerokuApiClient>(api_client: &T) {
    let token_id = "123";
    let response = api_client.request(&teams::TeamInvitationDetails { token_id });
    print_response(response);
}

// revoke team invitation
fn revoke_team_invitation<T: HerokuApiClient>(api_client: &T) {
    let team_id = "123";
    let invitation_id = "123";
    let response = api_client.request(&teams::TeamInvitationRevoke::new(team_id, invitation_id));
    print_response(response);
}

// create team invitations
fn create_team_invitation<T: HerokuApiClient>(api_client: &T) {
    let team_id = "123";
    let email = "name@gmail.com";
    let response = api_client.request(
        &teams::TeamInvitationCreate::new(team_id, email)
            .role("member")
            .build(),
    );
    print_response(response);
}

// get team invitations
fn get_team_invitations<T: HerokuApiClient>(api_client: &T) {
    let team_id = "123";
    let response = api_client.request(&teams::TeamInvitationList { team_id });
    print_response(response);
}

// get team feature
fn get_team_feature<T: HerokuApiClient>(api_client: &T) {
    let team_id = "123";
    let feature_id = "123";
    let response = api_client.request(&teams::TeamFeatureDetails {
        team_id,
        feature_id,
    });
    print_response(response);
}

// get team features
fn get_team_features<T: HerokuApiClient>(api_client: &T) {
    let team_id = "123";
    let response = api_client.request(&teams::TeamFeatureList { team_id });
    print_response(response);
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
    let response = api_client.request(
        &teams::TeamUpdate::new(team_id)
            .default(false)
            .name("new-team-name")
            .build(),
    );
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
    let response = api_client.request(
        &teams::TeamAppCreate::new()
            .name("team-app")
            .locked(true)
            .build(),
    );
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
fn create_team<T: HerokuApiClient>(api_client: &T) {

    // `new` method takes only the required parameters and gives access to the builder pattern
    //address_2 is optional, but showcasing builder pattern
    let response = api_client.request(&teams::TeamCreate::new("herokursteam2020").address_2("test").build());
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
