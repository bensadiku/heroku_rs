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
pub struct AddonDetails<'a> {
    pub addon_id: &'a str,
}

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
pub struct AddonList {}

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
pub struct AddonDetailsByApp<'a> {
    /// unique app identifier, either app name or app id
    pub app_id: &'a str,
    /// unique add-on identifier, either add-on id or add-on name
    pub addon_id: &'a str,
}

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
pub struct AddonListByApp<'a> {
    /// unique app identifier, either app name or app id
    pub app_id: &'a str,
}

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
pub struct AddonListByAccount<'a> {
    /// unique account identifier, either account email or account id
    pub account_id: &'a str,
}

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
pub struct AddonListByTeam<'a> {
    /// unique team identifier, either team name or team id
    pub team_id: &'a str,
}

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
pub struct AttachmentDetails<'a> {
    /// unique addoon attachment identifier
    pub attachment_id: &'a str,
}

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
pub struct AttachmentList {}

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
pub struct AttachmentListByAddon<'a> {
    /// unique add-on identifier.
    pub addon_id: &'a str,
}

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
pub struct AttachmentListByApp<'a> {
    /// unique app identifier.
    pub app_id: &'a str,
}

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
pub struct AttachmentDetailsByApp<'a> {
    /// unique app identifier.
    pub app_id: &'a str,
    /// unique attachment identifier,
    pub attachment_id: &'a str,
}

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
pub struct AddonConfigList<'a> {
    /// unique addon identifier.
    pub addon_id: &'a str,
}

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
pub struct RegionCapabilityList {}

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
pub struct RegionCapabilityListByService<'a> {
    pub service_id: &'a str,
}

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
pub struct RegionCapabilityListByRegion<'a> {
    pub region_id: &'a str,
}

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
pub struct AddonServiceDetails<'a> {
    pub service_id: &'a str,
}

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
pub struct AddonServiceList {}

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
pub struct WebhookList<'a> {
    /// unique addon identifier
    pub addon_id: &'a str,
}

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
pub struct WebhookDetails<'a> {
    /// unique addon identifier
    pub addon_id: &'a str,
    /// unique webhook identifier
    pub webhook_id: &'a str,
}

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
pub struct WebhookDeliveryDetails<'a> {
    /// unique addon identifier
    pub addon_id: &'a str,
    /// unique webhook delivery identifier
    pub delivery_id: &'a str,
}

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
pub struct WebhookDeliveryList<'a> {
    /// unique addon identifier
    pub addon_id: &'a str,
}

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
pub struct WebhookEventList<'a> {
    /// unique addon identifier
    pub addon_id: &'a str,
}

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
pub struct WebhookEventDetails<'a> {
    /// unique addon identifier
    pub addon_id: &'a str,
    /// unique webhook event identifier
    pub event_id: &'a str,
}

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
