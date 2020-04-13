use crate::framework::response::ApiResult;
use serde::Deserialize;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::{AppDelete, AppDisableAcm, AppWebhookDelete, SNIDelete, SSLDelete};
pub use get::{
    AccountAppList, AppDetails, AppFeatureDetails, AppFeatureList, AppList, AppSetupDetails,
    AppWebhookDeliveryDetails, AppWebhookDeliveryList, AppWebhookDetails, AppWebhookList, SSLList, SSLDetails,
    SNIDetails, SNIList,
};
pub use patch::{
    AppFeatureUpdate, AppFeatureUpdateParams, AppRefreshAcm, AppUpdate, AppUpdateParams,
    AppWebhookUpdate, AppWebhookUpdateParams, SNIUpdate, SNIUpdateParams, SSLUpdate, SSLUpdateParams
};
pub use post::{
    AppCreate, AppCreateParams, AppEnableAcm, AppSetupCreate, AppSetupCreateParams,
    AppWebhookCreate, AppWebhookCreateParams, SNICreate, SNICreateParams, SSLCreate, SSLCreateParams
};

impl ApiResult for App {}
impl ApiResult for Vec<App> {}

impl ApiResult for AppFeature {}
impl ApiResult for Vec<AppFeature> {}

impl ApiResult for AppWebhook {}
impl ApiResult for Vec<AppWebhook> {}

impl ApiResult for AppWebhookDelivery {}
impl ApiResult for Vec<AppWebhookDelivery> {}

impl ApiResult for AppSetup {}
impl ApiResult for Vec<AppSetup> {}

impl ApiResult for SNI {}
impl ApiResult for Vec<SNI> {}

impl ApiResult for SSL {}
impl ApiResult for Vec<SSL> {}

pub use app_setup::AppSetup;
pub use sni_endpoints::SNI;
pub use ssl_endpoints::SSL;

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

mod app_setup {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// App Setup
    ///
    /// Stability: production
    ///
    /// An app setup represents an app on Heroku that is setup using an environment, addons, and scripts described in an app.json manifest file.
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-setup)
    ///
    // TODO: (ben) inspect the nullable properties more. As of 20th March 2020, Heroku docs say that none of these properties can be nullable,
    //     but some are... and that's leading so an error decoding response body. e.g. invalid type: null, expected a string.
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct AppSetup {
        /// unique identifier of app setup
        pub id: String,
        /// when app setup was created
        pub created_at: DateTime<Utc>,
        /// when app setup was updated
        pub updated_at: DateTime<Utc>,
        /// the overall status of app setup
        ///  one of:"failed" or "pending" or "succeeded"
        pub status: String,
        /// reason that app setup has failed
        pub failure_message: Option<String>,
        /// app
        pub app: App,
        /// identity and status of build
        pub build: Option<Build>,
        /// errors associated with invalid app.json manifest file
        pub manifest_errors: Vec<String>,
        /// result of postdeploy script
        pub postdeploy: Option<Postdeploy>,
        /// fully qualified success url
        pub resolved_success_url: Option<String>,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct App {
        /// unique identifier
        pub id: String,
        /// name of app
        ///  pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
        pub name: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Build {
        /// unique identifier of build
        pub id: String,
        /// status of build
        ///  one of:"failed" or "pending" or "succeeded"
        pub status: String,
        /// Build process output will be available from this URL as a stream. The stream is available as either text/plain or text/event-stream.
        /// Clients should be prepared to handle disconnects and can resume the stream by sending a Range header (for text/plain) or a Last-Event-Id header (for text/event-stream).
        pub output_stream_url: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Postdeploy {
        /// output of the postdeploy script
        pub output: String,
        /// The exit code of the postdeploy script
        pub exit_code: i64,
    }
}

mod sni_endpoints {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// SNI Endpoint
    ///
    /// Stability: development
    ///
    /// SNI Endpoint is a public address serving a custom SSL cert for HTTPS traffic, using the SNI TLS extension, to a Heroku app.
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#sni-endpoint)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct SNI {
        /// raw contents of the public certificate chain (eg: .crt or .pem file)
        pub certificate_chain: String,
        /// deprecated; refer to GET /apps/:id/domains for valid CNAMEs for this app
        pub cname: String,
        /// when endpoint was created
        pub created_at: DateTime<Utc>,
        /// unique identifier of this SNI endpoint
        pub id: String,
        /// unique name for SNI endpoint
        ///  pattern: ^[a-z][a-z0-9-]{2,29}$
        pub name: String,
        /// when SNI endpoint was updated
        pub updated_at: DateTime<Utc>,
    }
}

mod ssl_endpoints {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// SNI Endpoint
    ///
    /// Stability: development
    ///
    /// SNI Endpoint is a public address serving a custom SSL cert for HTTPS traffic, using the SNI TLS extension, to a Heroku app.
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#sni-endpoint)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct SSL {
        /// app
        pub app: App,
        /// raw contents of the public certificate chain (eg: .crt or .pem file)
        pub certificate_chain: String,
        /// canonical name record, the address to point a domain at
        pub cname: String,
        /// when endpoint was created
        pub created_at: DateTime<Utc>,
        /// unique identifier of this SSL endpoint
        pub id: String,
        /// unique name for SSL endpoint
        ///  pattern: ^[a-z][a-z0-9-]{2,29}$ 
        pub name: String,
        /// when endpoint was updated
        pub updated_at: DateTime<Utc>,
    }
    
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct App {
        /// unique identifier
        pub id: String,
        /// name of app
        ///  pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$ 
        pub name: String,
    }
    
}