//Anything related to GET requests for account and it's properties goes here.
use super::{Account, AccountFeature, AppTransfer, Credit};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Info for account.
/// No parameters required.
/// https://devcenter.heroku.com/articles/platform-api-reference#account-info
pub struct AccountDetails {}

impl HerokuEndpoint<Account> for AccountDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("account")
    }
}

/// Account Info By User
/// https://devcenter.heroku.com/articles/platform-api-reference#account-info
pub struct UserAccountDetails {
    /// identifier can be the account email or id.
    pub account_identifier: String,
}

impl HerokuEndpoint<Account> for UserAccountDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("users/{}", self.account_identifier)
    }
}

/// Account Feature List.
/// List existing account features.
/// https://devcenter.heroku.com/articles/platform-api-reference#account-feature-list
pub struct AccountFeatureList {}

impl HerokuEndpoint<Vec<AccountFeature>> for AccountFeatureList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("account/features")
    }
}

/// Account Feature Info.
/// Info for an existing account feature.
/// https://devcenter.heroku.com/articles/platform-api-reference#account-feature-info
pub struct AccountFeatureDetails {
    /// identifier can be the feature name or id.
    pub account_feature_id: String,
}

impl HerokuEndpoint<AccountFeature> for AccountFeatureDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("account/features/{}", self.account_feature_id)
    }
}

/// App Transfer List
/// List existing apps transfers.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-transfer-list
pub struct AppTransferList {}

impl HerokuEndpoint<Vec<AppTransfer>> for AppTransferList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("account/app-transfers")
    }
}

/// App Transfer Info
/// Info for existing app transfer.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-transfer-info
pub struct AppTransferDetails {
    /// identifier can be the transfer name or id.
    pub transfer_id: String,
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
/// Info for existing credit.
/// https://devcenter.heroku.com/articles/platform-api-reference#credit-info
pub struct AccountCreditDetails {
    /// credit_id is the credit identifier.
    pub credit_id: String,
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
/// List existing credits.
/// https://devcenter.heroku.com/articles/platform-api-reference#credit-list
pub struct AccountCreditList {}

impl HerokuEndpoint<Vec<Credit>> for AccountCreditList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("account/credits")
    }
}