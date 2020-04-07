//Anything related to creating apps and it's properties goes here.
use super::{App, AppWebhook};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// App Create
///
/// Create a new app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-create)
pub struct AppCreate {
    /// The parameters to pass to the Heroku API
    pub params: AppCreateParams,
}

impl AppCreate {
    /// Create a new Heroku app with parameters
    pub fn new(name: Option<String>, region: Option<String>, stack: Option<String>) -> AppCreate {
        AppCreate {
            params: AppCreateParams {
                name,
                region,
                stack,
            },
        }
    }

    /// Create a new Heroku app without parameters
    pub fn create() -> AppCreate {
        AppCreate {
            params: AppCreateParams {
                name: None,
                region: None,
                stack: None,
            },
        }
    }
}

/// Create a new app with parameters.
///
/// All three paramemters are optional.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-create-optional-parameters)
#[serde_with::skip_serializing_none] 
#[derive(Serialize, Clone, Debug)]
pub struct AppCreateParams {
    /// name of app. pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
    pub name: Option<String>,
    /// unique identifier or name of region
    pub region: Option<String>,
    /// unique name or identifier of stack
    pub stack: Option<String>,
}

impl HerokuEndpoint<App, (), AppCreateParams> for AppCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps")
    }
    fn body(&self) -> Option<AppCreateParams> {
        Some(self.params.clone())
    }
}

/// App Enable ACM
///
/// Enable ACM flag for an app
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-enable-acm)
pub struct AppEnableAcm {
    /// app_id can be the app id or name.
    pub app_id: String,
}

impl AppEnableAcm {
    pub fn new(app_id: String) -> AppEnableAcm {
        AppEnableAcm { app_id }
    }
}

impl HerokuEndpoint<App> for AppEnableAcm {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/acm", self.app_id)
    }
}

/// App Webhook Create
///
/// Create an app webhook subscription.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-create)
pub struct AppWebhookCreate {
    /// app_id can be the app name or the app id
    pub app_id: String,
    /// The parameters to pass to the Heroku API
    pub params: AppWebhookCreateParams,
}

impl AppWebhookCreate {
    /// Create a new webhook with optional parameters
    pub fn new(
        app_id: String,
        authorization: Option<String>,
        include: Vec<String>,
        level: String,
        secret: Option<String>,
        url: String,
    ) -> AppWebhookCreate {
        AppWebhookCreate {
            app_id,
            params: AppWebhookCreateParams {
                authorization,
                include,
                level,
                secret,
                url,
            },
        }
    }
    /// Create a new webhook without optional parameters
    pub fn create(
        app_id: String,
        include: Vec<String>,
        level: String,
        url: String,
    ) -> AppWebhookCreate {
        AppWebhookCreate {
            app_id: app_id,
            params: AppWebhookCreateParams {
                authorization: None,
                include: include,
                level: level,
                secret: None,
                url: url,
            },
        }
    }
}

/// Create a new app webhook with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-create-required-parameters)
#[serde_with::skip_serializing_none] 
#[derive(Serialize, Clone, Debug)]
pub struct AppWebhookCreateParams {
    /// A custom Authorization header that Heroku will include with all webhook notifications
    pub authorization: Option<String>,
    /// The entities that the subscription provides notifications for
    pub include: Vec<String>,
    /// One of: "notify" or "sync"
    /// If notify, Heroku makes a single, fire-and-forget delivery attempt. If sync, Heroku attempts multiple deliveries until the request is successful or a limit is reached
    pub level: String,
    /// A value that Heroku will use to sign all webhook notification requests (the signature is included in the request’s Heroku-Webhook-Hmac-SHA256 header)
    pub secret: Option<String>,
    /// The URL where the webhook’s notification requests are sent
    pub url: String,
}

impl HerokuEndpoint<AppWebhook, (), AppWebhookCreateParams> for AppWebhookCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/webhooks", self.app_id)
    }
    fn body(&self) -> Option<AppWebhookCreateParams> {
        Some(self.params.clone())
    }
}
