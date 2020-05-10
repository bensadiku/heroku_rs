//Anything related to deleting apps and it's properties goes here.
use super::{App, AppWebhook, SNI, SSL};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// App Delete
///
/// Delete an existing app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-delete)
pub struct AppDelete<'a> {
    /// app_id can be the app id or app name.
    pub app_id: &'a str,
}

impl<'a> AppDelete<'a> {
    pub fn new(app_id: &'a str) -> AppDelete {
        AppDelete { app_id }
    }
}

impl<'a> HerokuEndpoint<App> for AppDelete<'a> {
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
pub struct AppDisableAcm<'a> {
    /// app_id can be the app id or name.
    pub app_id: &'a str,
}

impl<'a> AppDisableAcm<'a> {
    pub fn new(app_id: &'a str) -> AppDisableAcm {
        AppDisableAcm { app_id }
    }
}

impl<'a> HerokuEndpoint<App> for AppDisableAcm<'a> {
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
pub struct AppWebhookDelete<'a> {
    /// app_id can be the app id or app name.
    pub app_id: &'a str,
    /// webhook_id is the webhook id.
    pub webhook_id: &'a str,
}
impl<'a> AppWebhookDelete<'a> {
    pub fn new(app_id: &'a str, webhook_id: &'a str) -> AppWebhookDelete<'a> {
        AppWebhookDelete { app_id, webhook_id }
    }
}

impl<'a> HerokuEndpoint<AppWebhook> for AppWebhookDelete<'a> {
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

/// SNI Endpoint Delete
///
/// Delete existing SNI endpoint.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#ssl-endpoint-delete)
pub struct SSLDelete<'a> {
    /// app_id can be the app id or app name.
    pub app_id: &'a str,
    /// ssl unique identifier or name
    pub ssl_id: &'a str,
}

impl<'a> SSLDelete<'a> {
    pub fn new(app_id: &'a str, ssl_id: &'a str) -> SSLDelete<'a> {
        SSLDelete { app_id, ssl_id }
    }
}

impl<'a> HerokuEndpoint<SSL> for SSLDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/ssl-endpoints/{}", self.app_id, self.ssl_id)
    }
}
