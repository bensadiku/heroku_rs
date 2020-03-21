use crate::framework::response::ApiResult;
use serde::Deserialize;

pub mod delete_apps;
pub mod get_apps;
pub mod patch_apps;
pub mod post_apps;

pub use delete_apps::{AppDelete, AppDisableAcm, AppWebhookDelete};
pub use get_apps::{
    AccountAppList, AppDetails, AppFeatureDetails, AppFeatureList, AppList, AppWebhookDetails,
    AppWebhookList,
};
pub use patch_apps::{
    AppFeatureUpdate, AppFeatureUpdateParams, AppRefreshAcm, AppUpdate, AppUpdateParams,
    AppWebhookUpdate, AppWebhookUpdateParams,
};
pub use post_apps::{
    AppCreate, AppCreateParams, AppEnableAcm, AppWebhookCreate, AppWebhookCreateParams,
};

impl ApiResult for App {}
impl ApiResult for Vec<App> {}

impl ApiResult for AppFeature {}
impl ApiResult for Vec<AppFeature> {}

impl ApiResult for AppWebhook {}
impl ApiResult for Vec<AppWebhook> {}

/// Heroku App
/// An app represents the program that you would like to deploy and run on Heroku.
/// https://devcenter.heroku.com/articles/platform-api-reference#app
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct App {
    pub acm: bool,
    pub archived_at: Option<String>,
    pub buildpack_provided_description: Option<String>,
    pub build_stack: BuildStack,
    pub created_at: String,
    pub git_url: String,
    pub id: String,
    pub internal_routing: Option<bool>,
    pub maintenance: bool,
    pub name: String,
    pub owner: Owner,
    pub organization: Option<Organization>,
    pub team: Option<Team>,
    pub region: Region,
    pub released_at: Option<String>,
    pub repo_size: Option<i64>,
    pub slug_size: Option<i64>,
    pub space: Option<Space>,
    pub stack: Stack,
    pub updated_at: String,
    pub web_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct BuildStack {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Owner {
    pub email: String,
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Organization {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Team {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Region {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Space {
    pub id: String,
    pub name: String,
    pub shield: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Stack {
    pub id: String,
    pub name: String,
}

/// Heroku App Feature
/// An app feature represents a Heroku labs capability that can be enabled or disabled for an app on Heroku.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-feature
// TODO: (ben) inspect the nullable properties more. As of 20th March 2020, Heroku docs say that none of these properties can be nullable,
//     but some are... and that's leading so an error decoding response body. e.g. invalid type: null, expected a string.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AppFeature {
    pub created_at: String,
    pub description: String,
    pub doc_url: String,
    pub enabled: bool,
    pub id: String,
    pub name: String,
    pub state: String,
    pub updated_at: String,
    pub display_name: Option<String>,
    pub feedback_email: Option<String>,
}

/// Heroku App Webhook
/// Represents the details of a webhook subscription
/// https://devcenter.heroku.com/articles/platform-api-reference#app-webhook
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AppWebhook {
    pub app: WebhookApp,
    pub created_at: String,
    pub id: String,
    pub include: Vec<String>,
    pub level: String,
    pub updated_at: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WebhookApp {
    pub id: String,
    pub name: String,
}
