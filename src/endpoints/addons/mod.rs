use crate::framework::response::ApiResult;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

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
        pub id: String,
        pub label: String,
        pub action: String,
        pub url: String,
        pub requires_owner: bool,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct AddonService {
        pub id: String,
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct BillingEntity {
        pub id: String,
        pub name: String,
        #[serde(rename = "type")]
        pub type_field: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct App {
        pub id: String,
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct BilledPrice {
        pub cents: i64,
        pub contract: bool,
        pub unit: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Plan {
        pub id: String,
        pub name: String,
    }
}
