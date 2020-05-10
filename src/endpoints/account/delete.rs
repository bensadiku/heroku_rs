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
pub struct UserAccountDelete<'a> {
    /// account_id can be the account email or id.
    pub account_id: &'a str,
}

impl<'a> UserAccountDelete<'a> {
    pub fn new(account_id: &'a str) -> UserAccountDelete<'a> {
        UserAccountDelete { account_id }
    }
}

impl<'a> HerokuEndpoint<Account> for UserAccountDelete<'a> {
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
pub struct AppTransferDelete<'a> {
    /// transfer_id can be the transfer name or id.
    pub transfer_id: &'a str,
}

impl<'a> AppTransferDelete<'a> {
    pub fn new(transfer_id: &'a str) -> AppTransferDelete {
        AppTransferDelete { transfer_id }
    }
}

impl<'a> HerokuEndpoint<Account> for AppTransferDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("account/app-transfers/{}", self.transfer_id)
    }
}
