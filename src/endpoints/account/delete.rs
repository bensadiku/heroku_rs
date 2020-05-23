//Anything related to DELETE requests for account and it's properties goes here.
use super::{Account, AppTransfer};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Account Delete.
///
/// Delete account. Note that this action cannot be undone.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-delete)
///
/// # Example:
///
/// AccountDelete takes no parameters, and returns the deleted [`account`][response] struct.
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
///let response = api_client.request(&AccountDelete::new());
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
///
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.Account.html
pub struct AccountDelete {}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// UserAccountDelete takes account_id as parameter, and returns the deleted [`account`][response] struct.
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///let account_id = "USER_ID_OR_EMAIL";
///let response = api_client.request(&UserAccountDelete::new( account_id ));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.Account.html
pub struct UserAccountDelete<'a> {
    /// account_id can be the account email or id.
    pub account_id: &'a str,
}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// AppTransferDelete takes transfer_id as parameter, and returns the deleted [`AppTransfer`][response] struct.
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///let transfer_id = "transfer_id";
///let response = api_client.request(&AppTransferDelete::new( transfer_id ));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.AppTransfer.html
pub struct AppTransferDelete<'a> {
    /// transfer_id can be the transfer name or id.
    pub transfer_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AppTransferDelete<'a> {
    pub fn new(transfer_id: &'a str) -> AppTransferDelete {
        AppTransferDelete { transfer_id }
    }
}

impl<'a> HerokuEndpoint<AppTransfer> for AppTransferDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("account/app-transfers/{}", self.transfer_id)
    }
}
