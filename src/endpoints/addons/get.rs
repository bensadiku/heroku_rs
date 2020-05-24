//Anything related to GET requests for Addons and it's variations goes here.
use super::{
    Addon, AddonAttachment, AddonConfig, AddonRegionCapability, AddonService, AddonWebhook,
    AddonWebhookDelivery, AddonWebhookEvent,
};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Add-on Info
///
/// Info for an existing add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-info)
///
/// # Example:
///
/// AddonDetails has one required parameter, addon_id, and returns the [`Addon`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let addon_id = "ADDON_NAME_OR_ID";
/// let response = api_client.request(&AddonDetails::new(addon_id));
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
pub struct AddonDetails<'a> {
    pub addon_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AddonDetails<'a> {
    pub fn new(addon_id: &'a str) -> AddonDetails {
        AddonDetails { addon_id }
    }
}

impl<'a> HerokuEndpoint<Addon> for AddonDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("addons/{}", self.addon_id)
    }
}

/// Add-on List
///
/// List all existing add-ons.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-list)
///
/// # Example:
///
/// AddonList has no required parameters, and returns a list of [`Addons`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&AddonList::new());
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
pub struct AddonList {}

#[cfg(feature = "builder")]
impl AddonList {
    pub fn new() -> AddonList {
        AddonList {}
    }
}

impl HerokuEndpoint<Vec<Addon>> for AddonList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("addons")
    }
}

/// Add-on Info By App
///
/// Info for an existing add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-info-by-app)
///
/// # Example:
///
/// AddonDetailsByApp has two required parameters, app_id and addon_id, returns the [`Addon`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let app_id = "APP_NAME_OR_ID";
/// let addon_id = "ADDON_NAME_OR_ID";
/// let response = api_client.request(&AddonDetailsByApp::new(app_id,addon_id));
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
pub struct AddonDetailsByApp<'a> {
    /// unique app identifier, either app name or app id
    pub app_id: &'a str,
    /// unique add-on identifier, either add-on id or add-on name
    pub addon_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AddonDetailsByApp<'a> {
    pub fn new(app_id: &'a str, addon_id: &'a str) -> AddonDetailsByApp<'a> {
        AddonDetailsByApp { app_id, addon_id }
    }
}

impl<'a> HerokuEndpoint<Addon> for AddonDetailsByApp<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/addons/{}", self.app_id, self.addon_id)
    }
}

/// Add-on List By App
///
/// List existing add-ons for an app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-list-by-app)
///
/// # Example:
///
/// AddonListByApp has one required parameter, app_id, returns a list of [`Addons`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let app_id = "APP_NAME_OR_ID";
/// let response = api_client.request(&AddonListByApp::new(app_id));
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
pub struct AddonListByApp<'a> {
    /// unique app identifier, either app name or app id
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AddonListByApp<'a> {
    pub fn new(app_id: &'a str) -> AddonListByApp<'a> {
        AddonListByApp { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<Addon>> for AddonListByApp<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/addons", self.app_id)
    }
}

/// Add-on List By User
///
/// List all existing add-ons a user has access to
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-list-by-user)
///
/// # Example:
///
/// AddonListByAccount has one required parameter, account_id, returns a list of [`Addons`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let account_id = "ACCOUNT_EMAIL_OR_ID";
/// let response = api_client.request(&AddonListByAccount::new(account_id));
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
pub struct AddonListByAccount<'a> {
    /// unique account identifier, either account email or account id
    pub account_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AddonListByAccount<'a> {
    pub fn new(account_id: &'a str) -> AddonListByAccount<'a> {
        AddonListByAccount { account_id }
    }
}

impl<'a> HerokuEndpoint<Vec<Addon>> for AddonListByAccount<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("users/{}/addons", self.account_id)
    }
}

/// Add-on List By Team
///
/// List add-ons used across all Team apps
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-list-by-team)
///
/// # Example:
///
/// AddonListByTeam has one required parameter, team_id, returns a list of [`Addons`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let team_id = "TEAM_NAME_OR_ID";
/// let response = api_client.request(&AddonListByTeam::new(team_id));
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
pub struct AddonListByTeam<'a> {
    /// unique team identifier, either team name or team id
    pub team_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AddonListByTeam<'a> {
    pub fn new(team_id: &'a str) -> AddonListByTeam<'a> {
        AddonListByTeam { team_id }
    }
}

impl<'a> HerokuEndpoint<Vec<Addon>> for AddonListByTeam<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("teams/{}/addons", self.team_id)
    }
}

/// Add-on Attachment Info
///
/// Info for existing add-on attachment.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-attachment-info)
///
/// # Example:
///
/// AttachmentDetails has one required parameter, attachment_id, returns the [`AddonAttachment`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let attachment_id = "ATTACHMENT_ID";
/// let response = api_client.request(&AttachmentDetails::new(attachment_id));
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
pub struct AttachmentDetails<'a> {
    /// unique addoon attachment identifier
    pub attachment_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AttachmentDetails<'a> {
    pub fn new(attachment_id: &'a str) -> AttachmentDetails<'a> {
        AttachmentDetails { attachment_id }
    }
}

impl<'a> HerokuEndpoint<AddonAttachment> for AttachmentDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("addon-attachments/{}", self.attachment_id)
    }
}

/// Add-on Attachment List
///
/// List existing add-on attachments.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-attachment-list)
///
/// # Example:
///
/// AttachmentList has no required parameters and returns a list of [`AddonAttachment`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&AttachmentList::new());
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
pub struct AttachmentList {}

#[cfg(feature = "builder")]
impl AttachmentList {
    pub fn new() -> AttachmentList {
        AttachmentList {}
    }
}

impl HerokuEndpoint<Vec<AddonAttachment>> for AttachmentList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("addon-attachments")
    }
}

/// Add-on Attachment List By Addon
///
/// List existing add-on attachments.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-attachment-list-by-add-on)
///
/// # Example:
///
/// AttachmentListByAddon has one required parameter and returns a list of [`AddonAttachment`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let addon_id = "ADDON_ID";
/// let response = api_client.request(&AttachmentListByAddon::new(addon_id));
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
pub struct AttachmentListByAddon<'a> {
    /// unique add-on identifier.
    pub addon_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AttachmentListByAddon<'a> {
    pub fn new(addon_id: &'a str) -> AttachmentListByAddon {
        AttachmentListByAddon { addon_id }
    }
}

impl<'a> HerokuEndpoint<Vec<AddonAttachment>> for AttachmentListByAddon<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("addons/{}/addon-attachments", self.addon_id)
    }
}

/// Add-on Attachment List by App
///
/// List existing add-on attachments for an app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-attachment-list-by-app)
///
/// # Example:
///
/// AttachmentListByApp has one required parameter, app_id, and returns a list of [`AddonAttachment`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let app_id = "APP_NAME_OR_ID";
/// let response = api_client.request(&AttachmentListByApp::new(app_id));
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
pub struct AttachmentListByApp<'a> {
    /// unique app identifier.
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AttachmentListByApp<'a> {
    pub fn new(app_id: &'a str) -> AttachmentListByApp {
        AttachmentListByApp { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<AddonAttachment>> for AttachmentListByApp<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/addon-attachments", self.app_id)
    }
}

/// Add-on Attachment Info by App
///
/// Info for existing add-on attachment for an app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-attachment-info-by-app)
///
/// # Example:
///
/// AttachmentDetailsByApp takes two required parameters, app_id and attachment_id, and returns a [`AddonAttachment`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let app_id = "APP_NAME_OR_ID";
/// let attachment_id = "ADDON_ATTACHMENT_ID";
/// let response = api_client.request(&AttachmentDetailsByApp::new(app_id, attachment_id));
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
pub struct AttachmentDetailsByApp<'a> {
    /// unique app identifier.
    pub app_id: &'a str,
    /// unique attachment identifier,
    pub attachment_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AttachmentDetailsByApp<'a> {
    pub fn new(app_id: &'a str, attachment_id: &'a str) -> AttachmentDetailsByApp<'a> {
        AttachmentDetailsByApp {
            app_id,
            attachment_id,
        }
    }
}

impl<'a> HerokuEndpoint<AddonAttachment> for AttachmentDetailsByApp<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "apps/{}/addon-attachments/{}",
            self.app_id, self.attachment_id
        )
    }
}

/// Add-on Config List
///
/// Get an add-on’s config. Accessible by customers with access and by the add-on partner providing this add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-config-list)
///
/// # Example:
///
/// AddonConfigList takes one required parameter, addon_id, and returns a list of [`AddonConfig`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let addon_id = "ADDON_ID";
/// let response = api_client.request(&AddonConfigList::new(addon_id));
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
pub struct AddonConfigList<'a> {
    /// unique addon identifier.
    pub addon_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AddonConfigList<'a> {
    pub fn new(addon_id: &'a str) -> AddonConfigList<'a> {
        AddonConfigList { addon_id }
    }
}

impl<'a> HerokuEndpoint<Vec<AddonConfig>> for AddonConfigList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("addons/{}/config", self.addon_id)
    }
}

/// Add-on Region Capability List
///
/// List all existing add-on region capabilities.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-region-capability-list)
///
/// # Example:
///
/// RegionCapabilityList takes no required parameters, and returns a list of [`AddonRegionCapability`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&RegionCapabilityList::new());
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
/// [response]: ../struct.AddonRegionCapability.html
pub struct RegionCapabilityList {}

#[cfg(feature = "builder")]
impl RegionCapabilityList {
    pub fn new() -> RegionCapabilityList {
        RegionCapabilityList {}
    }
}

impl HerokuEndpoint<Vec<AddonRegionCapability>> for RegionCapabilityList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("addon-region-capabilities")
    }
}

/// Add-on Region Capability List by Add-on Service
///
/// List existing add-on region capabilities for an add-on-service
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-region-capability-list-by-add-on-service)
///
/// # Example:
///
/// RegionCapabilityListByService takes one required parameter, service_id, and returns a list of [`AddonRegionCapability`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let service_id = "SERVICE_NAME_OR_ID";
/// let response = api_client.request(&RegionCapabilityListByService::new(service_id));
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
/// [response]: ../struct.AddonRegionCapability.html
pub struct RegionCapabilityListByService<'a> {
    /// unique service identifier, either name or id
    pub service_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> RegionCapabilityListByService<'a> {
    pub fn new(service_id: &'a str) -> RegionCapabilityListByService {
        RegionCapabilityListByService { service_id }
    }
}

impl<'a> HerokuEndpoint<Vec<AddonRegionCapability>> for RegionCapabilityListByService<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("addon-services/{}/region-capabilities", self.service_id)
    }
}

/// Add-on Region Capability List by Region
///
/// List existing add-on region capabilities for a region.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-region-capability-list-by-region)
///
/// # Example:
///
/// RegionCapabilityListByRegion takes one required parameter, region_id, and returns a list of [`AddonRegionCapability`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let region_id = "REGION_NAME_OR_ID";
/// let response = api_client.request(&RegionCapabilityListByRegion::new(region_id));
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
/// [response]: ../struct.AddonRegionCapability.html
pub struct RegionCapabilityListByRegion<'a> {
    /// unique region identifier, either name or id
    pub region_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> RegionCapabilityListByRegion<'a> {
    pub fn new(region_id: &'a str) -> RegionCapabilityListByRegion {
        RegionCapabilityListByRegion { region_id }
    }
}

impl<'a> HerokuEndpoint<Vec<AddonRegionCapability>> for RegionCapabilityListByRegion<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("regions/{}/addon-region-capabilities", self.region_id)
    }
}

/// Add-on Service Info
///
/// Info for existing add-on-service.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-service-info)
///
/// # Example:
///
/// AddonServiceDetails takes one required parameter, service_id, and returns a [`AddonService`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let service_id = "SERVICE_NAME_OR_ID";
/// let response = api_client.request(&AddonServiceDetails::new(service_id));
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
/// [response]: ../struct.AddonService.html
pub struct AddonServiceDetails<'a> {
    /// unique service identifier, either name or id
    pub service_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AddonServiceDetails<'a> {
    pub fn new(service_id: &'a str) -> AddonServiceDetails {
        AddonServiceDetails { service_id }
    }
}

impl<'a> HerokuEndpoint<AddonService> for AddonServiceDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("addon-services/{}", self.service_id)
    }
}

/// Add-on Service List
///
/// List existing add-on-services.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-service-list)
///
/// # Example:
///
/// AddonServiceList takes no required parameters, and returns a list of [`AddonServices`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&AddonServiceList::new());
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
/// [response]: ../struct.AddonService.html
pub struct AddonServiceList {}

#[cfg(feature = "builder")]
impl AddonServiceList {
    pub fn new() -> AddonServiceList {
        AddonServiceList {}
    }
}

impl<'a> HerokuEndpoint<Vec<AddonService>> for AddonServiceList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("addon-services")
    }
}

/// Add-on Webhook List
///
/// List all webhook subscriptions for a particular add-on. Can only be accessed by the add-on partner providing this add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-webhook-list)
///
/// # Example:
///
/// WebhookList takes one required parameter, addon_id, and returns a list of [`AddonWebhooks`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let addon_id = "ADDON_ID";
/// let response = api_client.request(&WebhookList::new(addon_id));
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
pub struct WebhookList<'a> {
    /// unique addon identifier
    pub addon_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> WebhookList<'a> {
    pub fn new(addon_id: &'a str) -> WebhookList {
        WebhookList { addon_id }
    }
}

impl<'a> HerokuEndpoint<Vec<AddonWebhook>> for WebhookList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("addons/{}/webhooks", self.addon_id)
    }
}

/// Add-on Webhook Info
///
/// Returns the info for an add-on webhook subscription. Can only be accessed by the add-on partner providing this add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-webhook-info)
///
/// # Example:
///
/// WebhookList takes two required parameters, addon_id and webhook_id, and returns a [`AddonWebhook`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let addon_id = "ADDON_ID";
/// let webhook_id = "WEBHOOK_ID";
/// let response = api_client.request(&WebhookDetails::new(addon_id, webhook_id));
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
pub struct WebhookDetails<'a> {
    /// unique addon identifier
    pub addon_id: &'a str,
    /// unique webhook identifier
    pub webhook_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> WebhookDetails<'a> {
    pub fn new(addon_id: &'a str, webhook_id: &'a str) -> WebhookDetails<'a> {
        WebhookDetails {
            addon_id,
            webhook_id,
        }
    }
}

impl<'a> HerokuEndpoint<AddonWebhook> for WebhookDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("addons/{}/webhooks/{}", self.addon_id, self.webhook_id)
    }
}

/// Add-on Webhook Delivery Info
///
/// Returns the info for an existing delivery. Can only be accessed by the add-on partner providing this add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-webhook-delivery-info)
///
/// # Example:
///
/// WebhookDeliveryDetails takes two required parameters, addon_id and delivery_id and returns a [`AddonWebhookDelivery`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let addon_id = "ADDON_ID";
/// let delivery_id = "WEBHOOK_DELIVERY_ID";
/// let response = api_client.request(&WebhookDeliveryDetails::new(addon_id, delivery_id));
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
/// [response]: ../struct.AddonWebhookDelivery.html
pub struct WebhookDeliveryDetails<'a> {
    /// unique addon identifier
    pub addon_id: &'a str,
    /// unique webhook delivery identifier
    pub delivery_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> WebhookDeliveryDetails<'a> {
    pub fn new(addon_id: &'a str, delivery_id: &'a str) -> WebhookDeliveryDetails<'a> {
        WebhookDeliveryDetails {
            addon_id,
            delivery_id,
        }
    }
}

impl<'a> HerokuEndpoint<AddonWebhookDelivery> for WebhookDeliveryDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "addons/{}/webhook-deliveries/{}",
            self.addon_id, self.delivery_id
        )
    }
}

/// Add-on Webhook Delivery List
///
/// Lists existing deliveries for an add-on. Can only be accessed by the add-on partner providing this add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-webhook-delivery-list)
///
/// # Example:
///
/// WebhookDeliveryList takes one required parameter, addon_id, and returns a list of [`AddonWebhookDelivery`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let addon_id = "ADDON_ID";
/// let response = api_client.request(&WebhookDeliveryList::new(addon_id));
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
/// [response]: ../struct.AddonWebhookDelivery.html
pub struct WebhookDeliveryList<'a> {
    /// unique addon identifier
    pub addon_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> WebhookDeliveryList<'a> {
    pub fn new(addon_id: &'a str) -> WebhookDeliveryList {
        WebhookDeliveryList { addon_id }
    }
}

impl<'a> HerokuEndpoint<Vec<AddonWebhookDelivery>> for WebhookDeliveryList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("addons/{}/webhook-deliveries", self.addon_id)
    }
}

/// Add-on Webhook Event List
///
/// Lists existing webhook events for an add-on. Can only be accessed by the add-on partner providing this add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-webhook-event-list)
///
/// # Example:
///
/// WebhookEventList takes one required parameter, addon_id, and returns a list of [`AddonWebhookEvent`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let addon_id = "ADDON_ID";
/// let response = api_client.request(&addons::WebhookEventList::new(addon_id));
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
/// [response]: ../struct.AddonWebhookEvent.html
pub struct WebhookEventList<'a> {
    /// unique addon identifier
    pub addon_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> WebhookEventList<'a> {
    pub fn new(addon_id: &'a str) -> WebhookEventList {
        WebhookEventList { addon_id }
    }
}

impl<'a> HerokuEndpoint<Vec<AddonWebhookEvent>> for WebhookEventList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("addons/{}/webhook-events", self.addon_id)
    }
}

/// Add-on Webhook Event Info
///
/// Returns the info for a specified webhook event. Can only be accessed by the add-on partner providing this add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-webhook-event-info)
///
/// # Example:
///
/// WebhookEventDetails takes two required parameters, addon_id and event_id, and returns a [`AddonWebhookDelivery`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let addon_id = "ADDON_ID";
/// let event_id = "WEBHOOK_EVENT_ID";
/// let response = api_client.request(&addons::WebhookEventDetails::new(addon_id, event_id));
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
/// [response]: ../struct.AddonWebhookDelivery.html
pub struct WebhookEventDetails<'a> {
    /// unique addon identifier
    pub addon_id: &'a str,
    /// unique webhook event identifier
    pub event_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> WebhookEventDetails<'a> {
    pub fn new(addon_id: &'a str, event_id: &'a str) -> WebhookEventDetails<'a> {
        WebhookEventDetails { addon_id, event_id }
    }
}

impl<'a> HerokuEndpoint<AddonWebhookDelivery> for WebhookEventDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("addons/{}/webhook-events/{}", self.addon_id, self.event_id)
    }
}
