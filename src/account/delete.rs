//! Access the Account portion of the Heroku API
imports!();
use crate::client::DeleteQueryBuilder;

// Declaration of types representing the various items under /account and users/account
new_type!(
    Accounts
    Account
    UserAccount
    AccountAppTransfers
    AccountAppTransferId
    AccountAppTransferName
);

// From implementations for conversion
from!(
    @DeleteQueryBuilder
        -> Account = "account"
        -> Accounts = "users"
    @Account 
        -> AccountAppTransfers = "app-transfers"
    @Accounts
        => UserAccount
    @AccountAppTransfers
        => AccountAppTransferId
        => AccountAppTransferName
    
);

// impls of each type
impl_macro!(
    @Account
        |=> account_app_tranfer -> AccountAppTransfers
        |
    @Accounts
        |
        |=> account_email -> UserAccount = account_email_str
        |=> account_id -> UserAccount = account_id_str
    @AccountAppTransfers
        |
        |=> transfer_id -> AccountAppTransferId = transfer_id_str
        |=> transfer_name -> AccountAppTransferName = transfer_name_str
    
);

exec!(Account);
exec!(Accounts);
exec!(UserAccount);
exec!(AccountAppTransfers);
exec!(AccountAppTransferId);
exec!(AccountAppTransferName);