//Anything related to POST requests for Addons and it's variations goes here.
use super::{Addon, AddonAttachment, AddonWebhook};
use crate::framework::endpoint::{HerokuEndpoint, Method};
use std::collections::HashMap;

/// Add-on Create
///
/// Create a new add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-create)
pub struct AddonCreate<'a> {
    pub app_id: &'a str,
    params: AddonCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> AddonCreate<'a> {
    /// Create a new addon without required parameters only
    pub fn new(app_id: &'a str, plan: &'a str) -> AddonCreate<'a> {
        AddonCreate {
            app_id,
            params: AddonCreateParams {
                attachment: None,
                config: None,
                plan: plan,
                confirm: None,
                name: None,
            },
        }
    }

    pub fn attachment_name(&mut self, attachment_name: &'a str) -> &mut Self {
        self.params.attachment = Some(Attachment {
            name: Some(attachment_name),
        });
        self
    }

    pub fn config(&mut self, config: HashMap<&'a str, &'a str>) -> &mut Self {
        self.params.config = Some(config);
        self
    }

    pub fn confirm(&mut self, confirm: &'a str) -> &mut Self {
        self.params.confirm = Some(confirm);
        self
    }

    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.params.name = Some(name);
        self
    }
    pub fn build(&self) -> AddonCreate<'a> {
        AddonCreate {
            app_id: self.app_id,
            params: AddonCreateParams {
                attachment: self.params.attachment.clone(),
                config: self.params.config.clone(),
                plan: self.params.plan,
                confirm: self.params.confirm,
                name: self.params.name,
            },
        }
    }
}

/// Create add-on with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-create-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct AddonCreateParams<'a> {
    /// unique name for this add-on attachment to this app
    pub attachment: Option<Attachment<'a>>,
    /// custom add-on provisioning options
    pub config: Option<HashMap<&'a str, &'a str>>,
    /// name of billing entity for confirmation
    pub confirm: Option<&'a str>,
    /// unique identifier or name of this plan
    pub plan: &'a str,
    /// globally unique name of the add-on
    ///  pattern: ^[a-zA-Z][A-Za-z0-9_-]+$
    pub name: Option<&'a str>,
}

#[derive(Serialize, Clone, Debug)]
pub struct Attachment<'a> {
    /// unique name for this add-on attachment to this app
    pub name: Option<&'a str>,
}

impl<'a> HerokuEndpoint<Addon, (), AddonCreateParams<'a>> for AddonCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/addons", self.app_id)
    }
    fn body(&self) -> Option<AddonCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Add-on Resolution
///
/// Resolve an add-on from a name, optionally passing an app name. If there are matches it returns at least one add-on (exact match) or many.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-resolution)
pub struct AddonResolutionCreate<'a> {
    /// parameters to pass to the Heroku API
    pub params: AddonResolutionCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> AddonResolutionCreate<'a> {
    /// Create a new addon resolution without optional parameters
    pub fn new(addon: &'a str) -> AddonResolutionCreate<'a> {
        AddonResolutionCreate {
            params: AddonResolutionCreateParams {
                addon: addon,
                addon_service: None,
                app: None,
            },
        }
    }
    pub fn addon_service(&mut self, addon_service: &'a str) -> &mut Self {
        self.params.addon_service = Some(addon_service);
        self
    }
    pub fn app(&mut self, app: &'a str) -> &mut Self {
        self.params.app = Some(app);
        self
    }

    pub fn build(&self) -> AddonResolutionCreate<'a> {
        AddonResolutionCreate {
            params: AddonResolutionCreateParams {
                addon: self.params.addon,
                addon_service: self.params.addon_service,
                app: self.params.app,
            },
        }
    }
}

/// Create add-on resolution with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-resolution-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct AddonResolutionCreateParams<'a> {
    /// globally unique name of the add-on
    ///  pattern: ^[a-zA-Z][A-Za-z0-9_-]+$
    pub addon: &'a str,
    /// unique name of this add-on-service
    pub addon_service: Option<&'a str>,
    /// unique name of app
    ///  pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
    pub app: Option<&'a str>,
}

impl<'a> HerokuEndpoint<Vec<Addon>, (), AddonResolutionCreateParams<'a>>
    for AddonResolutionCreate<'a>
{
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("actions/addons/resolve")
    }
    fn body(&self) -> Option<AddonResolutionCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Add-on Action Provision
///
/// Mark an add-on as provisioned for use.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-action)
pub struct AddonActionProvision<'a> {
    pub addon_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AddonActionProvision<'a> {
    pub fn new(addon_id: &'a str) -> AddonActionProvision<'a> {
        AddonActionProvision { addon_id }
    }
}

impl<'a> HerokuEndpoint<Addon> for AddonActionProvision<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("addons/{}/actions/provision", self.addon_id)
    }
}

/// Add-on Action Deprovision
///
/// Mark an add-on as deprovisioned.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-action-deprovision)
pub struct AddonActionDeprovision<'a> {
    pub addon_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AddonActionDeprovision<'a> {
    pub fn new(addon_id: &'a str) -> AddonActionDeprovision<'a> {
        AddonActionDeprovision { addon_id }
    }
}

impl<'a> HerokuEndpoint<Addon> for AddonActionDeprovision<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("addons/{}/actions/deprovision", self.addon_id)
    }
}

/// Add-on Attachment Create
///
/// Create a new add-on attachment.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-attachment-create)
pub struct AttachmentCreate<'a> {
    /// parameters to pass to the Heroku API
    pub params: AttachmentCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> AttachmentCreate<'a> {
    /// Create a new addon resolution without optional parameters
    pub fn new(addon: &'a str, app: &'a str) -> AttachmentCreate<'a> {
        AttachmentCreate {
            params: AttachmentCreateParams {
                addon: addon,
                app: app,
                confirm: None,
                name: None,
                namespace: None,
            },
        }
    }

    pub fn confirm(&mut self, confirm: &'a str) -> &mut Self {
        self.params.confirm = Some(confirm);
        self
    }
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.params.name = Some(name);
        self
    }
    pub fn namespace(&mut self, namespace: &'a str) -> &mut Self {
        self.params.namespace = Some(namespace);
        self
    }

    pub fn build(&self) -> AttachmentCreate<'a> {
        AttachmentCreate {
            params: AttachmentCreateParams {
                addon: self.params.addon,
                app: self.params.app,
                confirm: self.params.confirm,
                name: self.params.name,
                namespace: self.params.namespace,
            },
        }
    }
}

/// Create add-on resolution with parameters.
///
/// [See Heroku documentation for more information about these paramters](https://devcenter.heroku.com/articles/platform-api-reference#add-on-attachment-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct AttachmentCreateParams<'a> {
    /// globally unique name of the add-on
    ///  pattern: ^[a-zA-Z][A-Za-z0-9_-]+$
    pub addon: &'a str,
    /// unique name of app
    ///  pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
    pub app: &'a str,
    /// name of owning app for confirmation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<&'a str>,
    /// unique name for this add-on attachment to this app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    /// attachment namespace. [Nullable]
    pub namespace: Option<&'a str>,
}

impl<'a> HerokuEndpoint<AddonAttachment, (), AttachmentCreateParams<'a>> for AttachmentCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("addon-attachments")
    }
    fn body(&self) -> Option<AttachmentCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Add-on Attachment Resolution
///
/// Resolve an add-on attachment from a name, optionally passing an app name. If there are matches it returns at least one add-on attachment (exact match) or many.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-attachment-resolution)
pub struct AttachmentResolutionCreate<'a> {
    /// parameters to pass to the Heroku API
    pub params: AttachmentResolutionCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> AttachmentResolutionCreate<'a> {
    /// Create a new addon resolution without optional parameters
    pub fn new(addon_attachment: &'a str) -> AttachmentResolutionCreate<'a> {
        AttachmentResolutionCreate {
            params: AttachmentResolutionCreateParams {
                addon_attachment: addon_attachment,
                addon_service: None,
                app: None,
            },
        }
    }
    pub fn app(&mut self, app: &'a str) -> &mut Self {
        self.params.app = Some(app);
        self
    }

    pub fn addon_service(&mut self, addon_service: &'a str) -> &mut Self {
        self.params.addon_service = Some(addon_service);
        self
    }

    pub fn build(&self) -> AttachmentResolutionCreate<'a> {
        AttachmentResolutionCreate {
            params: AttachmentResolutionCreateParams {
                addon_attachment: self.params.addon_attachment,
                addon_service: self.params.addon_service,
                app: self.params.app,
            },
        }
    }
}

/// Create add-on resolution with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-resolution-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct AttachmentResolutionCreateParams<'a> {
    /// unique name for this add-on attachment to this app
    pub addon_attachment: &'a str,
    /// unique name of this add-on-service
    pub addon_service: Option<&'a str>,
    /// unique name of app
    ///  pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
    pub app: Option<&'a str>,
}

impl<'a> HerokuEndpoint<Vec<AddonAttachment>, (), AttachmentResolutionCreateParams<'a>>
    for AttachmentResolutionCreate<'a>
{
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("actions/addon-attachments/resolve")
    }
    fn body(&self) -> Option<AttachmentResolutionCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Add-on Webhook Create
///
/// Create an add-on webhook subscription. Can only be accessed by the add-on partner providing this add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-webhook-create)
pub struct WebhookCreate<'a> {
    /// unique addon indentifier, either id or name
    pub addon_id: &'a str,
    /// parameters to pass to the Heroku API
    pub params: WebhookCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> WebhookCreate<'a> {
    /// Create a new addon webhook without optional parameters
    pub fn new(
        addon_id: &'a str,
        include: Vec<&'a str>,
        level: &'a str,
        url: &'a str,
    ) -> WebhookCreate<'a> {
        WebhookCreate {
            addon_id,
            params: WebhookCreateParams {
                authorization: None,
                include: include,
                level: level,
                secret: None,
                url: url,
            },
        }
    }

    pub fn authorization(&mut self, authorization: &'a str) -> &mut Self {
        self.params.authorization = Some(authorization);
        self
    }

    pub fn secret(&mut self, secret: &'a str) -> &mut Self {
        self.params.secret = Some(secret);
        self
    }

    pub fn build(&self) -> WebhookCreate<'a> {
        WebhookCreate {
            addon_id: self.addon_id,
            params: WebhookCreateParams {
                authorization: None,
                include: self.params.include.clone(),
                level: self.params.level,
                secret: self.params.secret,
                url: self.params.url,
            },
        }
    }
}

/// Create add-on webhook with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-webhook-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct WebhookCreateParams<'a> {
    /// a custom Authorization header that Heroku will include with all webhook notifications. [Nullable]
    pub authorization: Option<&'a str>,
    /// the entities that the subscription provides notifications for
    pub include: Vec<&'a str>,
    /// if notify, Heroku makes a single, fire-and-forget delivery attempt. If sync, Heroku attempts multiple deliveries until the request is successful or a limit is reached
    ///  one of:"notify" or "sync"
    pub level: &'a str,
    /// a value that Heroku will use to sign all webhook notification requests (the signature is included in the request’s Heroku-Webhook-Hmac-SHA256 header). [Nullable]
    pub secret: Option<&'a str>,
    /// the URL where the webhook’s notification requests are sent
    pub url: &'a str,
}

impl<'a> HerokuEndpoint<AddonWebhook, (), WebhookCreateParams<'a>> for WebhookCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("addons/{}/webhooks", self.addon_id)
    }
    fn body(&self) -> Option<WebhookCreateParams<'a>> {
        Some(self.params.clone())
    }
}
