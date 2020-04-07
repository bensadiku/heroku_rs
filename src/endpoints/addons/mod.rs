use crate::framework::response::ApiResult;
pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::AddonDelete;
pub use get::{AddonDetails, AddonDetailsByApp, AddonList, AddonListByAccount, AddonListByTeam};
pub use patch::{AddonUpdate, AddonUpdateParams};
pub use post::{
    AddonCreate, AddonCreateParams, AddonResolutionCreate, AddonResolutionCreateParams,
};

impl ApiResult for Addon {}
impl ApiResult for Vec<Addon> {}

pub use addon::Addon;

mod addon {
    use chrono::offset::Utc;
    use chrono::DateTime;
    use std::collections::HashMap;

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
        pub actions: Actions,
        /// Addon service
        pub addon_service: AddonService,
        /// Billing entity
        pub billing_entity: BillingEntity,
        /// App
        pub app: App,
        /// billed price
        pub billed_price: Option<BilledPrice>,
        /// config vars exposed to the owning app by this add-on
        pub config_vars: Vec<HashMap<String, String>>,
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
        pub action: String,
        /// absolute URL to use instead of an action
        pub url: String,
        /// if the action requires the user to own the app
        pub requires_owner: bool,
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
