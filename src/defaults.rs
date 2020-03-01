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
