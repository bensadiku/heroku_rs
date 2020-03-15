#![allow(dead_code)] // Until every starting struct gets used

use serde::{Deserialize, Serialize};

/// Used to enable or disable feature on heroku apps
/// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#app-feature-update
#[derive(Serialize, Deserialize)]
pub struct EnableFeature {
    pub enabled: bool,
}

/// A simple struct for an app patch
/// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#app-update
#[derive(Serialize, Deserialize)]
pub struct AppPatch {
    pub build_stack: String,
    pub maintenance: bool,
    pub name: String,
}

/// A simple struct for an app post
/// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#app-create
#[derive(Serialize, Deserialize)]
pub struct AppPost {
    pub region: String,
    pub stack: String,
    pub name: String,
}

/// A default struct to create a app webhook
/// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-create
/// include: entities you want to subscribe to notifications for
/// level: must be either notify or sync
/// url: The URL of your server endpoint that will receive all webhook notifications.
#[derive(Serialize, Deserialize)]
pub struct WebhookPost {
    pub include: Vec<String>,
    pub level: String,
    pub url: String,
}

/// A default struct to create a app build
/// Endpoint: https://devcenter.heroku.com/articles/platform-api-reference#build
#[derive(Serialize, Deserialize)]
pub struct BuildPost {
    pub buildpacks: Option<Vec<Buildpack>>,
    pub source_blob: SourceBlob,
}

/// A struct to use for the buildpacks executed for this build, in order
/// Docs: https://devcenter.heroku.com/articles/platform-api-reference#build-create-optional-parameters
/// url: the URL of the buildpack for the app
/// name: Buildpack Registry name of the buildpack for the app
#[derive(Serialize, Deserialize)]
pub struct Buildpack {
    pub url: String,
    pub name: String,
}

/// A struct to use for the build pack with required paramenters
/// Docs https://devcenter.heroku.com/articles/platform-api-reference#build-create-required-parameters
/// checksum : Optional checksum of the gzipped tarball for verifying its integrity
/// url : URL where gzipped tar archive of source code for build was downloaded.
/// version : Optional version of the gzipped tarball.
#[derive(Serialize, Deserialize)]
pub struct SourceBlob {
    pub checksum: Option<String>,
    pub url: String,
    pub version: Option<String>,
}

/// A struct to use for updating the heroku build pack
/// Docs https://devcenter.heroku.com/articles/platform-api-reference#buildpack-installations-update
/// updates : The buildpack attribute can accept a name, a url, or a urn.
#[derive(Serialize, Deserialize)]
pub struct BuildpackInstallation {
    pub updates: Vec<BuildPackUpdate>,
}
/// Used in tandem with BuildpackInstallation
#[derive(Serialize, Deserialize)]
pub struct BuildPackUpdate {
    pub buildpack: String,
}

/// A struct to use for adding a new collaborator on the app
/// Docs https://devcenter.heroku.com/articles/platform-api-reference#collaborator-create
/// silent : Optional whether to suppress email invitation when creating collaborator 
/// user : unique email address, identifier of an account or Implicit reference to currently authorized user
#[derive(Serialize, Deserialize, Debug)]
pub struct NewCollaborator {
    pub silent: Option<bool>,
    pub user: String,
}

/// A struct to use for adding a new app for a team
/// Docs https://devcenter.heroku.com/articles/platform-api-reference#collaborator-create
/// All fields on this POST request are optional
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTeamApp  {
    pub locked: Option<bool>,
    pub name: Option<String>,
    pub team: Option<String>,
    pub personal: Option<bool>,
    pub region: Option<String>,
    pub space: Option<String>,
    pub stack: Option<String>,
    pub internal_routing: Option<bool>,
}

/// A struct to use for adding updating team members 
/// Docs https://devcenter.heroku.com/articles/platform-api-reference#team-member-create 
/// Docs https://devcenter.heroku.com/articles/platform-api-reference#team-member-update
/// federated field is optional
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTeamMember {
    pub email: String,
    pub federated: Option<bool>,
    pub role: String,
}

/// A struct to use for creating a new identity provider for a team
/// Docs https://devcenter.heroku.com/articles/platform-api-reference#identity-provider-create-by-team
/// Only slo_target_url field on this POST request is optional
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTeamIdentityProvider {
    pub certificate: String,
    pub entity_id: String,
    pub sso_target_url: String,
    pub slo_target_url: Option<String>,
}

/// A struct to use for creating a new collaborator a specific app from a specific team
/// Docs https://devcenter.heroku.com/articles/platform-api-reference#team-app-collaborator-create
/// Only user field on this POST request is required
/// User should be a unique email address, identifier of an account or Implicit reference to currently authorized user
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTeamAppCollaborator {
    pub permissions: Option<Vec<String>>,
    pub silent: Option<bool>,
    pub user: String,
}

/// A struct to use for patching a team
/// Docs https://devcenter.heroku.com/articles/platform-api-reference#team-update
/// Both fields, default and name are optional
/// default : whether to use this team when none is specified
/// name : unique name of team
#[derive(Serialize, Deserialize, Debug)]
pub struct PatchTeam {
    pub default: Option<bool>,
    pub name: Option<String>,
}

/// A struct to use for patching a team's lock or unlock.
/// Docs https://devcenter.heroku.com/articles/platform-api-reference#team-app-update-locked
/// locked : are other team members forbidden from joining this app.
#[derive(Serialize, Deserialize, Debug)]
pub struct PatchTeamLock {
    pub locked: bool,
}

/// A struct to use for transferring an existing team app to another Heroku account.
/// Docs https://devcenter.heroku.com/articles/platform-api-reference#team-app-transfer-to-account
/// owner : unique email address, identifier of an account or Implicit reference to currently authorized user or a unique team name 
#[derive(Serialize, Deserialize, Debug)]
pub struct PatchTeamTransfer {
    pub owner: String,
}

pub struct PatchTeamMember {
    
}