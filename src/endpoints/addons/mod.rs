use crate::framework::response::ApiResult;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::{AddonDelete, AttachmentDelete, WebhookDelete};
pub use get::{
    AddonConfigList, AddonDetails, AddonDetailsByApp, AddonList, AddonListByAccount,
    AddonListByApp, AddonListByTeam, AddonServiceDetails, AddonServiceList, AttachmentDetails,
    AttachmentDetailsByApp, AttachmentList, AttachmentListByAddon, AttachmentListByApp,
    RegionCapabilityList, RegionCapabilityListByRegion, RegionCapabilityListByService,
    WebhookDeliveryDetails, WebhookDeliveryList, WebhookDetails, WebhookEventDetails,
    WebhookEventList, WebhookList,
};
pub use patch::{
    AddonConfigUpdate, AddonConfigUpdateParams, AddonUpdate, AddonUpdateParams, WebhookUpdate,
    WebhookUpdateParams,
};
pub use post::{
    AddonActionDeprovision, AddonActionProvision, AddonCreate, AddonCreateParams,
    AddonResolutionCreate, AddonResolutionCreateParams, AttachmentCreate, AttachmentCreateParams,
    AttachmentResolutionCreate, AttachmentResolutionCreateParams, WebhookCreate,
    WebhookCreateParams,
};

impl ApiResult for Addon {}
impl ApiResult for Vec<Addon> {}

impl ApiResult for AddonAttachment {}
impl ApiResult for Vec<AddonAttachment> {}

impl ApiResult for AddonConfig {}
impl ApiResult for Vec<AddonConfig> {}

impl ApiResult for AddonRegionCapability {}
impl ApiResult for Vec<AddonRegionCapability> {}

impl ApiResult for AddonService {}
impl ApiResult for Vec<AddonService> {}

impl ApiResult for AddonWebhook {}
impl ApiResult for Vec<AddonWebhook> {}

impl ApiResult for AddonWebhookDelivery {}
impl ApiResult for Vec<AddonWebhookDelivery> {}

impl ApiResult for AddonWebhookEvent {}
impl ApiResult for Vec<AddonWebhookEvent> {}

pub use addon::Addon;
pub use addon_attachment::AddonAttachment;
pub use addon_config::AddonConfig;
pub use addon_region::AddonRegionCapability;
pub use addon_services::AddonService;
pub use addon_webhook::AddonWebhook;
pub use addon_webhook_delivery::AddonWebhookDelivery;
pub use addon_webhook_event::AddonWebhookEvent;

mod addon {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Add-on
    ///
    /// Stability: production
    ///
    /// Add-ons represent add-ons that have been provisioned and attached to one or more apps.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#add-on)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Addon {
        /// Addon action
        pub actions: Vec<Actions>,
        /// Addon service
        pub addon_service: AddonService,
        /// Billing entity
        pub billing_entity: BillingEntity,
        /// App
        pub app: App,
        /// billed price
        pub billed_price: Option<BilledPrice>,
        /// config vars exposed to the owning app by this add-on
        pub config_vars: Vec<String>,
        ///when add-on was created
        pub created_at: DateTime<Utc>,
        /// unique identifier of add-on
        pub id: String,
        /// globally unique name of the add-on
        /// pattern: ^[a-zA-Z][A-Za-z0-9_-]+$
        pub name: String,
        /// Plan
        pub plan: Plan,
        /// id of this add-on with its provider
        pub provider_id: String,
        /// state in the add-on’s lifecycle
        /// one of:"provisioning" or "provisioned" or "deprovisioned"
        pub state: String,
        /// when add-on was updated
        pub updated_at: DateTime<Utc>,
        /// URL for logging into web interface of add-on (e.g. a dashboard)
        pub web_url: Option<String>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Actions {
        /// a unique identifier
        pub id: String,
        /// the display text shown in Dashboard
        pub label: String,
        /// identifier of the action to take that is sent via SSO
        pub action: Option<String>,
        /// absolute URL to use instead of an action
        pub url: String,
        /// if the action requires the user to own the app
        pub requires_owner: Option<bool>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct AddonService {
        /// unique identifier of this add-on-service
        pub id: String,
        /// unique name of this add-on-service
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct BillingEntity {
        /// unique identifier of the billing entity
        pub id: String,
        /// name of the billing entity
        pub name: String,
        /// type of Object of the billing entity; new types allowed at any time.
        ///  one of:"app" or "team"
        #[serde(rename = "type")]
        pub type_field: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct App {
        /// unique identifier of app
        pub id: String,
        /// unique name of app
        ///  pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct BilledPrice {
        /// price in cents per unit of plan
        pub cents: i64,
        /// price is negotiated in a contract outside of monthly add-on billing
        pub contract: bool,
        /// unit of price for plan
        pub unit: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Plan {
        /// unique identifier of this plan
        pub id: String,
        /// unique name of this plan
        pub name: String,
    }
}

mod addon_attachment {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Add-on Attachment
    ///
    /// Stability: prototype
    ///
    /// An add-on attachment represents a connection between an app and an add-on that it has been given access to.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#add-on-attachment)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct AddonAttachment {
        /// addon
        pub addon: Addon,
        /// app
        pub app: App,
        /// when add-on attachment was created
        pub created_at: DateTime<Utc>,
        /// unique identifier of this add-on attachment
        pub id: String,
        /// unique name for this add-on attachment to this app
        pub name: String,
        /// attachment namespace
        pub namespace: Option<String>,
        /// when add-on attachment was updated
        pub updated_at: DateTime<Utc>,
        /// URL for logging into web interface of add-on in attached app context
        pub web_url: Option<String>,
        /// URL for add-on partners to write to an add-on’s logs
        pub log_input_url: Option<String>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Addon {
        /// unique identifier of add-on
        pub id: String,
        /// globally unique name of the add-on
        ///  pattern: ^[a-zA-Z][A-Za-z0-9_-]+$
        pub name: String,
        /// addon app
        pub app: App,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct App {
        /// unique identifier of app
        pub id: String,
        /// unique name of app
        ///  pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
        pub name: String,
    }
}

mod addon_config {
    /// Add-on Config
    ///
    /// Stability: development
    ///
    /// Configuration of an Add-on
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#add-on-config)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct AddonConfig {
        /// unique name of the config
        pub name: String,
        /// value of the config
        pub value: String,
    }
}

mod addon_region {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Add-on Region Capability
    ///
    /// Stability: production
    ///
    /// Add-on region capabilities represent the relationship between an Add-on Service and a specific Region. Only Beta and GA add-ons are returned by these endpoints.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#add-on-region-capability)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct AddonRegionCapability {
        /// unique identifier of this add-on-region-capability
        pub id: String,
        /// whether the add-on can be installed to a Space
        pub supports_private_networking: bool,
        /// add-on services represent add-ons that may be provisioned for apps.
        pub addon_service: AddonService,
        /// region represents a geographic location in which your application may run.
        pub region: Region,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct AddonService {
        /// npm package name of the add-on service’s Heroku CLI plugin
        pub cli_plugin_name: Option<String>,
        /// when add-on-service was created
        pub created_at: DateTime<Utc>,
        /// human-readable name of the add-on service provider
        pub human_name: String,
        /// unique identifier of this add-on-service
        pub id: String,
        /// unique name of this add-on-service
        pub name: String,
        /// release status for add-on service
        /// one of:"alpha" or "beta" or "ga" or "shutdown"
        pub state: String,
        /// whether or not apps can have access to more than one instance of this add-on at the same time
        pub supports_multiple_installations: bool,
        /// whether or not apps can have access to add-ons billed to a different app
        pub supports_sharing: bool,
        ///when add-on-service was updated
        pub updated_at: DateTime<Utc>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Region {
        /// country where the region exists
        pub country: String,
        /// when region was created
        pub created_at: DateTime<Utc>,
        /// description of region
        pub description: String,
        /// unique identifier of region
        pub id: String,
        /// area in the country where the region exists
        pub locale: String,
        /// unique name of region
        pub name: String,
        /// whether or not region is available for creating a Private Space
        pub private_capable: bool,
        /// provider
        pub provider: Provider,
        /// when region was updated
        pub updated_at: DateTime<Utc>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Provider {
        /// name of provider
        pub name: String,
        /// region name used by provider
        /// one of:"ap-south-1" or "eu-west-1" or "ap-southeast-1" or "ap-southeast-2" or "eu-central-1"
        ///   or "ap-northeast-2" or "ap-northeast-1" or "us-east-1" or "sa-east-1" or "us-west-1" or "us-west-2"
        pub region: String,
    }
}

mod addon_services {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Add-on Service
    ///
    /// Stability: production
    ///
    /// Add-on services represent add-ons that may be provisioned for apps. Endpoints under add-on services can be accessed without authentication.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#add-on-service)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct AddonService {
        /// npm package name of the add-on service’s Heroku CLI plugin
        pub cli_plugin_name: Option<String>,
        /// when add-on-service was created
        pub created_at: DateTime<Utc>,
        /// human-readable name of the add-on service provider
        pub human_name: String,
        /// unique identifier of this add-on-service
        pub id: String,
        /// unique name of this add-on-service
        pub name: String,
        /// release status for add-on service
        ///  one of:"alpha" or "beta" or "ga" or "shutdown"
        pub state: String,
        /// whether or not apps can have access to more than one instance of this add-on at the same time
        pub supports_multiple_installations: bool,
        /// whether or not apps can have access to add-ons billed to a different app
        pub supports_sharing: bool,
        /// when add-on-service was updated
        pub updated_at: DateTime<Utc>,
    }
}

mod addon_webhook {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Add-on Service
    ///
    /// Stability: production
    ///
    /// Add-on services represent add-ons that may be provisioned for apps. Endpoints under add-on services can be accessed without authentication.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#add-on-service)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct AddonWebhook {
        pub addon: Addon,
        /// when the webhook was created
        pub created_at: DateTime<Utc>,
        /// the webhook’s unique identifier
        pub id: String,
        /// the entities that the subscription provides notifications for
        pub include: Vec<String>,
        /// If notify, Heroku makes a single, fire-and-forget delivery attempt.
        /// If sync, Heroku attempts multiple deliveries until the request is successful or a limit is reached
        /// one of:"notify" or "sync"
        pub level: String,
        /// when the webhook was updated
        pub updated_at: DateTime<Utc>,
        /// the URL where the webhook’s notification requests are sent
        pub url: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Addon {
        /// unique identifier of add-on
        pub id: String,
        /// globally unique name of the add-on
        ///  pattern: ^[a-zA-Z][A-Za-z0-9_-]+$
        pub name: String,
    }
}

mod addon_webhook_delivery {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Add-on Webhook Delivery
    ///
    /// Stability: production
    ///
    /// Represents the delivery of a webhook notification, including its current status.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#add-on-webhook-delivery)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct AddonWebhookDelivery {
        /// when the delivery was created
        pub created_at: DateTime<Utc>,
        /// event created
        pub event: Event,
        /// the delivery’s unique identifier
        pub id: String,
        /// number of times a delivery has been attempted
        pub num_attempts: i64,
        /// when delivery will be attempted again
        pub next_attempt_at: Option<DateTime<Utc>>,
        /// last attempt of a delivery
        pub last_attempt: Option<LastAttempt>,
        /// the delivery’s status
        ///  one of:"pending" or "scheduled" or "retrying" or "failed" or "succeeded"
        pub status: String,
        /// when the delivery was last updated
        pub updated_at: DateTime<Utc>,
        /// the webhook which we get the deliveries for
        pub webhook: Webhook,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Event {
        /// the event’s unique identifier
        pub id: String,
        /// the type of entity that the event is related to
        pub include: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct LastAttempt {
        /// unique identifier of attempt
        pub id: String,
        /// http response code received during attempt
        pub code: Option<i64>,
        /// error class encountered during attempt
        pub error_class: Option<String>,
        /// status of an attempt
        ///  one of:"scheduled" or "succeeded" or "failed"
        pub status: String,
        /// when attempt was created
        pub created_at: DateTime<Utc>,
        /// when attempt was updated
        pub updated_at: DateTime<Utc>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Webhook {
        /// the webhook’s unique identifier
        pub id: String,
        /// If notify, Heroku makes a single, fire-and-forget delivery attempt.
        /// If sync, Heroku attempts multiple deliveries until the request is successful or a limit is reached
        ///  one of:"notify" or "sync"
        pub level: String,
    }
}

mod addon_webhook_event {
    use chrono::offset::Utc;
    use chrono::DateTime;
    use serde_json::Value;

    /// Add-on Webhook Event
    ///
    /// Stability: production
    ///
    /// Represents a webhook event that occurred.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#add-on-webhook-event)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct AddonWebhookEvent {
        /// when event was created
        pub created_at: DateTime<Utc>,
        /// the event’s unique identifier
        pub id: String,
        /// the type of entity that the event is related to
        pub include: String,
        /// payload
        pub payload: Payload,
        /// when the event was last updated
        pub updated_at: DateTime<Utc>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Payload {
        /// the type of event that occurred
        pub action: String,
        /// actor
        pub actor: Actor,
        /// the current details of the event
        pub data: Option<Value>,
        /// previous details of the event (if any)
        pub previous_data: Option<Value>,
        /// the type of resource associated with the event
        pub resource: String,
        /// the version of the details provided for the event
        pub version: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Actor {
        /// unique email address
        pub email: String,
        /// identifier of an account
        pub id: String,
    }
}
