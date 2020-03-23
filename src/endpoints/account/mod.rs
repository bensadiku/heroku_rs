use crate::framework::response::ApiResult;
use serde::Deserialize;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::{AccountDelete, AppTransferDelete, UserAccountDelete};
pub use get::{
    AccountCreditDetails, AccountCreditList, AccountDetails, AccountFeatureDetails,
    AccountFeatureList, AppTransferDetails, AppTransferList, UserAccountDetails,
};
pub use patch::{
    AccountFeatureUpdate, AccountFeatureUpdateParams, AccountUpdate, AccountUpdateParams,
    AppTransferUpdate, AppTransferUpdateParams, UserAccountUpdate, UserAccountUpdateParams,
};

pub use post::{
    AccountCreditCreate, AccountCreditCreateParams, AppTransferCreate, AppTransferCreateParams,
};

impl ApiResult for Account {}

impl ApiResult for AccountFeature {}
impl ApiResult for Vec<AccountFeature> {}

impl ApiResult for AppTransfer {}
impl ApiResult for Vec<AppTransfer> {}

impl ApiResult for Credit {}
impl ApiResult for Vec<Credit> {}

/// Heroku Account
///
/// Stability: production
/// 
/// An account represents an individual signed up to use the Heroku platform.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Account {
    /// whether to allow third party web activity tracking, default: true
    pub allow_tracking: bool,
    /// whether allowed to utilize beta Heroku features
    pub beta: bool,
    /// when account was created
    pub created_at: String,
    /// unique email address of account
    pub email: String,
    /// whether the user is federated and belongs to an Identity Provider
    pub federated: bool,
    /// unique identifier of an account
    pub id: String,
    /// Identity Provider details for federated users.
    pub identity_provider: Option<IdentityProvider>,
    /// when account last authorized with Heroku
    pub last_login: Option<String>,
    /// full name of the account owner
    pub name: Option<String>,
    /// SMS number of account
    pub sms_number: Option<String>,
    /// when account was suspended
    pub suspended_at: Option<String>,
    /// when account became delinquent
    pub delinquent_at: Option<String>,
    /// whether two-factor auth is enabled on the account
    pub two_factor_authentication: bool,
    /// when account was updated
    pub updated_at: String,
    /// whether account has been verified with billing information
    pub verified: bool,
    /// team selected by default
    pub default_organization: Option<DefaultOrganization>,
    /// team selected by default
    pub default_team: Option<DefaultTeam>,
}

/// Identity Provider
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct IdentityProvider {
    /// unique identifier of this identity provider
    pub id: String,
    /// the team that this identity provider belongs
    pub team: Team,
    /// the organization that this identity provider belongs
    pub organization: Organization,
    /// the owner that this identity provider belongs
    pub owner: Owner,
}

/// Team
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Team {
    /// unique name
    pub name: String,
}
/// Organization 
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Organization {
    /// unique name
    pub name: String,
}

/// Owner
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Owner {
    /// unique identifier of the owner
    pub id: String,
    /// name of the owner
    pub name: String,
    /// type of the owner. One of:"team" or "enterprise-account"
    #[serde(rename = "type")]
    pub type_field: String,
}

/// team selected by default
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DefaultOrganization {
    /// unique identifier of team
    pub id: String,
    /// unique name of team
    pub name: String,
}

/// Team selected by default
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DefaultTeam {
    /// unique identifier of team
    pub id: String,
    /// unique name of team
    pub name: String,
}

/// Account Feature
///
/// An account feature represents a Heroku labs capability that can be enabled or disabled for an account on Heroku.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-feature)
///
// TODO: (ben) inspect the nullable properties more. As of 21th March 2020, Heroku docs say that none of these properties can be nullable,
//     but some are... and that's leading so an error decoding response body. e.g. invalid type: null, expected a string.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AccountFeature {
    /// when account feature was created
    pub created_at: String,
    /// description of account feature
    pub description: String,
    /// documentation URL of account feature
    pub doc_url: String,
    /// whether or not account feature has been enabled
    pub enabled: bool,
    /// unique identifier of account feature
    pub id: String,
    /// unique name of account feature
    pub name: String,
    /// state of account feature
    pub state: String,
    /// when account feature was updated
    pub updated_at: String,
    /// user readable feature name
    pub display_name: Option<String>,
    /// e-mail to send feedback about the feature
    pub feedback_email: Option<String>,
}

/// Account App Transfer
///
/// An app transfer represents a two party interaction for transferring ownership of an app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-transfer)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AppTransfer {
    /// the app struct containing the app id and name
    pub app: AppTransferApp,
    /// when app transfer was created
    pub created_at: String,
    /// unique identifier of app transfer
    pub id: String,
    /// the owner struct containing the owner email and id
    pub owner: AppTransferOwner,
    /// the recipient struct containing the recipient email and id
    pub recipient: AppTransferRecipient,
    /// the current state of an app transfer, one of:"pending" or "accepted" or "declined"
    pub state: String,
    /// when app transfer was updated
    pub updated_at: String,
}

/// AppTransferApp
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AppTransferApp {
    /// unique name of team
    pub name: String,
    /// unique identifier of team
    pub id: String,
}

/// AppTransferOwner
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AppTransferOwner {
    /// unique email address of account
    pub email: String,
    /// unique identifier of account
    pub id: String,
}

/// AppTransferRecipient
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct AppTransferRecipient {
    /// unique email address of account
    pub email: String,
    /// unique identifier of an account
    pub id: String,
}

/// Credit
/// 
/// Stability: development
/// 
/// A credit represents value that will be used up before further charges are assigned to an account.
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#credit)
/// 
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Credit {
    /// total value of credit in cents
    pub amount: i64,
    /// remaining value of credit in cents
    pub balance: i64,
    /// when credit was created
    pub created_at: String,
    /// when credit will expire
    pub expires_at: String,
    /// unique identifier of credit
    pub id: String,
    /// a name for credit
    pub title: String,
    /// when credit was updated
    pub updated_at: String,
}
