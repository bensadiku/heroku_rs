//Anything related to getting apps and it's properties goes here.
use super::{App, AppFeature, AppSetup, AppWebhook, AppWebhookDelivery, WebhookEvent, SNI, SSL};

use crate::framework::apiclient::HerokuApiClient;
use crate::framework::endpoint::{HerokuEndpoint, Method};
use crate::framework::response::HerokuApiFailure;
/// App Info
///
/// Get info for existing app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-info)
///
/// # Example:
///
/// AppDetails takes one required parameter, app_id, and returns the [`App`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&AppDetails::new("APP_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.App.html
pub struct AppDetails<'a> {
    /// app_id can be the app id or app name.
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AppDetails<'a> {
    pub fn new(app_id: &'a str) -> AppDetails {
        AppDetails { app_id }
    }

    pub fn get<T: HerokuApiClient>(&self, client: &T) -> Result<App, HerokuApiFailure> {
        client.request(self)
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
///
/// # Example:
///
/// AppDetails has no required parameters, and returns a list of [`Apps`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&AppList::new());
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.App.html
pub struct AppList {}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// AccountAppList takes one required parameter, account_id, and returns a list of [`Apps`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&AccountAppList::new("ACCOUNT_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.App.html
pub struct AccountAppList<'a> {
    /// account_id can be the account email, id or self.
    pub account_id: &'a str,
}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// AppFeatureDetails takes two required parameters, app_id and feature_id, and returns a [`AppFeature`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&AppFeatureDetails::new("APP_ID","FEATURE_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.AppFeature.html
pub struct AppFeatureDetails<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// feature_id can be the feature name or id.
    pub feature_id: &'a str,
}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// AppFeatureList takes one required parameter, app_id, and returns a list of [`AppFeatures`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&AppFeatureList::new("APP_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.AppFeature.html
pub struct AppFeatureList<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// AppWebhookList takes one required parameter, app_id, and returns a list of [`AppWebhooks`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&AppWebhookList::new("APP_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.AppWebhook.html
pub struct AppWebhookList<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// AppWebhookDetails takes two required parameters, app_id and webhook_id, and returns a [`AppWebhook`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&AppWebhookDetails::new("APP_ID", "WEBHOOK_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.AppWebhook.html
pub struct AppWebhookDetails<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// webhook_id is the webhook id.
    pub webhook_id: &'a str,
}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// AppWebhookDeliveryDetails takes two required parameters, app_id and webhook_delivery_id, and returns a [`AppWebhookDelivery`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&AppWebhookDeliveryDetails::new("APP_ID", "WEBHOOK_DELIVERY_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.AppWebhookDelivery.html
pub struct AppWebhookDeliveryDetails<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// webhook_delivery_id is the webhook delivery id.
    pub webhook_delivery_id: &'a str,
}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// AppWebhookDeliveryList takes one required parameter, app_id, and returns a list of [`AppWebhookDelivery`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&AppWebhookDeliveryList::new("APP_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.AppWebhookDelivery.html
pub struct AppWebhookDeliveryList<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// AppSetupDetails takes one required parameter, setup_id, and returns a [`AppSetup`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&AppSetupDetails::new("SETUP_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.AppSetup.html
pub struct AppSetupDetails<'a> {
    /// setup_id is the unique setup identifier.
    pub setup_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AppSetupDetails<'a> {
    pub fn new(setup_id: &'a str) -> AppSetupDetails<'a> {
        AppSetupDetails { setup_id }
    }
}

impl<'a> HerokuEndpoint<AppSetup> for AppSetupDetails<'a> {
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
///
/// # Example:
///
/// SNIDetails takes two required parameters, app_id and sni_id, and returns a [`SNI`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&SNIDetails::new("APP_ID", "SNI_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.SNI.html
pub struct SNIDetails<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// sni unique identifier
    pub sni_id: &'a str,
}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// SNIList takes one required parameter, app_id, and returns a list of [`SNI`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&SNIList::new("APP_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.SNI.html
pub struct SNIList<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// SSLList takes one required parameter, app_id, and returns a list of [`SSL`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&SSLList::new("APP_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.SSL.html
pub struct SSLList<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// SSLDetails takes two required parameters, app_id and ssl_id, and returns a [`SSL`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&SSLDetails::new("APP_ID", "SSL_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.SSL.html
pub struct SSLDetails<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// ssl unique identifier
    pub ssl_id: &'a str,
}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// WebhookEventDetails takes two required parameters, app_id and event_id, and returns a [`WebhookEvent`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&apps::WebhookEventDetails::new("APP_ID", "EVENT_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.WebhookEvent.html
pub struct WebhookEventDetails<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// webhook event's unique identifier
    pub event_id: &'a str,
}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// WebhookEventList takes one required parameter, app_id, and returns a list of [`WebhookEvents`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&apps::WebhookEventList::new("APP_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.WebhookEvent.html
pub struct WebhookEventList<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
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
