//Anything related to DELETE requests for account and it's properties goes here.
use super::Account;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Account Delete.
///
/// Delete account. Note that this action cannot be undone.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-delete)
pub struct AccountDelete {}

impl AccountDelete {
    pub fn new() -> AccountDelete {
        AccountDelete {}
    }
}

impl HerokuEndpoint<Account> for AccountDelete {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("account")
    }
}

/// User Account Delete.
///
/// Delete user account. Note that this action cannot be undone.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-delete-by-user)
pub struct UserAccountDelete {
    /// account_id can be the account email or id.
    pub account_id: String,
}

impl UserAccountDelete {
    pub fn new(account_id: String) -> UserAccountDelete {
        UserAccountDelete { account_id }
    }
}

impl HerokuEndpoint<Account> for UserAccountDelete {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("users/{}", self.account_id)
    }
}

/// Account Transfer Delete.
///
/// Delete an existing app transfer
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-transfer-delete)
pub struct AppTransferDelete {
    /// transfer_id can be the transfer name or id.
    pub transfer_id: String,
}

impl AppTransferDelete {
    pub fn new(transfer_id: String) -> AppTransferDelete {
        AppTransferDelete { transfer_id }
    }
}

impl HerokuEndpoint<Account> for AppTransferDelete {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("account/app-transfers/{}", self.transfer_id)
    }
}
