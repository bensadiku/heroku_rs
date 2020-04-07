//Anything related to PATCH requests for account and it's properties goes here.
use super::{Account, AccountFeature, AppTransfer};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Account Update
///
/// Update account.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-update)
pub struct AccountUpdate {
    pub params: AccountUpdateParams,
}

impl AccountUpdate {
    pub fn new(
        allow_tracking: Option<bool>,
        beta: Option<bool>,
        name: Option<String>,
    ) -> AccountUpdate {
        AccountUpdate {
            params: AccountUpdateParams {
                allow_tracking,
                beta,
                name,
            },
        }
    }

    pub fn create() -> AccountUpdate {
        AccountUpdate {
            params: AccountUpdateParams {
                allow_tracking: None,
                beta: None,
                name: None,
            },
        }
    }
}

/// Update account with parameters.
///
/// All three paramemters are optional.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-update-optional-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct AccountUpdateParams {
    /// whether to allow third party web activity tracking, by default: true
    pub allow_tracking: Option<bool>,
    /// whether allowed to utilize beta Heroku features
    pub beta: Option<bool>,
    /// full name of the account owner
    pub name: Option<String>,
}

impl HerokuEndpoint<Account, (), AccountUpdateParams> for AccountUpdate {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("account",)
    }
    fn body(&self) -> Option<AccountUpdateParams> {
        Some(self.params.clone())
    }
}

/// Account Update By User
///
/// Update account.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-update-by-user)
pub struct UserAccountUpdate {
    /// account_id can be the account email or id.
    pub account_id: String,
    /// The parameters to pass to the Heroku API
    pub params: UserAccountUpdateParams,
}

impl UserAccountUpdate {
    pub fn new(
        account_id: String,
        allow_tracking: Option<bool>,
        beta: Option<bool>,
        name: Option<String>,
    ) -> UserAccountUpdate {
        UserAccountUpdate {
            account_id,
            params: UserAccountUpdateParams {
                allow_tracking,
                beta,
                name,
            },
        }
    }

    pub fn create(account_id: String) -> UserAccountUpdate {
        UserAccountUpdate {
            account_id,
            params: UserAccountUpdateParams {
                allow_tracking: None,
                beta: None,
                name: None,
            },
        }
    }
}

/// Update user account with parameters.
///
/// All three paramemters are optional.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-update-by-user-optional-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct UserAccountUpdateParams {
    /// whether to allow third party web activity tracking, by default: true
    pub allow_tracking: Option<bool>,
    /// whether allowed to utilize beta Heroku features
    pub beta: Option<bool>,
    /// full name of the account owner
    pub name: Option<String>,
}

impl HerokuEndpoint<Account, (), UserAccountUpdateParams> for UserAccountUpdate {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("users/{}", self.account_id)
    }
    fn body(&self) -> Option<UserAccountUpdateParams> {
        Some(self.params.clone())
    }
}

/// Account Feature Update
///
/// Update an existing account feature.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-feature-update)
pub struct AccountFeatureUpdate {
    /// feature_id can be the feature name or id.
    pub feature_id: String,
    /// The parameters to pass to the Heroku API
    pub params: AccountFeatureUpdateParams,
}

impl AccountFeatureUpdate {
    pub fn new(feature_id: String, enabled: bool) -> AccountFeatureUpdate {
        AccountFeatureUpdate {
            feature_id,
            params: AccountFeatureUpdateParams { enabled },
        }
    }
}

/// Update account feature with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-feature-update-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct AccountFeatureUpdateParams {
    /// whether or not account feature has been enabled
    pub enabled: bool,
}

impl HerokuEndpoint<AccountFeature, (), AccountFeatureUpdateParams> for AccountFeatureUpdate {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("account/features/{}", self.feature_id)
    }
    fn body(&self) -> Option<AccountFeatureUpdateParams> {
        Some(self.params.clone())
    }
}

/// App Transfer Update
///
/// Update an existing app transfer.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-transfer-update)
pub struct AppTransferUpdate {
    /// unique identifier or the transfer name
    pub transfer_id: String,
    /// The parameters to pass to the Heroku API
    pub params: AppTransferUpdateParams,
}

impl AppTransferUpdate {
    pub fn new(transfer_id: String, state: String) -> AppTransferUpdate {
        AppTransferUpdate {
            transfer_id,
            params: AppTransferUpdateParams { state },
        }
    }
}

/// Update account app transfer with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-transfer-update-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct AppTransferUpdateParams {
    /// the current state of an app transfer, one of:"pending" or "accepted" or "declined"
    pub state: String,
}

impl HerokuEndpoint<AppTransfer, (), AppTransferUpdateParams> for AppTransferUpdate {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("account/app-transfers/{}", self.transfer_id)
    }
    fn body(&self) -> Option<AppTransferUpdateParams> {
        Some(self.params.clone())
    }
}
