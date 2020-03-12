#![allow(dead_code)] // Just warns about un-used methods until they're used.
use heroku_rs::client::{Executor, Heroku};
use heroku_rs::defaults::{CreateTeamApp, CreateTeamIdentityProvider, CreateTeamMember, CreateTeamAppCollaborator};
use heroku_rs::errors::Error;
use heroku_rs::{HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
// Uncomment methods to run them.
pub fn run(client: Heroku) {
    // let team_name = "herokuexample";
    // let team_app_name = "heroku-rs-tests";

    get_teams(&client);
    // post_team(&client, team_name);
    // get_team_addons(&client, team_name);
    // patch_team(&client, team_name);
    // put_teams_invitation(&client, team_name);
    // delete_team(&client, team_name);
    // delete_team_member(&client, team_name);
    // delete_team_invitation(&client, team_name);
    // post_team_member(&client, team_name);
    // post_team_apps(&client);
    // post_team_app_collaborators(&client,team_app_name);
    // post_team_addon_service(&client, team_name);
    // post_team_identity_provider(&client, team_name);
    // post_team_invitation_accept(&client);
}

// == GET teams  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team
// Requires the Heroku client
// get a specific team by name: .get().teams().team_name("NAME_HERE")
//  or by id: .get().teams().formation_type("TYPE_HERE")
fn get_teams(client: &Heroku) {
    let me = client.get().teams().execute::<Value>();

    log_response(me);
}

// == GET team addons  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#add-on-list-by-team
// Requires the Heroku client
fn get_team_addons(client: &Heroku, team_name: &str) {
    let me = client
        .get()
        .teams()
        .team_name(team_name)
        .team_addons()
        .execute::<Value>();

    log_response(me);
}

// == POST team  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-team-create-1
// Requires the Heroku client
fn post_team(client: &Heroku, team_name: &str) {
    #[derive(Serialize, Deserialize, Debug)]
    struct Team {
        name: String,
    };

    let team = Team {
        name: String::from(team_name),
    };

    let me = client.post(team).teams().execute::<Value>();

    log_response(me);
}

// == PATCH team  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-update
// Requires the Heroku client and the team name you want to patch
// PATCH /teams/
fn patch_team(client: &Heroku, team_name: &str) {
    #[derive(Serialize, Deserialize, Debug)]
    struct Team {
        certificate: String,
    };

    let team = Team {
        certificate: String::from("-----BEGIN CERTIFICATE----- "),
    };

    let me = client
        .patch(team)
        .teams()
        .team_name(team_name)
        .execute::<Value>();

    log_response(me);
}

// == PUT team  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-invitation-create
// Requires the Heroku client and the team name you want to create an invitation for
// PUT /teams/{team_name_or_id}/invitations
fn put_teams_invitation(client: &Heroku, team_name: &str) {
    #[derive(Serialize, Deserialize, Debug)]
    struct Invitation {
        email: String,
    };

    let team = Invitation {
        email: String::from("username@example.com"),
    };

    let me = client
        .put(team)
        .teams()
        .team_name(team_name)
        .team_invitation()
        .execute::<Value>();

    log_response(me);
}

// == DELETE team  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-delete
// Requires the Heroku client and the team name you want to delete
// DELETE /teams/{team_name_or_id}
fn delete_team(client: &Heroku, team_name: &str) {
    let me = client
        .delete_empty()
        .teams()
        .team_name(team_name)
        .execute::<Value>();

    log_response(me);
}

// == DELETE team member  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-member-delete
// Requires the Heroku client, the team name and the member email of the you want to delete
// You can also delete a team member by id  .team_members().member_id("ID_HERE")
// DELETE /teams/{team_name_or_id}/members/{team_member_email_or_id}

fn delete_team_member(client: &Heroku, team_name: &str) {
    let me = client
        .delete_empty()
        .teams()
        .team_name(team_name)
        .team_members()
        .member_email("username@example.com")
        .execute::<Value>();

    log_response(me);
}

// == DELETE team invitation  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-invitation-revoke
// Requires the Heroku client and the team name you want to delete the invitation for
// DELETE /teams/{team_name_or_id}/invitations/{team_invitation_id}
fn delete_team_invitation(client: &Heroku, team_name: &str) {
    let me = client
        .delete_empty()
        .teams()
        .team_name(team_name)
        .team_invitations()
        .invitation_id("ID HERE")
        .execute::<Value>();

    log_response(me);
}

// == POST team apps  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-app-create
// Requires the Heroku client
// POST /teams/apps
fn post_team_apps(client: &Heroku) {
    let team_app = CreateTeamApp {
        locked: None,
        name: Some(String::from("test")),
        team: None,
        personal: None,
        region: None,
        space: None,
        stack: None,
        internal_routing: None,
    };

    let me = client.post(team_app).teams().team_apps().execute::<Value>();
    log_response(me);
}

// == POST team apps collaborator ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-app-collaborator-create
// Requires the Heroku client and the app name or id you want to add the collaborator to
// POST /teams/apps/{app_id_or_name}/collaborators
fn post_team_app_collaborators(client: &Heroku, app_name: &str) {
    let team_app = CreateTeamAppCollaborator {
        permissions: None,
        silent: None,
        user: String::from("01234567"),
    };

    let me = client
        .post(team_app)
        .teams()
        .team_apps()
        .app_name(app_name)
        .app_collaborators()
        .execute::<Value>();

    log_response(me);
}
// == POST team member  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-member-create
// Requires the Heroku client, team id or name you want to create the member for
// POST /teams/{team_name_or_id}/members
fn post_team_member(client: &Heroku, team_name: &str) {
    let team_member = CreateTeamMember {
        email: String::from("someone@example.org"),
        role: String::from("viewer"),
        federated: None,
    };

    let me = client
        .post(team_member)
        .teams()
        .team_name(team_name)
        .team_member()
        .execute::<Value>();

    log_response(me);
}

// == POST team whitelist addon  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#whitelisted-entity-create-by-team
// Requires the Heroku client, team id or name you want to add the addon for
// POST /teams/{team_name_or_id}/whitelisted-addon-services
fn post_team_addon_service(client: &Heroku, team_name: &str) {
    #[derive(Serialize, Deserialize, Debug)]
    struct WhiteListAddonService {
        pub addon_service: String,
    }
    let addon = WhiteListAddonService {
        addon_service: String::from("heroku-postgresql"),
    };

    let me = client
        .post(addon)
        .teams()
        .team_name(team_name)
        .team_whitelist_addon_service()
        .execute::<Value>();

    log_response(me);
}

// == POST team identity provider  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#identity-provider-create-by-team
// Requires the Heroku client, team id or name you want to add the identity provider to
// POST /teams/{team_name}/identity-providers
fn post_team_identity_provider(client: &Heroku, team_name: &str) {
    let identity_provider = CreateTeamIdentityProvider {
        certificate: String::from("-----BEGIN CERTIFICATE----- ..."),
        entity_id: String::from("https://heroku-rs-domain.idp .com"),
        sso_target_url: String::from("https://example .com/idp/login"),
        slo_target_url: None,
    };

    let me = client
        .post(identity_provider)
        .teams()
        .team_name(team_name)
        .team_identity_provider()
        .execute::<Value>();

    log_response(me);
}

// == POST team invitation accept  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-invitation-accept
// Requires the Heroku client, and the invitation token you want to accept
// POST /teams/invitations/{team_invitation_token}/accept
fn post_team_invitation_accept(client: &Heroku) {
    let me = client
        .post_empty()
        .teams()
        .team_invitations()
        .invitation_token("test")
        .invitation_accept()
        .execute::<Value>();

    log_response(me);
}

//a generic method to log heroku responses and avoid code duplication
fn log_response<T>(me: Result<(HeaderMap, StatusCode, Option<T>), Error>)
where
    T: ToString,
{
    match me {
        Ok((headers, status, json)) => {
            println!("Headers: {:#?}", headers);
            println!("Status: {}", status);
            if let Some(json) = json {
                println!("Response: {}", json.to_string());
            }
        }
        Err(e) => println!("Err {}", e),
    }
}
