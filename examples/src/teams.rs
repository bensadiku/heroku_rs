#![allow(dead_code)] // Just warns about un-used methods until they're used.
use heroku_rs::client::{Executor, Heroku};
use heroku_rs::defaults::{
    CreateTeamApp, CreateTeamAppCollaborator, CreateTeamIdentityProvider, CreateTeamMember,
    PatchTeamLock, PatchTeamTransfer,
};
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
    // get_team_identity_providers(&client, team_name);
    // get_team_permissions(&client, team_name);
    // get_team_pipeline_couplings(&client, team_name);
    // get_team_apps(&client, team_name);
    // get_team_app(&client, team_name);
    // get_team_app_collaborator(&client, team_app_name);
    // get_team_features(&client, team_name);
    // get_team_invitations(&client, team_name);
    // get_team_invitation_by_token(&client);
    // get_team_invoices(&client, team_name);
    // get_team_members(&client, team_name);
    // get_team_member_apps(&client, team_name);
    // get_team_preferences(&client);
    // get_team_spaces(&client, team_name);
    // get_team_whitelist_addon_services(&client, team_name);
    // patch_team_app_lock_unlock(&client, team_app_name);
    // patch_team_app_owner(&client, team_app_name);
    // patch_team_collaborators(&client, team_app_name);
    // patch_team_member(&client, team_name);
    // patch_team_preferences(&client);
    // delete_team_identity_provider(&client, team_name);
    // delete_team_app_collaborator(&client, team_app_name);
    // delete_team_whitelisted_addon_service(&client, team_name);
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
        email: String::from("someone @example.org"),
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

// == GET team identity providers  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#identity-provider-list-by-team
// Requires the Heroku client, and the team name you want to get identity providers of
// GET /teams/{team_name}/identity-providers
// NOTE: Only team name can be used as a parameter here
fn get_team_identity_providers(client: &Heroku, team_name: &str) {
    let me = client
        .get()
        .teams()
        .team_name(team_name)
        .team_identity_providers()
        .execute::<Value>();

    log_response(me);
}

// == GET team identity providers  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#permission-entity-list
// Requires the Heroku client, and the team name you want to get the permissions of
// GET /teams/{team_name_or_id}/permissions
fn get_team_permissions(client: &Heroku, team_name: &str) {
    let me = client
        .get()
        .teams()
        .team_name(team_name)
        .team_permissions()
        .execute::<Value>();

    log_response(me);
}

// == GET team pipeline couplings  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-list-by-team
// Requires the Heroku client, and the team name you want to get the pipline couplings of
// GET /teams/{team_name_or_id}/pipeline-couplings
fn get_team_pipeline_couplings(client: &Heroku, team_name: &str) {
    let me = client
        .get()
        .teams()
        .team_name(team_name)
        .team_pipeline_couplings()
        .execute::<Value>();

    log_response(me);
}

// == GET team app info  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-app-info
// Requires the Heroku client, and the team app name you want to get
// GET /teams/apps/{team_app_name}
fn get_team_app(client: &Heroku, team_app_name: &str) {
    let me = client
        .get()
        .teams()
        .team_apps()
        .team_app_name(team_app_name)
        .execute::<Value>();

    log_response(me);
}

// == GET team app list by team  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-app-list-by-team
// Requires the Heroku client, and the team you want to get the apps for
// GET /teams/{team_name_or_id}/apps
fn get_team_apps(client: &Heroku, team_name: &str) {
    let me = client
        .get()
        .teams()
        .team_name(team_name)
        .team_apps()
        .execute::<Value>();

    log_response(me);
}

// == GET team app collaborators ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-app-collaborator-info
// Requires the Heroku client, the team app name , and the collaborator email
// You can get all collaborators with the following: .get().teams().team_apps().team_app_name(team_app_name).app_collaborators()
// GET /teams/apps/{team_app_name}/collaborators/{team_app_collaborator_email}
fn get_team_app_collaborator(client: &Heroku, team_app_name: &str) {
    let me = client
        .get()
        .teams()
        .team_apps()
        .team_app_name(team_app_name)
        .app_collaborators()
        .collaborator_email("collaborator_email_here")
        .execute::<Value>();

    log_response(me);
}

// == GET team features  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-feature-info
// Requires the Heroku client, the team app name , and the feature name or id
// You can get all team features with the following: .get().teams().team_name(team_name).team_features()
// GET /teams/{team_name_or_id}/features/{team_feature_id_or_name}
fn get_team_features(client: &Heroku, team_name: &str) {
    let me = client
        .get()
        .teams()
        .team_name(team_name)
        .team_features()
        .feature_name("MY_FEATURE_NAME")
        .execute::<Value>();

    log_response(me);
}

// == GET team invitations  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-invitation-list
// Requires the Heroku client, the team app name you want to get the invitations for
// GET /teams/{team_name}/invitations
fn get_team_invitations(client: &Heroku, team_name: &str) {
    let me = client
        .get()
        .teams()
        .team_name(team_name)
        .team_invitations()
        .execute::<Value>();

    log_response(me);
}

// == GET an invitation by its token  ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-invitation-get
// Requires the Heroku client, the token of the invitation
// GET /teams/invitations/{team_invitation_token}
fn get_team_invitation_by_token(client: &Heroku) {
    let me = client
        .get()
        .teams()
        .team_invitations()
        .invitation_token("TOKEN_HERE")
        .execute::<Value>();

    log_response(me);
}

// == GET team invoices ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-invoice-list
// Requires the Heroku client, and the team name to get the invoices
// You can also get a specific invoice by number: .teams().team_name(team_name).team_invoices().invoice_number("1234")
// GET /teams/{team_name_or_id}/invoices
fn get_team_invoices(client: &Heroku, team_name: &str) {
    let me = client
        .get()
        .teams()
        .team_name(team_name)
        .team_invoices()
        .execute::<Value>();

    log_response(me);
}

// == GET team members ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-member-list
// Requires the Heroku client, and the team name to get the members of
// You can also get a specific member by email: .teams().team_name(team_name).team_members().member_email("EMAIL_HERE")
// You can also get a specific member by id: .teams().team_name(team_name).team_members().member_id("ID_HERE")
// GET /teams/{team_name_or_id}/members
fn get_team_members(client: &Heroku, team_name: &str) {
    let me = client
        .get()
        .teams()
        .team_name(team_name)
        .team_members()
        .execute::<Value>();

    log_response(me);
}

// == GET team member apps ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-member-list-by-member
// Requires the Heroku client, the team name, and the member email or id to get the apps
// You can also get all member apps by id: .teams().team_name(team_name).team_members().member_id("ID_HERE").team_apps()
// GET /teams/{team_name_or_id}/members
fn get_team_member_apps(client: &Heroku, team_name: &str) {
    let me = client
        .get()
        .teams()
        .team_name(team_name)
        .team_members()
        .member_email("EMAIL_HERE")
        .team_apps()
        .execute::<Value>();
    log_response(me);
}

// == GET team preference list ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-preferences-list
// Requires the Heroku client, and the preference name or id
// You can also get all prefences by id: .teams().team_preference_id("ID_HERE").preferences()
// GET /teams/{team_preferences_name_or_id}/preferences
fn get_team_preferences(client: &Heroku) {
    let me = client
        .get()
        .teams()
        .team_preference_name("NAME_HERE")
        .preferences()
        .execute::<Value>();
    log_response(me);
}

// == GET team spaces ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#space-space-list-2
// Requires the Heroku client and the team_name or team_id
// GET /teams/{team_name_or_id}/spaces
fn get_team_spaces(client: &Heroku, team_name: &str) {
    let me = client
        .get()
        .teams()
        .team_name(team_name)
        .team_spaces()
        .execute::<Value>();
    log_response(me);
}

// == GET team whitelisted addon services ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#whitelisted-entity-list-by-team
// Requires the Heroku client and the team_name or team_id
// GET /teams/{team_name_or_id}/whitelisted-addon-services
fn get_team_whitelist_addon_services(client: &Heroku, team_name: &str) {
    let me = client
        .get()
        .teams()
        .team_name(team_name)
        .team_whitelisted_addon_services()
        .execute::<Value>();
    log_response(me);
}

// == PATCH lock or unlock a team app. ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-app-update-locked
// Requires the Heroku client and the team_name
// PATCH /teams/apps/{team_app_name}
fn patch_team_app_lock_unlock(client: &Heroku, team_app_name: &str) {
    let patch = PatchTeamLock { locked: false };

    let me = client
        .patch(patch)
        .teams()
        .team_apps()
        .team_app_name(team_app_name)
        .execute::<Value>();
    log_response(me);
}

// == PATCH transfer team app. ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-app-transfer-to-account
// Requires the Heroku client and the team_name
// PATCH /teams/apps/{team_app_name}
fn patch_team_app_owner(client: &Heroku, team_app_name: &str) {
    let patch = PatchTeamTransfer {
        owner: String::from("test @gmail. com"),
    };

    let me = client
        .patch(patch)
        .teams()
        .team_apps()
        .team_app_name(team_app_name)
        .execute::<Value>();
    log_response(me);
}

// == PATCH team app collaborators. ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-app-collaborator-update
// Requires the Heroku client and the team_app_name
// PATCH /teams/apps/{team_app_name}/collaborators/{team_app_collaborator_email}
fn patch_team_collaborators(client: &Heroku, team_app_name: &str) {
    let patch = serde_json::json!({
        "permissions": [
            "view"
          ],
    });

    let me = client
        .patch(patch)
        .teams()
        .team_apps()
        .team_app_name(team_app_name)
        .app_collaborators()
        .collaborator_email("@EMAILHERE")
        .execute::<Value>();
    log_response(me);
}

// == PATCH team members. ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-member-update
// Requires the Heroku client and the team_name
// PATCH /teams/{team_name_or_id}/members
fn patch_team_member(client: &Heroku, team_name: &str) {
    let team_member = CreateTeamMember {
        email: String::from("someone @example.org"),
        role: String::from("viewer"),
        federated: None,
    };
    let me = client
        .patch(team_member)
        .teams()
        .team_name(team_name)
        .team_members()
        .execute::<Value>();
    log_response(me);
}

// == PATCH team preferences. ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#team-preferences-update
// Requires the Heroku client and the team preference name or id
// PATCH /teams/{team_preferences_name_or_id}/preferences
fn patch_team_preferences(client: &Heroku) {
    let patch = serde_json::json!({
        "whitelisting-enabled": true
    });

    let me = client
        .patch(patch)
        .teams()
        .team_preference_id("ID_HERE")
        .preferences()
        .execute::<Value>();
    log_response(me);
}

// == DELETE a team’s Identity Provider. ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#identity-provider-delete-by-team
// Requires the Heroku client and the team_name
// DELETE /teams/{team_name}/identity-providers/{identity_provider_id}
fn delete_team_identity_provider(client: &Heroku, team_name: &str) {
    let me = client
        .delete_empty()
        .teams()
        .team_name(team_name)
        .team_identity_providers()
        .identity_provider_id("ID")
        .execute::<Value>();
    log_response(me);
}

// == DELETE an existing collaborator from a team app. ==
// Endpoint: hhttps://devcenter.heroku.com/articles/platform-api-reference#team-app-collaborator-delete
// Requires the Heroku client and the team_app_name
// DELETE /teams/apps/{team_app_name}/collaborators/{team_app_collaborator_email}
fn delete_team_app_collaborator(client: &Heroku, team_app_name: &str) {
    let me = client
        .delete_empty()
        .teams()
        .team_apps()
        .team_app_name(team_app_name)
        .app_collaborators()
        .collaborator_email("EMAIL_HERE")
        .execute::<Value>();
    log_response(me);
}

// == DELETE a whitelisted entity ==
// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#whitelisted-entity-delete-by-team
// Requires the Heroku client and the team_name
// YOu can also delete by name: .teams().team_name(team_name).team_whitelisted_addon_services().whitelist_addon_service_name("NAME_HERE")
// DELETE /teams/{team_name_or_id}/whitelisted-addon-services/{whitelisted_add_on_service_id_or_name}
fn delete_team_whitelisted_addon_service(client: &Heroku, team_name: &str) {
    let me = client
        .delete_empty()
        .teams()
        .team_name(team_name)
        .team_whitelisted_addon_services()
        .whitelist_addon_service_id("ID_HERE")
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
