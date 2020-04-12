//Anything related to deleting apps and it's properties goes here.
use super::{App, AppWebhook, SNI};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// App Delete
///
/// Delete an existing app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-delete)
pub struct AppDelete {
    /// app_id can be the app id or app name.
    pub app_id: String,
}

impl AppDelete {
    pub fn new(app_id: String) -> AppDelete {
        AppDelete { app_id }
    }
}

impl HerokuEndpoint<App> for AppDelete {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}", self.app_id)
    }
}

/// App Disable ACM
///
/// Disable ACM flag for an app
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-disable-acm)
pub struct AppDisableAcm {
    /// app_id can be the app id or name.
    pub app_id: String,
}

impl AppDisableAcm {
    pub fn new(app_id: String) -> AppDisableAcm {
        AppDisableAcm { app_id }
    }
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
///
/// Removes an app webhook subscription.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-delete)
pub struct AppWebhookDelete {
    /// app_id can be the app id or app name.
    pub app_id: String,
    /// webhook_id is the webhook id.
    pub webhook_id: String,
}

impl AppWebhookDelete {
    pub fn new(app_id: String, webhook_id: String) -> AppWebhookDelete {
        AppWebhookDelete { app_id, webhook_id }
    }
}

impl HerokuEndpoint<AppWebhook> for AppWebhookDelete {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/webhooks/{}", self.app_id, self.webhook_id)
    }
}

/// SNI Endpoint Delete
///
/// Delete existing SNI endpoint.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#sni-endpoint-delete)
pub struct SNIDelete<'a> {
    /// app_id can be the app id or app name.
    pub app_id: &'a str,
    /// sni unique identifier or name
    pub sni_id: &'a str,
}

impl<'a> SNIDelete<'a> {
    pub fn new(app_id: &'a str, sni_id: &'a str) -> SNIDelete<'a> {
        SNIDelete { app_id, sni_id }
    }
}

impl<'a> HerokuEndpoint<SNI> for SNIDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/sni-endpoints/{}", self.app_id, self.sni_id)
    }
}
