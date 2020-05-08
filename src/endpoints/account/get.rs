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
pub struct UserAccountDetails<'a> {
    /// account_id can be the account email or id.
    pub account_id: &'a str,
}

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
pub struct AccountFeatureDetails<'a> {
    /// feature_id can be the feature name or id.
    pub feature_id: &'a str,
}

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
pub struct AppTransferDetails<'a> {
    /// transfer_id can be the transfer name or id.
    pub transfer_id: &'a str,
}

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
pub struct AccountCreditDetails<'a> {
    /// credit_id is the credit identifier.
    pub credit_id: &'a str,
}

impl<'a> AccountCreditDetails<'a> {
    pub fn new(credit_id: &'a str) -> AccountCreditDetails {
        AccountCreditDetails { credit_id }
    }
}

impl<'a> HerokuEndpoint<AppTransfer> for AccountCreditDetails<'a> {
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

/// SMS NUMBER
///
/// Get sms number by account id or email
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#sms-number-sms-number)
pub struct SmsNumberDetails<'a> {
    /// unique identifier, email or account id
    pub account_id: &'a str,
}

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
pub struct InvoiceDetails<'a> {
    /// invoice number
    pub invoice_id: &'a str,
}

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
pub struct InvoiceList {}

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
pub struct InvoiceAddressDetails {}

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
pub struct KeyDetails<'a> {
    /// unique key identifier, either key_id or fingerprint
    pub key_id: &'a str,
}

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
pub struct KeyList {}

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
