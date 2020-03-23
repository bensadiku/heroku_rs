//Anything related to patching(updating) apps and it's properties goes here.
use super::{App, AppFeature, AppWebhook};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// App Update
/// 
/// Update an existing app.
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-update)
pub struct AppUpdate {
    /// app_id can be either app id or app name.
    pub app_id: String,
    /// params are the parameters sent to the API to patch the App.
    pub params: AppUpdateParams,
}

/// Update app with parameters.
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-update-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct AppUpdateParams {
    /// unique name or identifier of stack
    pub build_stack: Option<String>,
    /// maintenance status of app
    pub maintenance: Option<bool>,
    /// name of app. pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
    pub name: Option<String>,
}

impl HerokuEndpoint<App, (), AppUpdateParams> for AppUpdate {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("apps/{}", self.app_id)
    }
    fn body(&self) -> Option<AppUpdateParams> {
        Some(self.params.clone())
    }
}

/// App Refresh ACM
/// 
/// Refresh ACM for an app
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-refresh-acm)
pub struct AppRefreshAcm {
    /// app_id can be either app id or app name.
    pub app_id: String,
}

impl HerokuEndpoint<App> for AppRefreshAcm {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("apps/{}/acm", self.app_id)
    }
}

/// App Feature Update
/// 
/// Update an existing app feature.
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-feature-update)
pub struct AppFeatureUpdate {
    /// app_id can be either app id or app name.
    pub app_id: String,
    /// feature_id can be either feature id or feature name.
    pub feature_id: String,
    /// params are the parameters sent to the API to patch the feature.
    pub params: AppFeatureUpdateParams,
}

/// Update an existing app feature with parameters.
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-feature-update-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct AppFeatureUpdateParams {
    /// whether or not app feature should be enabled
    pub enabled: bool,
}

impl HerokuEndpoint<AppFeature, (), AppFeatureUpdateParams> for AppFeatureUpdate {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("apps/{}/features/{}", self.app_id, self.feature_id)
    }
    fn body(&self) -> Option<AppFeatureUpdateParams> {
        Some(self.params.clone())
    }
}

/// App Webhook Update.
/// 
/// Updates the details of an app webhook subscription.
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-update)
pub struct AppWebhookUpdate {
    /// app_id can be the app id or app name.
    pub app_id: String,
    /// webhook_id is the webhook id.
    pub webhook_id: String,
    /// params are the parameters sent to the API to patch the webhook.
    pub params: AppWebhookUpdateParams,
}

/// Update an existing app webhook with parameters.
/// 
/// All parameters for this patch are optional.
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-update-optional-parameters)
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
        format!("apps/{}/webhooks/{}", self.app_id, self.webhook_id)
    }
    fn body(&self) -> Option<AppWebhookUpdateParams> {
        Some(self.params.clone())
    }
}
