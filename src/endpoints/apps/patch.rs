//Anything related to patching(updating) apps and it's properties goes here.
use super::{App, AppFeature, AppWebhook, SNI, SSL};

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

impl AppUpdate {
    /// Update a Heroku app with optional parameters
    pub fn new(
        app_id: String,
        build_stack: Option<String>,
        maintenance: Option<bool>,
        name: Option<String>,
    ) -> AppUpdate {
        AppUpdate {
            app_id,
            params: AppUpdateParams {
                build_stack,
                maintenance,
                name,
            },
        }
    }

    /// Update a Heroku app without optional parameters
    pub fn create(app_id: String) -> AppUpdate {
        AppUpdate {
            app_id: app_id,
            params: AppUpdateParams {
                build_stack: None,
                maintenance: None,
                name: None,
            },
        }
    }
}

/// Update app with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-update-optional-parameters)
#[serde_with::skip_serializing_none]
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

impl AppRefreshAcm {
    pub fn new(app_id: String) -> AppRefreshAcm {
        AppRefreshAcm { app_id }
    }
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

impl AppFeatureUpdate {
    pub fn new(app_id: String, feature_id: String, enabled: bool) -> AppFeatureUpdate {
        AppFeatureUpdate {
            app_id,
            feature_id,
            params: AppFeatureUpdateParams { enabled },
        }
    }
}

/// Update an existing app feature with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-feature-update-required-parameters)
#[serde_with::skip_serializing_none]
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

impl AppWebhookUpdate {
    pub fn new(
        app_id: String,
        webhook_id: String,
        authorization: Option<String>,
        include: Option<Vec<String>>,
        level: Option<String>,
        secret: Option<String>,
        url: Option<String>,
    ) -> AppWebhookUpdate {
        AppWebhookUpdate {
            app_id,
            webhook_id,
            params: AppWebhookUpdateParams {
                authorization,
                include,
                level,
                secret,
                url,
            },
        }
    }
}

/// Update an existing app webhook with parameters.
///
/// All parameters for this patch are optional.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-update-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct AppWebhookUpdateParams {
    /// A custom Authorization header that Heroku will include with all webhook notifications [Nullable]
    pub authorization: Option<String>,
    /// The entities that the subscription provides notifications for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    /// One of: "notify" or "sync"
    /// If notify, Heroku makes a single, fire-and-forget delivery attempt. If sync, Heroku attempts multiple deliveries until the request is successful or a limit is reached
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// A value that Heroku will use to sign all webhook notification requests (the signature is included in the request’s Heroku-Webhook-Hmac-SHA256 header) [Nullable]
    pub secret: Option<String>,
    /// The URL where the webhook’s notification requests are sent
    #[serde(skip_serializing_if = "Option::is_none")]
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

/// SNI Endpoint Update
///
/// Update an existing SNI endpoint.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#sni-endpoint-update)
pub struct SNIUpdate<'a> {
    /// unique app identifier, either app id or app name
    pub app_id: &'a str,
    /// unique sni identifier
    pub sni_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: SNIUpdateParams<'a>,
}

impl<'a> SNIUpdate<'a> {
    /// Update Heroku app's SNI with parameters
    pub fn new(
        app_id: &'a str,
        sni_id: &'a str,
        certificate_chain: &'a str,
        private_key: &'a str,
    ) -> SNIUpdate<'a> {
        SNIUpdate {
            app_id,
            sni_id,
            params: SNIUpdateParams {
                certificate_chain,
                private_key,
            },
        }
    }
}

/// Update a new app sni with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#sni-endpoint-update-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct SNIUpdateParams<'a> {
    /// raw contents of the public certificate chain (eg: .crt or .pem file)
    pub certificate_chain: &'a str,
    /// contents of the private key (eg .key file)
    pub private_key: &'a str,
}

impl<'a> HerokuEndpoint<SNI, (), SNIUpdateParams<'a>> for SNIUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("apps/{}/sni-endpoints/{}", self.app_id, self.sni_id)
    }
    fn body(&self) -> Option<SNIUpdateParams<'a>> {
        Some(self.params.clone())
    }
}

/// SSL Endpoint Update
///
/// Update an existing SSL endpoint.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#ssl-endpoint-update)
pub struct SSLUpdate<'a> {
    /// unique app identifier, either app id or app name
    pub app_id: &'a str,
    /// unique ssl identifier
    pub ssl_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: SSLUpdateParams<'a>,
}

impl<'a> SSLUpdate<'a> {
    /// Update Heroku app SSL with parameters
    pub fn new(
        app_id: &'a str,
        ssl_id: &'a str,
        certificate_chain: Option<&'a str>,
        private_key: Option<&'a str>,
        preprocess: Option<bool>,
    ) -> SSLUpdate<'a> {
        SSLUpdate {
            app_id,
            ssl_id,
            params: SSLUpdateParams {
                certificate_chain,
                private_key,
                preprocess,
            },
        }
    }
}

/// Update app's ssl endpoint with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#ssl-endpoint-update-optional-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct SSLUpdateParams<'a> {
    /// raw contents of the public certificate chain (eg: .crt or .pem file)
    pub certificate_chain: Option<&'a str>,
    /// contents of the private key (eg .key file)
    pub private_key: Option<&'a str>,
    /// allow Heroku to modify an uploaded public certificate chain if deemed advantageous by adding missing intermediaries, stripping unnecessary ones, etc.
    ///  default: true
    pub preprocess: Option<bool>,
}

impl<'a> HerokuEndpoint<SSL, (), SSLUpdateParams<'a>> for SSLUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("apps/{}/ssl-endpoints/{}", self.app_id, self.ssl_id)
    }
    fn body(&self) -> Option<SSLUpdateParams<'a>> {
        Some(self.params.clone())
    }
}
