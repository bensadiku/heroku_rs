//Anything related to patching(updating) apps and it's properties goes here.
use super::{App, AppFeature, AppWebhook};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Update an existing app.
/// identifier can be the app id or app name.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-update
pub struct AppUpdate {
    pub app_identifier: String,
    pub params: AppUpdateParams,
}

/// Update app with parameters.
/// All three paramemters are optional.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-update-optional-parameters
#[derive(Serialize, Clone, Debug)]
pub struct AppUpdateParams {
    pub build_stack: Option<String>,
    pub maintenance: Option<bool>,
    pub name: Option<String>,
}

impl HerokuEndpoint<App, (), AppUpdateParams> for AppUpdate {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("apps/{}", self.app_identifier)
    }
    fn body(&self) -> Option<AppUpdateParams> {
        Some(self.params.clone())
    }
}

/// Refresh ACM for an app
/// app_identifier is required to refresh app acm.
/// app_identifier can be the app id or name.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-refresh-acm
pub struct AppRefreshAcm {
    pub app_identifier: String,
}

impl HerokuEndpoint<App> for AppRefreshAcm {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("apps/{}/acm", self.app_identifier)
    }
}

/// Update an existing app feature.
/// app_identifier can be the app id or app name.
/// feature_identifier can be the feature id or feature name.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-feature-update
pub struct AppFeatureUpdate {
    pub app_identifier: String,
    pub feature_identifier: String,
    pub params: AppFeatureUpdateParams,
}

/// Update an existing app feature with parameters.
/// enabled: whether or not app feature should be enabled
/// https://devcenter.heroku.com/articles/platform-api-reference#app-feature-update-required-parameters
#[derive(Serialize, Clone, Debug)]
pub struct AppFeatureUpdateParams {
    pub enabled: bool,
}

impl HerokuEndpoint<AppFeature, (), AppFeatureUpdateParams> for AppFeatureUpdate {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!(
            "apps/{}/features/{}",
            self.app_identifier, self.feature_identifier
        )
    }
    fn body(&self) -> Option<AppFeatureUpdateParams> {
        Some(self.params.clone())
    }
}

/// App Webhook Update.
/// Updates the details of an app webhook subscription.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-update
pub struct AppWebhookUpdate {
    /// app_identifier can be the app id or app name.
    pub app_identifier: String,
    /// webhook_identifier is the webhook id.
    pub webhook_identifier: String,
    /// params are the parameters sent to the API to patch the webhook.
    pub params: AppWebhookUpdateParams,
}

/// Update an existing app webhook with parameters.
/// All parameters for this patch are Optional.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-update-optional-parameters
#[derive(Serialize, Clone, Debug)]
pub struct AppWebhookUpdateParams {
    /// A custom Authorization header that Heroku will include with all webhook notifications
    pub authorization: Option<String>,
    /// The entities that the subscription provides notifications for
    pub include: Option<Vec<String>>,
    /// One of: "notify" or "sync"
    /// If notify, Heroku makes a single, fire-and-forget delivery attempt. If sync, Heroku attempts multiple deliveries until the request is successful or a limit is reached
    pub level: Option<String>,
    /// A value that Heroku will use to sign all webhook notification requests (the signature is included in the request’s Heroku-Webhook-Hmac-SHA256 header)
    pub secret: Option<String>,
    /// The URL where the webhook’s notification requests are sent
    pub url: Option<String>,
}

impl HerokuEndpoint<AppWebhook, (), AppWebhookUpdateParams> for AppWebhookUpdate {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!(
            "apps/{}/webhooks/{}",
            self.app_identifier, self.webhook_identifier
        )
    }
    fn body(&self) -> Option<AppWebhookUpdateParams> {
        Some(self.params.clone())
    }
}
