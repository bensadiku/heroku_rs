//Anything related to GET requests for account and it's properties goes here.
use super::{
    Account, AccountFeature, AppTransfer, Credit, Invoice, InvoiceAddress, Key, SmsNumber,
};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Account Info
///
/// Info for account.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#account-info)
///
/// # Example:
///
/// AccountDetails takes no parameters, and returns the [`account`][response] struct.
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///let response = api_client.request(&AccountDetails::new());
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
pub struct AccountDetails {}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// UserAccountDetails takes one parameter account_id, and returns the [`account`][response] struct.
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///let account_id = "USER_ID_OR_EMAIL";
///let response = api_client.request(&UserAccountDetails::new(account_id));
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
pub struct UserAccountDetails<'a> {
    /// account_id can be the account email or id.
    pub account_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> UserAccountDetails<'a> {
    pub fn new(account_id: &'a str) -> UserAccountDetails {
        UserAccountDetails { account_id }
    }
}

impl<'a> HerokuEndpoint<Account> for UserAccountDetails<'a> {
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
///
/// # Example:
///
/// AccountFeatureList takes no parameters, and returns a list of [`account features`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///let response = api_client.request(&AccountFeatureList::new());
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
/// [response]: ../struct.AccountFeature.html
pub struct AccountFeatureList {}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// AccountFeatureDetails takes feature_id parameter, and returns a [`account feature`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///let feature_id = "FEATURE_NAME_OR_ID";
///let response = api_client.request(&AccountFeatureDetails::new(feature_id));
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
/// [response]: ../struct.AccountFeature.html
pub struct AccountFeatureDetails<'a> {
    /// feature_id can be the feature name or id.
    pub feature_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AccountFeatureDetails<'a> {
    pub fn new(feature_id: &'a str) -> AccountFeatureDetails {
        AccountFeatureDetails { feature_id }
    }
}

impl<'a> HerokuEndpoint<AccountFeature> for AccountFeatureDetails<'a> {
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
///
/// # Example:
///
/// AppTransferList takes no parameters, and returns a list of [`app transfer`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///let response = api_client.request(&AppTransferList::new());
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
pub struct AppTransferList {}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// AppTransferDetails takes transfer_id parameter, and returns a [`app transfer`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
/// 
///let transfer_id = "TRANSFER_NAME_OR_ID";
///let response = api_client.request(&AppTransferDetails::new(transfer_id));
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
pub struct AppTransferDetails<'a> {
    /// transfer_id can be the transfer name or id.
    pub transfer_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AppTransferDetails<'a> {
    pub fn new(transfer_id: &'a str) -> AppTransferDetails {
        AppTransferDetails { transfer_id }
    }
}

impl<'a> HerokuEndpoint<AppTransfer> for AppTransferDetails<'a> {
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
///
/// # Example:
///
/// AccountCreditDetails takes credit_id parameter, and returns a [`credit`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
/// 
///let credit_id = "CREDIT_ID";
///let response = api_client.request(&AccountCreditDetails::new(credit_id));
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
/// [response]: ../struct.Credit.html
pub struct AccountCreditDetails<'a> {
    /// credit_id is the credit identifier.
    pub credit_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> AccountCreditDetails<'a> {
    pub fn new(credit_id: &'a str) -> AccountCreditDetails {
        AccountCreditDetails { credit_id }
    }
}

impl<'a> HerokuEndpoint<Credit> for AccountCreditDetails<'a> {
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
///
/// # Example:
///
/// AccountCreditList takes no parameters, and returns a list of [`credits`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
/// 
///let response = api_client.request(&AccountCreditList::new());
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
/// [response]: ../struct.Credit.html
pub struct AccountCreditList {}

#[cfg(feature = "builder")]
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

/// Sms Number
///
/// Get sms number by account id or email
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#sms-number-sms-number)
///
/// # Example:
///
/// SmsNumberDetails takes account_id parameter, and returns a [`sms number`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///let account_id = "ACCOUNT_EMAIL_OR_ID";
/// 
///let response = api_client.request(&SmsNumberDetails::new(account_id));
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
/// [response]: ../struct.SmsNumber.html
pub struct SmsNumberDetails<'a> {
    /// unique identifier, email or account id
    pub account_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> SmsNumberDetails<'a> {
    pub fn new(account_id: &'a str) -> SmsNumberDetails {
        SmsNumberDetails { account_id }
    }
}

impl<'a> HerokuEndpoint<SmsNumber> for SmsNumberDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("users/{}/sms-number", self.account_id)
    }
}

/// Invoice Info
///
/// Info for existing invoice.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#invoice-info)
///
/// # Example:
///
/// InvoiceDetails takes invoice_id parameter, and returns a [`Invoice`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///let invoice_id = "INVOICE_NUMBER";
/// 
///let response = api_client.request(&InvoiceDetails::new(invoice_id));
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
/// [response]: ../struct.Invoice.html
pub struct InvoiceDetails<'a> {
    /// invoice number
    pub invoice_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> InvoiceDetails<'a> {
    pub fn new(invoice_id: &'a str) -> InvoiceDetails {
        InvoiceDetails { invoice_id }
    }
}

impl<'a> HerokuEndpoint<Invoice> for InvoiceDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("account/invoices/{}", self.invoice_id)
    }
}

/// Invoice List
///
/// List existing invoices.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#invoice-list)
///
/// # Example:
///
/// InvoiceList takes no parameters, and returns a list of [`Invoices`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
/// 
///let response = api_client.request(&InvoiceList::new());
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
/// [response]: ../struct.Invoice.html
pub struct InvoiceList {}

#[cfg(feature = "builder")]
impl InvoiceList {
    pub fn new() -> InvoiceList {
        InvoiceList {}
    }
}

impl HerokuEndpoint<Vec<Invoice>> for InvoiceList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("account/invoices")
    }
}

/// Invoice Address info
///
/// Retrieve existing invoice address.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#invoice-address-info)
///
/// # Example:
///
/// InvoiceAddressDetails takes no parameters, and returns a [`InvoiceAddress`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
/// 
///let response = api_client.request(&InvoiceAddressDetails::new());
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
/// [response]: ../struct.InvoiceAddress.html
pub struct InvoiceAddressDetails {}

#[cfg(feature = "builder")]
impl InvoiceAddressDetails {
    pub fn new() -> InvoiceAddressDetails {
        InvoiceAddressDetails {}
    }
}

impl HerokuEndpoint<InvoiceAddress> for InvoiceAddressDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("account/invoice-address")
    }
}

/// Key Info
///
/// Info for existing key.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#key-info)
///
/// # Example:
///
/// KeyDetails takes key_id parameter, and returns a [`Key`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///let key_id = "KEY_ID_OR_FINGERPRINT";
/// 
///let response = api_client.request(&KeyDetails::new(key_id));
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
/// [response]: ../struct.Key.html
pub struct KeyDetails<'a> {
    /// unique key identifier, either key_id or fingerprint
    pub key_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> KeyDetails<'a> {
    pub fn new(key_id: &'a str) -> KeyDetails {
        KeyDetails { key_id }
    }
}

impl<'a> HerokuEndpoint<Key> for KeyDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("account/keys/{}", self.key_id)
    }
}

/// Key List
///
/// List existing keys.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#key-list)
///
/// # Example:
///
/// KeyList takes key_id parameter, and returns a list of [`Keys`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
/// 
///let response = api_client.request(&KeyList::new());
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
/// [response]: ../struct.Key.html
pub struct KeyList {}

#[cfg(feature = "builder")]
impl KeyList {
    pub fn new() -> KeyList {
        KeyList {}
    }
}

impl HerokuEndpoint<Vec<Key>> for KeyList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("account/keys")
    }
}
