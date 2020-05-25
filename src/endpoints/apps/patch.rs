//Anything related to patching(updating) apps and it's properties goes here.
use super::{App, AppFeature, AppWebhook, SNI, SSL};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// App Update
///
/// Update an existing app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-update)
///
/// # Example:
///
/// AppUpdate takes one required parameter, app_id, and returns the updated [`App`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let patch = &AppUpdate::new("APP_ID")
///     .name("cool-name")
///     .maintenance(false)
///     .build_stack("cedar-14")
///     .build();
/// let response = api_client.request(patch);
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
pub struct AppUpdate<'a> {
    /// app_id can be either app id or app name.
    pub app_id: &'a str,
    /// params are the parameters sent to the API to patch the App.
    pub params: AppUpdateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> AppUpdate<'a> {
    /// Update a Heroku app without optional parameters
    pub fn new(app_id: &'a str) -> AppUpdate {
        AppUpdate {
            app_id: app_id,
            params: AppUpdateParams {
                build_stack: None,
                maintenance: None,
                name: None,
            },
        }
    }
    /// # build_stack: unique name or identifier of stack
    pub fn build_stack(&mut self, build_stack: &'a str) -> &mut Self {
        self.params.build_stack = Some(build_stack);
        self
    }

    /// # maintenance: maintenance status of app
    pub fn maintenance(&mut self, maintenance: bool) -> &mut Self {
        self.params.maintenance = Some(maintenance);
        self
    }

    /// # name: name of app
    ///
    /// `pattern`: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.params.name = Some(name);
        self
    }

    pub fn build(&self) -> AppUpdate<'a> {
        AppUpdate {
            app_id: self.app_id,
            params: AppUpdateParams {
                build_stack: self.params.build_stack,
                maintenance: self.params.maintenance,
                name: self.params.name,
            },
        }
    }
}

/// Update app with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-update-optional-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct AppUpdateParams<'a> {
    /// unique name or identifier of stack
    pub build_stack: Option<&'a str>,
    /// maintenance status of app
    pub maintenance: Option<bool>,
    /// name of app. pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
    pub name: Option<&'a str>,
}

impl<'a> HerokuEndpoint<App, (), AppUpdateParams<'a>> for AppUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("apps/{}", self.app_id)
    }
    fn body(&self) -> Option<AppUpdateParams<'a>> {
        Some(self.params.clone())
    }
}

/// App Refresh ACM
///
/// Refresh ACM for an app
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-refresh-acm)
///
/// # Example:
///
/// AppRefreshAcm takes one required parameter, app_id, and returns a [`App`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&AppRefreshAcm::new("APP_ID"));
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
pub struct AppRefreshAcm<'a> {
    /// app_id can be either app id or app name.
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AppRefreshAcm<'a> {
    pub fn new(app_id: &'a str) -> AppRefreshAcm<'a> {
        AppRefreshAcm { app_id }
    }
}

impl<'a> HerokuEndpoint<App> for AppRefreshAcm<'a> {
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
///
/// # Example:
///
/// AppFeatureUpdate takes three required parameters, app_id, feature_id and enabled, and returns a [`AppFeature`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
/// let feature_enabled = true;
/// let patch = &AppFeatureUpdate::new("APP_ID", "FEATURE_ID", feature_enabled);
/// let response = api_client.request(patch);
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
pub struct AppFeatureUpdate<'a> {
    /// app_id can be either app id or app name.
    pub app_id: &'a str,
    /// feature_id can be either feature id or feature name.
    pub feature_id: &'a str,
    /// params are the parameters sent to the API to patch the feature.
    pub params: AppFeatureUpdateParams,
}

#[cfg(feature = "builder")]
impl<'a> AppFeatureUpdate<'a> {
    pub fn new(app_id: &'a str, feature_id: &'a str, enabled: bool) -> AppFeatureUpdate<'a> {
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

impl<'a> HerokuEndpoint<AppFeature, (), AppFeatureUpdateParams> for AppFeatureUpdate<'a> {
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
///
/// # Example:
///
/// AppWebhookUpdate takes two required parameters, app_id and webhook_id, and returns a [`AppWebhook`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
/// let update_app_webhook = &AppWebhookUpdate::new("APP_ID", "WEBHOOK_ID")
///      .include(vec!["api:release"])
///      .level("notify")
///      .url("https://www.bing.com")
///      .authorization("Bearer 9266671b2767f804c9d5809c2d384ed57d8f8ce1abd1043e1fb3fbbcb8c3")
///      .secret("dcbff0c4430a2960a2552389d587bc58d30a37a8cf3f75f8fb77abe667ad")
///      .build();
///
/// let response = api_client.request(update_app_webhook);
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
pub struct AppWebhookUpdate<'a> {
    /// app_id can be the app id or app name.
    pub app_id: &'a str,
    /// webhook_id is the webhook id.
    pub webhook_id: &'a str,
    /// params are the parameters sent to the API to patch the webhook.
    pub params: AppWebhookUpdateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> AppWebhookUpdate<'a> {
    pub fn new(app_id: &'a str, webhook_id: &'a str) -> AppWebhookUpdate<'a> {
        AppWebhookUpdate {
            app_id,
            webhook_id,
            params: AppWebhookUpdateParams {
                authorization: None,
                include: None,
                level: None,
                secret: None,
                url: None,
            },
        }
    }

    /// # authorization: a custom Authorization header that Heroku will include with all webhook notifications
    pub fn authorization(&mut self, authorization: &'a str) -> &mut Self {
        self.params.authorization = Some(authorization);
        self
    }

    /// # include: the entities that the subscription provides notifications for
    pub fn include(&mut self, include: Vec<&'a str>) -> &mut Self {
        self.params.include = Some(include);
        self
    }

    /// # level: if notify, Heroku makes a single, fire-and-forget delivery attempt. If sync, Heroku attempts multiple deliveries until the request is successful or a limit is reached
    /// 
    /// `one of`: "notify" or "sync"
    pub fn level(&mut self, level: &'a str) -> &mut Self {
        self.params.level = Some(level);
        self
    }

    /// # secret: a value that Heroku will use to sign all webhook notification requests (the signature is included in the request’s Heroku-Webhook-Hmac-SHA256 header)
    pub fn secret(&mut self, secret: &'a str) -> &mut Self {
        self.params.secret = Some(secret);
        self
    }

    /// # url: the URL where the webhook’s notification requests are sent
    pub fn url(&mut self, url: &'a str) -> &mut Self {
        self.params.url = Some(url);
        self
    }

    pub fn build(&self) -> AppWebhookUpdate<'a> {
        AppWebhookUpdate {
            app_id: self.app_id,
            webhook_id: self.webhook_id,
            params: AppWebhookUpdateParams {
                authorization: self.params.authorization,
                include: self.params.include.clone(),
                level: self.params.level,
                secret: self.params.secret,
                url: self.params.url,
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
pub struct AppWebhookUpdateParams<'a> {
    /// A custom Authorization header that Heroku will include with all webhook notifications [Nullable]
    pub authorization: Option<&'a str>,
    /// The entities that the subscription provides notifications for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<&'a str>>,
    /// One of: "notify" or "sync"
    /// If notify, Heroku makes a single, fire-and-forget delivery attempt. If sync, Heroku attempts multiple deliveries until the request is successful or a limit is reached
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<&'a str>,
    /// A value that Heroku will use to sign all webhook notification requests (the signature is included in the request’s Heroku-Webhook-Hmac-SHA256 header) [Nullable]
    pub secret: Option<&'a str>,
    /// The URL where the webhook’s notification requests are sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
}

impl<'a> HerokuEndpoint<AppWebhook, (), AppWebhookUpdateParams<'a>> for AppWebhookUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("apps/{}/webhooks/{}", self.app_id, self.webhook_id)
    }
    fn body(&self) -> Option<AppWebhookUpdateParams<'a>> {
        Some(self.params.clone())
    }
}

/// SNI Endpoint Update
///
/// Update an existing SNI endpoint.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#sni-endpoint-update)
///
/// # Example:
///
/// SNIUpdate takes four required parameters, app_id, sni_id, certificate_chain and private_key. Returns the updated [`SNI`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
/// 
/// let certificate_chain = "chain_here";
/// let private_key = "key_here";
/// let response = api_client.request(&SNIUpdate::new(
///     "APP_ID",
///     "SNI_ID",
///     certificate_chain,
///     private_key,
/// ));
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
pub struct SNIUpdate<'a> {
    /// unique app identifier, either app id or app name
    pub app_id: &'a str,
    /// unique sni identifier
    pub sni_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: SNIUpdateParams<'a>,
}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// SNIUpdate takes two required parameters, app_id, ssl_id. Returns the updated [`SSL`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
/// 
/// let update_ssl = &SSLUpdate::new("APP_ID", "SSL_ID")
///     .certificate_chain("chain_here")
///     .private_key("key_here")
///     .preprocess(true)
///     .build();
/// let response = api_client.request(update_ssl);
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
pub struct SSLUpdate<'a> {
    /// unique app identifier, either app id or app name
    pub app_id: &'a str,
    /// unique ssl identifier
    pub ssl_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: SSLUpdateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> SSLUpdate<'a> {
    /// Update Heroku app SSL with parameters
    pub fn new(app_id: &'a str, ssl_id: &'a str) -> SSLUpdate<'a> {
        SSLUpdate {
            app_id,
            ssl_id,
            params: SSLUpdateParams {
                certificate_chain: None,
                private_key: None,
                preprocess: None,
            },
        }
    }

    /// # certificate_chain: raw contents of the public certificate chain (eg: .crt or .pem file)
    pub fn certificate_chain(&mut self, certificate_chain: &'a str) -> &mut Self {
        self.params.certificate_chain = Some(certificate_chain);
        self
    }

    /// # private_key: contents of the private key (eg .key file)
    pub fn private_key(&mut self, private_key: &'a str) -> &mut Self {
        self.params.private_key = Some(private_key);
        self
    }

    /// # preprocess: allow Heroku to modify an uploaded public certificate chain if deemed advantageous by adding missing intermediaries, stripping unnecessary ones, etc.
    /// 
    /// `default`: true
    pub fn preprocess(&mut self, preprocess: bool) -> &mut Self {
        self.params.preprocess = Some(preprocess);
        self
    }
    pub fn build(&self) -> SSLUpdate<'a> {
        SSLUpdate {
            app_id: self.app_id,
            ssl_id: self.ssl_id,
            params: SSLUpdateParams {
                certificate_chain: self.params.certificate_chain,
                private_key: self.params.private_key,
                preprocess: self.params.preprocess,
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
