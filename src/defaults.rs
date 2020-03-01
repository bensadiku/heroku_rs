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
