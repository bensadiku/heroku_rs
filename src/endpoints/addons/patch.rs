//Anything related to PATCH requests for Addons and it's variations goes here.
use super::{Addon, AddonConfig};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Add-on Update
///
/// Change add-on plan. Some add-ons may not support changing plans. In that case, an error will be returned.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-update)
pub struct AddonUpdate<'a> {
    /// unique app identifier, either app name or app id
    pub app_id: &'a str,
    /// unique add-on identifier, either add-on id or add-on name
    pub addon_id: &'a str,
    /// parameters to pass to the Heroku API
    pub params: AddonUpdateParams<'a>,
}

impl<'a> AddonUpdate<'a> {
    pub fn new(
        app_id: &'a str,
        addon_id: &'a str,
        plan: &'a str,
        name: Option<&'a str>,
    ) -> AddonUpdate<'a> {
        AddonUpdate {
            app_id,
            addon_id,
            params: AddonUpdateParams { plan, name },
        }
    }

    pub fn create(app_id: &'a str, addon_id: &'a str, plan: &'a str) -> AddonUpdate<'a> {
        AddonUpdate {
            app_id,
            addon_id,
            params: AddonUpdateParams {
                plan: plan,
                name: None,
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
pub struct AddonConfigUpdate<'a> {
    /// unique add-on identifier, either add-on id or add-on name
    pub addon_id: &'a str,
    /// parameters to pass to the Heroku API
    pub params: AddonConfigUpdateParams,
}

impl<'a> AddonConfigUpdate<'a> {
    pub fn new(addon_id: &'a str, config: Vec<AddonConfig>) -> AddonConfigUpdate<'a> {
        AddonConfigUpdate {
            addon_id,
            params: AddonConfigUpdateParams {
                config: Some(config),
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
pub struct WebhookUpdate<'a> {
    /// unique add-on identifier, either add-on id or add-on name
    pub addon_id: &'a str,
    /// unique webhook identifier
    pub webhook_id: &'a str,
    /// parameters to pass to the Heroku API
    pub params: WebhookUpdateParams<'a>,
}

impl<'a> WebhookUpdate<'a> {
    /// Update webhook with optional parameters
    pub fn new(
        addon_id: &'a str,
        webhook_id: &'a str,
        authorization: Option<&'a str>,
        include: Option<Vec<&'a str>>,
        level: Option<&'a str>,
        secret: Option<&'a str>,
        url: Option<&'a str>,
    ) -> WebhookUpdate<'a> {
        WebhookUpdate {
            addon_id,
            webhook_id,
            params: WebhookUpdateParams {
                authorization,
                include,
                level,
                secret,
                url,
            },
        }
    }
}

/// Update add-on webhooks with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-config-update-optional-parameters)
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

impl<'a> HerokuEndpoint<Vec<AddonConfig>, (), WebhookUpdateParams<'a>> for WebhookUpdate<'a> {
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
