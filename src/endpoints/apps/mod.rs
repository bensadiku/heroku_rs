use crate::framework::response::ApiResult;
use serde::Deserialize;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::{AppDelete, AppDisableAcm, AppWebhookDelete};
pub use get::{
    AccountAppList, AppDetails, AppFeatureDetails, AppFeatureList, AppFormationDetails,
    AppFormationList, AppList, AppWebhookDeliveryDetails, AppWebhookDeliveryList,
    AppWebhookDetails, AppWebhookList,
};
pub use patch::{
    AppFeatureUpdate, AppFeatureUpdateParams, AppFormationBatchUpdate,
    AppFormationBatchUpdateParams, AppFormationUpdate, AppFormationUpdateParams, AppRefreshAcm,
    AppUpdate, AppUpdateParams, AppWebhookUpdate, AppWebhookUpdateParams,
};
pub use post::{
    AppCreate, AppCreateParams, AppEnableAcm, AppWebhookCreate, AppWebhookCreateParams,
};

pub use formation::Formation;

impl ApiResult for App {}
impl ApiResult for Vec<App> {}

impl ApiResult for AppFeature {}
impl ApiResult for Vec<AppFeature> {}

impl ApiResult for AppWebhook {}
impl ApiResult for Vec<AppWebhook> {}

impl ApiResult for AppWebhookDelivery {}
impl ApiResult for Vec<AppWebhookDelivery> {}

impl ApiResult for formation::Formation {}
impl ApiResult for Vec<formation::Formation> {}

/// Heroku App
///
/// Stability: production
///
/// An app represents the program that you would like to deploy and run on Heroku.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct App {
    /// ACM status of this app
    pub acm: bool,
    /// when app was archived
    pub archived_at: Option<String>,
    /// description from buildpack of app
    pub buildpack_provided_description: Option<String>,
    /// Stacks are the different application execution environments available in the Heroku platform.
    pub build_stack: BuildStack,
    /// when app was created
    pub created_at: String,
    /// git repo URL of app
    pub git_url: String,
    /// unique identifier
    pub id: String,
    /// describes whether a Private Spaces app is externally routable or not
    pub internal_routing: Option<bool>,
    /// maintenance status of app
    pub maintenance: bool,
    /// name of app
    pub name: String,
    /// account owner
    pub owner: Owner,
    /// identity of team
    pub organization: Option<Organization>,
    /// identity of team
    pub team: Option<Team>,
    /// A region represents a geographic location in which your application may run.
    pub region: Region,
    /// when app was released
    pub released_at: Option<String>,
    /// git repo size in bytes of app
    pub repo_size: Option<i64>,
    /// slug size in bytes of app
    pub slug_size: Option<i64>,
    /// identity of space
    pub space: Option<Space>,
    /// Stacks are the different application execution environments available in the Heroku platform.
    pub stack: Stack,
    /// when app was updated
    pub updated_at: String,
    /// web URL of app
    pub web_url: String,
}

/// BuildStack struct containing identifier of stack and stack name.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct BuildStack {
    /// identifier of stack
    pub id: String,
    /// stack name
    pub name: String,
}

/// Owner struct containing email and name or the account.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Owner {
    /// email of owner
    pub email: String,
    /// unique identifier of owner
    pub id: String,
}

/// Organization struct containing id, name allows you to manage access to a shared group of applications and other resources.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Organization {
    /// unique identifier of organization
    pub id: String,
    /// unique identifier of organization
    pub name: String,
}

/// Teams struct containing id, name allows you to manage access to a shared group of applications and other resources.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Team {
    /// unique identifier of team
    pub id: String,
    /// unique name of team
    pub name: String,
}

/// Region struct containing id, name related to the geographic location in which your application may run.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Region {
    /// unique identifier
    pub id: String,
    /// name of region
    pub name: String,
}

/// Space struct containing id, name and shield related to the app execution environment.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Space {
    /// unique identifier of space
    pub id: String,
    /// unique name of space
    pub name: String,
    /// true if this space has shield enabled
    pub shield: bool,
}

/// Stacks are the different application execution environments available in the Heroku platform.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Stack {
    /// unique identifier
    pub id: String,
    /// name of stack
    pub name: String,
}

/// App Feature
///
/// Stability: production
///
/// An app feature represents a Heroku labs capability that can be enabled or disabled for an app on Heroku.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-feature)
///
// TODO: (ben) inspect the nullable properties more. As of 20th March 2020, Heroku docs say that none of these properties can be nullable,
//     but some are... and that's leading so an error decoding response body. e.g. invalid type: null, expected a string.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AppFeature {
    /// when app feature was created
    pub created_at: String,
    /// description of app feature
    pub description: String,
    /// documentation URL of app feature
    pub doc_url: String,
    /// whether or not app feature has been enabled
    pub enabled: bool,
    /// unique identifier of app feature
    pub id: String,
    /// unique name of app feature
    pub name: String,
    /// state of app feature
    pub state: String,
    /// when app feature was updated
    pub updated_at: String,
    /// user readable feature name
    pub display_name: Option<String>,
    /// e-mail to send feedback about the feature
    pub feedback_email: Option<String>,
}

/// App Webhook
///
/// Stability: production
///
/// Represents the details of a webhook subscription
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AppWebhook {
    /// the app that has the webhook
    pub app: WebhookApp,
    /// when app webhook was created
    pub created_at: String,
    /// unique identifier of app webhook
    pub id: String,
    /// the entities that the subscription provides notifications for
    pub include: Vec<String>,
    /// If notify, Heroku makes a single, fire-and-forget delivery attempt.
    /// If sync, Heroku attempts multiple deliveries until the request is successful or a limit is reached.
    /// one of: "notify" or "sync"
    pub level: String,
    /// when app webhook was updated
    pub updated_at: String,
    /// the URL where the webhook’s notification requests are sent
    pub url: String,
}

/// WebhookApp
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WebhookApp {
    /// unique identifier
    pub id: String,
    /// name of app
    pub name: String,
}

/// App Webhook Delivery
///
/// Stability: production
///
/// Represents the delivery of a webhook notification, including its current status.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-delivery)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AppWebhookDelivery {
    /// when the delivery was created
    pub created_at: String,
    /// the event’s struct containing
    pub event: WebhookDeliveryEvent,
    /// the delivery’s unique identifier
    pub id: String,
    /// number of times a delivery has been attempted
    pub num_attempts: i64,
    /// when delivery will be attempted again
    pub next_attempt_at: Option<String>,
    /// last attempt of a delivery
    pub last_attempt: Option<WebhookDeliveryLastAttempt>,
    /// the delivery’s status one of:"pending" or "scheduled" or "retrying" or "failed" or "succeeded"
    pub status: String,
    /// when the delivery was last updated
    pub updated_at: String,
    /// the webhook which we get the deliveries for
    pub webhook: WebhookDeliveryWebhook,
}

/// WebhookDeliveryEvent
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WebhookDeliveryEvent {
    /// the event’s unique identifier
    pub id: String,
    /// the type of entity that the event is related to
    pub include: String,
}

/// WebhookDeliveryLastAttempt
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WebhookDeliveryLastAttempt {
    /// unique identifier of attempt
    pub id: String,
    /// http response code received during attempt
    pub code: Option<i64>,
    /// error class encountered during attempt
    pub error_class: Option<String>,
    /// status of an attempt. One of:"scheduled" or "succeeded" or "failed"
    pub status: String,
    /// when attempt was created
    pub created_at: String,
    /// when attempt was updated
    pub updated_at: String,
}

/// WebhookDeliveryWebhook
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WebhookDeliveryWebhook {
    /// the webhook’s unique identifier
    pub id: String,
    /// If notify, Heroku makes a single, fire-and-forget delivery attempt.
    /// If sync, Heroku attempts multiple deliveries until the request is successful or a limit is reached.
    /// One of:"notify" or "sync"
    pub level: String,
}

mod formation {

    /// Formation
    ///
    /// Stability: production
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#formation)
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Formation {
        /// App struct
        pub app: App,
        /// command to use to launch this process
        pub command: String,
        /// when process type was created
        pub created_at: String,
        /// unique identifier of this process type
        pub id: String,
        /// number of processes to maintain
        pub quantity: i64,
        /// dyno size (default: “standard-1X”)
        pub size: String,
        /// type of process to maintain. pattern: ^[-\w]{1,128}$
        #[serde(rename = "type")]
        pub type_field: String,
        /// when dyno type was updated
        pub updated_at: String,
    }

    /// App struct
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct App {
        /// unique identifier
        pub id: String,
        /// name of app pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
        pub name: String,
    }
}
