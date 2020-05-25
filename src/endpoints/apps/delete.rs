//Anything related to deleting apps and it's properties goes here.
use super::{App, AppWebhook, SNI, SSL};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// App Delete
///
/// Delete an existing app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-delete)
///
/// # Example:
///
/// AppDelete takes one required parameter, app_id, and returns the delete [`App`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&AppDelete::new("APP_ID_HERE"));
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
pub struct AppDelete<'a> {
    /// app_id can be the app id or app name.
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// AppDisableAcm takes one required parameter, app_id, and returns the [`App`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&AppDisableAcm::new("APP_ID"));
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
pub struct AppDisableAcm<'a> {
    /// app_id can be the app id or name.
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// AppWebhookDelete takes two required parameters, app_id and webhook_id, and returns the [`AppWebhook`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&AppWebhookDelete::new("APP_ID", "WEBHOOK_ID"));
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
pub struct AppWebhookDelete<'a> {
    /// app_id can be the app id or app name.
    pub app_id: &'a str,
    /// webhook_id is the webhook id.
    pub webhook_id: &'a str,
}
#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// SNIDelete takes two required parameters, app_id and sni_id, and returns the [`SNI`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&SNIDelete::new("APP_ID", "SNI_ID"));
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
pub struct SNIDelete<'a> {
    /// app_id can be the app id or app name.
    pub app_id: &'a str,
    /// sni unique identifier or name
    pub sni_id: &'a str,
}

#[cfg(feature = "builder")]
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

/// SSL Endpoint Delete
///
/// Delete existing SSL endpoint.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#ssl-endpoint-delete)
///
/// # Example:
///
/// SSLDelete takes two required parameters, app_id and ssl_id, and returns the [`SSL`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&SSLDelete::new("APP_ID", "SSL_ID"));
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
pub struct SSLDelete<'a> {
    /// app_id can be the app id or app name.
    pub app_id: &'a str,
    /// ssl unique identifier or name
    pub ssl_id: &'a str,
}

#[cfg(feature = "builder")]
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
