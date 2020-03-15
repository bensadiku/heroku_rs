//! Access the Account portion of the Heroku API
imports!();
use crate::client::GetQueryBuilder;

// Declaration of types representing the various items under /account and users/account
new_type!(
    Accounts
    Account
    UserAccount
    AccountFeatures
    AccountFeatureId
    AccountFeatureName
    AccountAppTransfers
    AccountAppTransferId
    AccountAppTransferName
    AccountCredits
    AccountCreditId
    AccountInvoices
    AccountInvoiceId
    AccountInvoiceAddress
    AccountKeys
    AccountKeyFingerprint
    AccountKeyId
    AccountRateLimits
    UserAccountAddons
    UserAccountApps
    UserAccountSmsNumber
);

// From implementations for conversion
from!(
    @GetQueryBuilder
        -> Account = "account"
        -> Accounts = "users"
    @Account 
        -> AccountFeatures = "features"
        -> AccountAppTransfers = "app-transfers"
        -> AccountCredits = "credits"
        -> AccountInvoices = "invoices"
        -> AccountInvoiceAddress = "invoice-address"
        -> AccountKeys = "keys"
        -> AccountRateLimits = "rate-limits"
    @AccountFeatures
        => AccountFeatureId
        => AccountFeatureName
    @AccountAppTransfers
        => AccountAppTransferId
        => AccountAppTransferName
    @Accounts
        => UserAccount
    @AccountCredits
        => AccountCreditId
    @AccountInvoices
        => AccountInvoiceId
    @AccountKeys
        => AccountKeyFingerprint
        => AccountKeyId
    @UserAccount
        -> UserAccountAddons = "addons"
        -> UserAccountApps = "apps"
        -> UserAccountSmsNumber = "sms-number"
    
);

// impls of each type
impl_macro!(
    @Account
        |=> account_features -> AccountFeatures
        |=> account_app_tranfers -> AccountAppTransfers
        |=> account_credits -> AccountCredits
        |=> account_invoices -> AccountInvoices
        |=> account_invoice_address -> AccountInvoiceAddress
        |=> account_keys -> AccountKeys
        |=> account_rate_limits -> AccountRateLimits
        |
    @AccountFeatures
        |
        |=> feature_name -> AccountFeatureName = feature_name_str
        |=> feature_id -> AccountFeatureId = feature_id_str
    @AccountAppTransfers
        |
        |=> transfer_name -> AccountAppTransferName = transfer_name_str
        |=> transfer_id -> AccountAppTransferId = transfer_id_str
    @AccountCredits
        |
        |=> credit_id -> AccountCreditId = credit_id_str
    @AccountInvoices
        |
        |=> invoice_id -> AccountInvoiceId = invoice_id_str
    @AccountKeys
        |
        |=> key_fingerprint -> AccountKeyFingerprint = key_fingerprint_str
        |=> key_id -> AccountKeyId = key_id_str
    @Accounts
        |
        |=> account_email -> UserAccount = account_email_str
        |=> account_id -> UserAccount = account_id_str
    @UserAccount
        |=> account_addons -> UserAccountAddons
        |=> account_apps -> UserAccountApps
        |=> account_sms_number -> UserAccountSmsNumber
        |
    
);

exec!(Account);
exec!(AccountFeatures);
exec!(AccountFeatureId);
exec!(AccountFeatureName);
exec!(Accounts);
exec!(UserAccount);
exec!(AccountAppTransfers);
exec!(AccountAppTransferId);
exec!(AccountAppTransferName);
exec!(AccountCredits);
exec!(AccountCreditId);
exec!(AccountInvoices);
exec!(AccountInvoiceId);
exec!(AccountInvoiceAddress);
exec!(AccountKeys);
exec!(AccountKeyId);
exec!(AccountKeyFingerprint);
exec!(AccountRateLimits);
exec!(UserAccountAddons);
exec!(UserAccountApps);
exec!(UserAccountSmsNumber);
 