//Anything related to DELETE requests for Addons and it's variations goes here.
use super::{Addon, AddonAttachment, AddonWebhook};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Add-on Delete
///
/// Delete an existing add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-delete)
///
/// # Example:
///
/// AddonDelete has two required parameters, app_id and addon_id, and returns the deleted [`Addon`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let app_id = "APP_NAME_OR_ID";
/// let addon_id = "ADDON_NAME_OR_ID";
/// let response = api_client.request(&AddonDelete::new(app_id, addon_id));
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
pub struct AddonDelete<'a> {
    /// unique app identifier, either app name or app id
    pub app_id: &'a str,
    /// unique add-on identifier, either add-on id or add-on name
    pub addon_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AddonDelete<'a> {
    /// Delete addon
    pub fn new(app_id: &'a str, addon_id: &'a str) -> AddonDelete<'a> {
        AddonDelete { app_id, addon_id }
    }
}

impl<'a> HerokuEndpoint<Addon> for AddonDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/addons/{}", self.app_id, self.addon_id)
    }
}

/// Add-on Attachment Delete
///
/// Delete an existing add-on attachment.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-attachment-delete)
///
/// # Example:
///
/// AttachmentDelete has one required parameter, attachment_id, and returns the deleted [`AddonAttachment`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
/// 
/// let response = api_client.request(&AttachmentDelete::new("ADDON_ATTACHMENT_ID"));
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
/// [response]: ../struct.AddonAttachment.html
pub struct AttachmentDelete<'a> {
    /// unique addon attachment identifier
    pub attachment_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AttachmentDelete<'a> {
    /// Delete addon
    pub fn new(attachment_id: &'a str) -> AttachmentDelete<'a> {
        AttachmentDelete { attachment_id }
    }
}

impl<'a> HerokuEndpoint<AddonAttachment> for AttachmentDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("addon-attachments/{}", self.attachment_id)
    }
}

/// Add-on Webhook Delete
///
/// Removes an add-on webhook subscription. Can only be accessed by the add-on partner providing this add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-webhook-delete)
///
/// # Example:
///
/// WebhookDelete has two required parameters, addon_id and webhook_id, and returns the deleted [`AddonWebhook`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let addon_id = "ADDON_NAME_OR_ID";
/// let webhook_id = "WEBHOOK_ID";
/// let response = api_client.request(&WebhookDelete::new(addon_id, webhook_id));
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
pub struct WebhookDelete<'a> {
    /// unique addon identifier
    pub addon_id: &'a str,
    /// unique addon webhook identifier
    pub webhook_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> WebhookDelete<'a> {
    /// Delete webhook addon
    pub fn new(addon_id: &'a str, webhook_id: &'a str) -> WebhookDelete<'a> {
        WebhookDelete {
            addon_id,
            webhook_id,
        }
    }
}

impl<'a> HerokuEndpoint<AddonWebhook> for WebhookDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("addons/{}/webhooks/{}", self.addon_id, self.webhook_id)
    }
}
