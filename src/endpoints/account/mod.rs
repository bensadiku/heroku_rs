use crate::framework::response::ApiResult;
use serde::Deserialize;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::{AccountDelete, UserAccountDelete};
pub use get::{AccountDetails, UserAccountDetails};
pub use patch::{AccountUpdate, AccountUpdateParams, UserAccountUpdate, UserAccountUpdateParams};

impl ApiResult for Account {}

/// Heroku Account
/// An account represents an individual signed up to use the Heroku platform.
/// https://devcenter.heroku.com/articles/platform-api-reference#account
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub allow_tracking: bool,
    pub beta: bool,
    pub created_at: String,
    pub email: String,
    pub federated: bool,
    pub id: String,
    pub identity_provider: Option<IdentityProvider>,
    pub last_login: Option<String>,
    pub name: Option<String>,
    pub sms_number: Option<String>,
    pub suspended_at: Option<String>,
    pub delinquent_at: Option<String>,
    pub two_factor_authentication: bool,
    pub updated_at: String,
    pub verified: bool,
    pub default_organization: Option<DefaultOrganization>,
    pub default_team: Option<DefaultTeam>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct IdentityProvider {
    pub id: String,
    pub team: Team,
    pub organization: Organization,
    pub owner: Owner,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Team {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Organization {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Owner {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DefaultOrganization {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DefaultTeam {
    pub id: String,
    pub name: String,
}
