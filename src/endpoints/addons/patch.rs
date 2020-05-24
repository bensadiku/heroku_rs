//Anything related to PATCH requests for Addons and it's variations goes here.
use super::{Addon, AddonConfig, AddonWebhook};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Add-on Update
///
/// Change add-on plan. Some add-ons may not support changing plans. In that case, an error will be returned.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-update)
///
/// # Example:
///
/// AddonUpdate takes three required parameters, addon_id, app_id, plan, and returns the updated [`Addon`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let app_id = "APP_ID";
/// let addon_id = "ADDON_ID";
/// let plan = "heroku-postgresql:dev";
/// let update = &AddonUpdate::new(app_id, addon_id, plan).name("my-addon-name").build();
/// let response = api_client.request(update);
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
/// [response]: ../struct.Addon.html
pub struct AddonUpdate<'a> {
    /// unique app identifier, either app name or app id
    pub app_id: &'a str,
    /// unique add-on identifier, either add-on id or add-on name
    pub addon_id: &'a str,
    /// parameters to pass to the Heroku API
    pub params: AddonUpdateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> AddonUpdate<'a> {
    pub fn new(app_id: &'a str, addon_id: &'a str, plan: &'a str) -> AddonUpdate<'a> {
        AddonUpdate {
            app_id,
            addon_id,
            params: AddonUpdateParams {
                plan: plan,
                name: None,
            },
        }
    }
    /// # name: globally unique name of the add-on
    ///
    /// `pattern:` ^[a-zA-Z][A-Za-z0-9_-]+$
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.params.name = Some(name);
        self
    }

    pub fn build(&self) -> AddonUpdate<'a> {
        AddonUpdate {
            app_id: self.app_id,
            addon_id: self.addon_id,
            params: AddonUpdateParams {
                plan: self.params.plan,
                name: self.params.name,
            },
        }
    }
}

/// Update add-on with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-update-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct AddonUpdateParams<'a> {
    /// unique identifier or name of this plan
    pub plan: &'a str,
    /// globally unique name of the add-on
    ///  pattern: ^[a-zA-Z][A-Za-z0-9_-]+$
    pub name: Option<&'a str>,
}

impl<'a> HerokuEndpoint<Addon, (), AddonUpdateParams<'a>> for AddonUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("apps/{}/addons/{}", self.app_id, self.addon_id)
    }
    fn body(&self) -> Option<AddonUpdateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Add-on Config Update
///
/// Update an add-on’s config. Can only be accessed by the add-on partner providing this add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-config-update)
///
/// # Example:
///
/// AddonConfigUpdate takes three required parameters, addon_id, app_id, plan, and returns a list of updated [`AddonConfig`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let update = &AddonConfigUpdate::new("ADDON_ID")
///                     .config("config_key", "config_value")
///                     .build();
/// let response = api_client.request(update);
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
/// [response]: ../struct.AddonConfig.html
pub struct AddonConfigUpdate<'a> {
    /// unique add-on identifier, either add-on id or add-on name
    pub addon_id: &'a str,
    /// parameters to pass to the Heroku API
    pub params: AddonConfigUpdateParams,
}

#[cfg(feature = "builder")]
impl<'a> AddonConfigUpdate<'a> {
    pub fn new(addon_id: &'a str) -> AddonConfigUpdate<'a> {
        AddonConfigUpdate {
            addon_id,
            params: AddonConfigUpdateParams { config: None },
        }
    }

    /// # config: a addon config to update
    pub fn config(&mut self, config_name: &'a str, config_value: &'a str) -> &mut Self {
        self.params.config = Some(vec![AddonConfig {
            name: config_name.to_owned(),
            value: config_value.to_owned(),
        }]);
        self
    }

    pub fn build(&self) -> AddonConfigUpdate<'a> {
        AddonConfigUpdate {
            addon_id: self.addon_id,
            params: AddonConfigUpdateParams {
                config: self.params.config.clone(),
            },
        }
    }
}

/// Update add-on config with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-config-update-optional-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct AddonConfigUpdateParams {
    /// add-on config to pass
    pub config: Option<Vec<AddonConfig>>,
}

impl<'a> HerokuEndpoint<Vec<AddonConfig>, (), AddonConfigUpdateParams> for AddonConfigUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("addons/{}/config", self.addon_id)
    }
    fn body(&self) -> Option<AddonConfigUpdateParams> {
        Some(self.params.clone())
    }
}

/// Add-on Webhook Update
///
/// Updates the details of an add-on webhook subscription. Can only be accessed by the add-on partner providing this add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-webhook-update)
///
/// # Example:
///
/// WebhookUpdate takes three required parameters, addon_id, app_id, plan, and returns a list of updated [`AddonWebhook`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
///     let response = api_client.request(
///         &addons::WebhookUpdate::new("ADDON_ID", "WEBHOOK_ID")
///             .include(vec!["api:release"])
///             .level("notify")
///             .secret("dcbff0c4430a2960a2552389d587bc58d30a37a8cf3f75f8fb77abe667ad")
///             .url("https://www.google.com/")
///             .authorization("Bearer 9266671b2767f804c9d5809c2d384ed57d8f8ce1abd1043e1fb3fbbcb8c3")
///             .build(),
///     );
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
/// [response]: ../struct.AddonWebhook.html
pub struct WebhookUpdate<'a> {
    /// unique add-on identifier, either add-on id or add-on name
    pub addon_id: &'a str,
    /// unique webhook identifier
    pub webhook_id: &'a str,
    /// parameters to pass to the Heroku API
    pub params: WebhookUpdateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> WebhookUpdate<'a> {
    /// Update webhook with optional parameters
    pub fn new(addon_id: &'a str, webhook_id: &'a str) -> WebhookUpdate<'a> {
        WebhookUpdate {
            addon_id,
            webhook_id,
            params: WebhookUpdateParams {
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

    pub fn build(&self) -> WebhookUpdate<'a> {
        WebhookUpdate {
            addon_id: self.addon_id,
            webhook_id: self.webhook_id,
            params: WebhookUpdateParams {
                authorization: self.params.authorization,
                include: self.params.include.clone(),
                level: self.params.level,
                secret: self.params.secret,
                url: self.params.url,
            },
        }
    }
}

/// Update add-on webhooks with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-webhook-update-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct WebhookUpdateParams<'a> {
    /// a custom Authorization header that Heroku will include with all webhook notifications. [Nullable]
    pub authorization: Option<&'a str>,
    /// the entities that the subscription provides notifications for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<&'a str>>,
    /// if notify, Heroku makes a single, fire-and-forget delivery attempt. If sync, Heroku attempts multiple deliveries until the request is successful or a limit is reached
    ///  one of:"notify" or "sync"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<&'a str>,
    /// a value that Heroku will use to sign all webhook notification requests (the signature is included in the request’s Heroku-Webhook-Hmac-SHA256 header). [Nullable]
    pub secret: Option<&'a str>,
    /// the URL where the webhook’s notification requests are sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
}

impl<'a> HerokuEndpoint<AddonWebhook, (), WebhookUpdateParams<'a>> for WebhookUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("addons/{}/webhooks/{}", self.addon_id, self.webhook_id)
    }
    fn body(&self) -> Option<WebhookUpdateParams<'a>> {
        Some(self.params.clone())
    }
}
