//Anything related to GET requests for account and it's properties goes here.
use super::{Account, AccountFeature, AppTransfer, Credit};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Account Info
///
/// Info for account.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-info)
pub struct AccountDetails {}

impl AccountDetails {
    pub fn new() -> AccountDetails {
        AccountDetails {}
    }
}

impl HerokuEndpoint<Account> for AccountDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("account")
    }
}

/// Account Info By User
///
/// Info for account.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference##account-info-by-user)
pub struct UserAccountDetails {
    /// account_id can be the account email or id.
    pub account_id: String,
}

impl UserAccountDetails {
    pub fn new(account_id: String) -> UserAccountDetails {
        UserAccountDetails { account_id }
    }
}

impl HerokuEndpoint<Account> for UserAccountDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("users/{}", self.account_id)
    }
}

/// Account Feature List.
///
/// List existing account features.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-feature-list)
pub struct AccountFeatureList {}

impl AccountFeatureList {
    pub fn new() -> AccountFeatureList {
        AccountFeatureList {}
    }
}

impl HerokuEndpoint<Vec<AccountFeature>> for AccountFeatureList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("account/features")
    }
}

/// Account Feature Info.
///
/// Info for an existing account feature.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-feature-info)
pub struct AccountFeatureDetails {
    /// feature_id can be the feature name or id.
    pub feature_id: String,
}

impl AccountFeatureDetails {
    pub fn new(feature_id: String) -> AccountFeatureDetails {
        AccountFeatureDetails { feature_id }
    }
}

impl HerokuEndpoint<AccountFeature> for AccountFeatureDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("account/features/{}", self.feature_id)
    }
}

/// App Transfer List.
///
/// List existing apps transfers.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-transfer-list)
pub struct AppTransferList {}

impl AppTransferList {
    pub fn new() -> AppTransferList {
        AppTransferList {}
    }
}

impl HerokuEndpoint<Vec<AppTransfer>> for AppTransferList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("account/app-transfers")
    }
}

/// App Transfer Info
///
/// Info for existing app transfer.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-transfer-info)
pub struct AppTransferDetails {
    /// transfer_id can be the transfer name or id.
    pub transfer_id: String,
}

impl AppTransferDetails {
    pub fn new(transfer_id: String) -> AppTransferDetails {
        AppTransferDetails { transfer_id }
    }
}

impl HerokuEndpoint<AppTransfer> for AppTransferDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("account/app-transfers/{}", self.transfer_id)
    }
}

/// Account Credit Info
///
/// Info for existing credit.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#credit-info)
pub struct AccountCreditDetails {
    /// credit_id is the credit identifier.
    pub credit_id: String,
}

impl AccountCreditDetails {
    pub fn new(credit_id: String) -> AccountCreditDetails {
        AccountCreditDetails { credit_id }
    }
}

impl HerokuEndpoint<AppTransfer> for AccountCreditDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("account/credits/{}", self.credit_id)
    }
}

/// App Credit List
///
/// List existing credits.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#credit-list)
pub struct AccountCreditList {}

impl AccountCreditList {
    pub fn new() -> AccountCreditList {
        AccountCreditList {}
    }
}

impl HerokuEndpoint<Vec<Credit>> for AccountCreditList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("account/credits")
    }
}
