//Anything related to GET requests for account and it's properties goes here.
use super::Account;

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
