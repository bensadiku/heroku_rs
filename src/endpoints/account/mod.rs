use crate::framework::response::ApiResult;
use serde::Deserialize;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::{AccountDelete, AppTransferDelete, UserAccountDelete};
pub use get::{
    AccountDetails, AccountFeatureDetails, AccountFeatureList, AppTransferDetails, AppTransferList,
    UserAccountDetails,
};
pub use patch::{
    AccountFeatureUpdate, AccountFeatureUpdateParams, AccountUpdate, AccountUpdateParams,
    AppTransferUpdate, AppTransferUpdateParams, UserAccountUpdate, UserAccountUpdateParams,
};

pub use post::{AppTransferCreate, AppTransferCreateParams};

impl ApiResult for Account {}

impl ApiResult for AccountFeature {}
impl ApiResult for Vec<AccountFeature> {}

impl ApiResult for AppTransfer {}
impl ApiResult for Vec<AppTransfer> {}

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

/// Heroku Account Feature
/// An account feature represents a Heroku labs capability that can be enabled or disabled for an account on Heroku.
/// hhttps://devcenter.heroku.com/articles/platform-api-reference#account-feature
// TODO: (ben) inspect the nullable properties more. As of 21th March 2020, Heroku docs say that none of these properties can be nullable,
//     but some are... and that's leading so an error decoding response body. e.g. invalid type: null, expected a string.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AccountFeature {
    pub created_at: String,
    pub description: String,
    pub doc_url: String,
    pub enabled: bool,
    pub id: String,
    pub name: String,
    pub state: String,
    pub updated_at: String,
    pub display_name: Option<String>,
    pub feedback_email: Option<String>,
}

/// Heroku Account App Transfer
/// An app transfer represents a two party interaction for transferring ownership of an app.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-transfer
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AppTransfer {
    pub app: AppTransferApp,
    pub created_at: String,
    pub id: String,
    pub owner: AppTransferOwner,
    pub recipient: AppTransferRecipient,
    pub state: String,
    pub updated_at: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AppTransferApp {
    pub name: String,
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AppTransferOwner {
    pub email: String,
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AppTransferRecipient {
    pub email: String,
    pub id: String,
}
