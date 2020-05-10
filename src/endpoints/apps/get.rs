//Anything related to getting apps and it's properties goes here.
use super::{App, AppFeature, AppSetup, AppWebhook, AppWebhookDelivery, WebhookEvent, SNI, SSL};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// App Info
///
/// Get info for existing app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-info)
pub struct AppDetails<'a> {
    /// app_id can be the app id or app name.
    pub app_id: &'a str,
}

impl<'a> AppDetails<'a> {
    pub fn new(app_id: &'a str) -> AppDetails {
        AppDetails { app_id }
    }
}

impl<'a> HerokuEndpoint<App> for AppDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}", self.app_id)
    }
}

/// App List
///
/// List existing apps.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-list)
pub struct AppList {}

impl AppList {
    pub fn new() -> AppList {
        AppList {}
    }
}

impl HerokuEndpoint<Vec<App>> for AppList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps")
    }
}

/// App List Owned and Collaborated
///
/// List owned and collaborated apps (excludes team apps).
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-list-owned-and-collaborated)
pub struct AccountAppList<'a> {
    /// account_id can be the account email, id or self.
    pub account_id: &'a str,
}

impl<'a> AccountAppList<'a> {
    pub fn new(account_id: &'a str) -> AccountAppList<'a> {
        AccountAppList { account_id }
    }
}

impl<'a> HerokuEndpoint<Vec<App>> for AccountAppList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("users/{}/apps", self.account_id)
    }
}

/// App Feature Info
///
/// Info for an existing app feature.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-feature-info)
pub struct AppFeatureDetails<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// feature_id can be the feature name or id.
    pub feature_id: &'a str,
}

impl<'a> AppFeatureDetails<'a> {
    pub fn new(app_id: &'a str, feature_id: &'a str) -> AppFeatureDetails<'a> {
        AppFeatureDetails { app_id, feature_id }
    }
}

impl<'a> HerokuEndpoint<AppFeature> for AppFeatureDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/features/{}", self.app_id, self.feature_id)
    }
}

/// App Feature List
///
/// List existing app features.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-feature-list)
pub struct AppFeatureList<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
}

impl<'a> AppFeatureList<'a> {
    pub fn new(app_id: &'a str) -> AppFeatureList {
        AppFeatureList { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<AppFeature>> for AppFeatureList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/features", self.app_id)
    }
}

/// App Webhook List
///
/// List all webhook subscriptions for a particular app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-list)
pub struct AppWebhookList<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
}

impl<'a> AppWebhookList<'a> {
    pub fn new(app_id: &'a str) -> AppWebhookList<'a> {
        AppWebhookList { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<AppWebhook>> for AppWebhookList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/webhooks", self.app_id)
    }
}

/// App Webhook Info
///
/// Returns the info for an app webhook subscription.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-info)
pub struct AppWebhookDetails<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// webhook_id is the webhook id.
    pub webhook_id: &'a str,
}

impl<'a> AppWebhookDetails<'a> {
    pub fn new(app_id: &'a str, webhook_id: &'a str) -> AppWebhookDetails<'a> {
        AppWebhookDetails { app_id, webhook_id }
    }
}

impl<'a> HerokuEndpoint<AppWebhook> for AppWebhookDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/webhooks/{}", self.app_id, self.webhook_id)
    }
}

/// App Webhook Delivery
///
/// Returns the info for an existing delivery.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-delivery-info)
pub struct AppWebhookDeliveryDetails<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// webhook_delivery_id is the webhook delivery id.
    pub webhook_delivery_id: &'a str,
}

impl<'a> AppWebhookDeliveryDetails<'a> {
    pub fn new(app_id: &'a str, webhook_delivery_id: &'a str) -> AppWebhookDeliveryDetails<'a> {
        AppWebhookDeliveryDetails {
            app_id,
            webhook_delivery_id,
        }
    }
}

impl<'a> HerokuEndpoint<AppWebhookDelivery> for AppWebhookDeliveryDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "apps/{}/webhook-deliveries/{}",
            self.app_id, self.webhook_delivery_id
        )
    }
}

/// App Webhook Deliveries
///
/// Lists existing deliveries for an app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-delivery-list)
pub struct AppWebhookDeliveryList<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
}

impl<'a> AppWebhookDeliveryList<'a> {
    pub fn new(app_id: &'a str) -> AppWebhookDeliveryList {
        AppWebhookDeliveryList { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<AppWebhookDelivery>> for AppWebhookDeliveryList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/webhook-deliveries", self.app_id,)
    }
}

/// App Setup Info
///
/// Get the status of an app setup.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-setup-info)
pub struct AppSetupDetails<'a> {
    /// setup_id is the unique setup identifier.
    pub setup_id: &'a str,
}

impl <'a>AppSetupDetails <'a>{
    pub fn new(setup_id: &'a str) -> AppSetupDetails<'a> {
        AppSetupDetails { setup_id }
    }
}

impl <'a>HerokuEndpoint<AppSetup> for AppSetupDetails <'a>{
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("app-setups/{}", self.setup_id)
    }
}

/// SNI Endpoint Info
///
/// Info for existing SNI endpoint.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#sni-endpoint-info)
pub struct SNIDetails<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// sni unique identifier
    pub sni_id: &'a str,
}

impl<'a> SNIDetails<'a> {
    pub fn new(app_id: &'a str, sni_id: &'a str) -> SNIDetails<'a> {
        SNIDetails { app_id, sni_id }
    }
}

impl<'a> HerokuEndpoint<SNI> for SNIDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/sni-endpoints/{}", self.app_id, self.sni_id)
    }
}

/// SNI Endpoint List
///
/// List existing SNI endpoints.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#sni-endpoint-list)
pub struct SNIList<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
}

impl<'a> SNIList<'a> {
    pub fn new(app_id: &'a str) -> SNIList<'a> {
        SNIList { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<SNI>> for SNIList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/sni-endpoints", self.app_id)
    }
}

/// SSL Endpoint List
///
/// List existing SSL endpoints.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#ssl-endpoint-list)
pub struct SSLList<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
}

impl<'a> SSLList<'a> {
    pub fn new(app_id: &'a str) -> SSLList<'a> {
        SSLList { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<SSL>> for SSLList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/ssl-endpoints", self.app_id)
    }
}

/// SSL Endpoint Info
///
/// Info for existing SSL endpoint.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#ssl-endpoint-info)
pub struct SSLDetails<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// ssl unique identifier
    pub ssl_id: &'a str,
}

impl<'a> SSLDetails<'a> {
    pub fn new(app_id: &'a str, ssl_id: &'a str) -> SSLDetails<'a> {
        SSLDetails { app_id, ssl_id }
    }
}

impl<'a> HerokuEndpoint<SSL> for SSLDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/ssl-endpoints/{}", self.app_id, self.ssl_id)
    }
}

/// App Webhook Event Info
///
/// Returns the info for a specified webhook event.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-event-info)
pub struct WebhookEventDetails<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// webhook event's unique identifier
    pub event_id: &'a str,
}

impl<'a> WebhookEventDetails<'a> {
    pub fn new(app_id: &'a str, event_id: &'a str) -> WebhookEventDetails<'a> {
        WebhookEventDetails { app_id, event_id }
    }
}

impl<'a> HerokuEndpoint<WebhookEvent> for WebhookEventDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/webhook-events/{}", self.app_id, self.event_id)
    }
}

/// App Webhook Event List
///
/// Lists existing webhook events for an app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-event-list)
pub struct WebhookEventList<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
}

impl<'a> WebhookEventList<'a> {
    pub fn new(app_id: &'a str) -> WebhookEventList<'a> {
        WebhookEventList { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<WebhookEvent>> for WebhookEventList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/webhook-events", self.app_id)
    }
}
