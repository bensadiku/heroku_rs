//! Access the Account portion of the Heroku API
imports!();
use crate::client::PostQueryBuilder;

// Declaration of types representing the various items under /account and users/account
new_type!(
    Accounts
    Account
    UserAccount
    AccountAppTransfers
    AccountCredits
    UserAccountSmsNumber
    UserAccountSmsNumberActionConfirm
    UserAccountSmsNumberActionRecover
);

// From implementations for conversion
from!(
    @PostQueryBuilder
        -> Account = "account"
        -> Accounts = "users"
    @Account 
        -> AccountAppTransfers = "app-transfers"
        -> AccountCredits = "credits"
    @Accounts
        => UserAccount
    @UserAccount
        -> UserAccountSmsNumber = "sms-number"
    @UserAccountSmsNumber
        -> UserAccountSmsNumberActionConfirm = "actions/confirm"
        -> UserAccountSmsNumberActionRecover = "actions/recover"
    
);

// impls of each type
impl_macro!(
    @Account
        |=> account_app_tranfer -> AccountAppTransfers
        |=> account_credit -> AccountCredits
        |
    @Accounts
        |
        |=> account_email -> UserAccount = account_email_str
        |=> account_id -> UserAccount = account_id_str
    @UserAccount
        |=> account_sms_number -> UserAccountSmsNumber
        |
    @UserAccountSmsNumber
        |=> action_confirm -> UserAccountSmsNumberActionConfirm
        |=> action_recover -> UserAccountSmsNumberActionRecover
        |
    
);

exec!(Account);
exec!(Accounts);
exec!(UserAccount);
exec!(AccountAppTransfers);
exec!(AccountCredits);
exec!(UserAccountSmsNumber);
exec!(UserAccountSmsNumberActionConfirm);
exec!(UserAccountSmsNumberActionRecover);
 