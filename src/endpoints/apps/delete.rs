//Anything related to deleting apps and it's properties goes here.
use super::{App, AppWebhook};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Delete an existing app.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-delete
pub struct AppDelete {
    /// app_id can be the app id or app name.
    pub app_id: String,
}

impl HerokuEndpoint<App> for AppDelete {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}", self.app_id)
    }
}

/// Disable ACM flag for an app
/// https://devcenter.heroku.com/articles/platform-api-reference#app-disable-acm
pub struct AppDisableAcm {
    /// app_id can be the app id or name.
    pub app_id: String,
}

impl HerokuEndpoint<App> for AppDisableAcm {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/acm", self.app_id)
    }
}

/// App Webhook Delete
/// Removes an app webhook subscription.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-delete
pub struct AppWebhookDelete {
    /// app_id can be the app id or app name.
    pub app_id: String,
    /// webhook_id is the webhook id.
    pub webhook_id: String,
}

impl HerokuEndpoint<AppWebhook> for AppWebhookDelete {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!(
            "apps/{}/webhooks/{}",
            self.app_id, self.webhook_id
        )
    }
}
