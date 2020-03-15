//! Access the Account portion of the Heroku API
imports!();
use crate::client::PatchQueryBuilder;

// Declaration of types representing the various items under /account and users/account
new_type!(
    Accounts
    Account
    UserAccount
    AccountAppTransfers
    AccountAppTransferId
    AccountAppTransferName
    AccountFeatures
    AccountFeatureId
    AccountFeatureName
);

// From implementations for conversion
from!(
    @PatchQueryBuilder
        -> Account = "account"
        -> Accounts = "users"
    @Account 
        -> AccountAppTransfers = "app-transfers"
        -> AccountFeatures = "features"
    @AccountFeatures
        => AccountFeatureId
        => AccountFeatureName
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
        |=> account_features -> AccountFeatures
        |
    @Accounts
        |
        |=> account_email -> UserAccount = account_email_str
        |=> account_id -> UserAccount = account_id_str
    @AccountFeatures
        |
        |=> feature_id -> AccountFeatureId = feature_id_str
        |=> feature_name -> AccountFeatureName = feature_name_str
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
exec!(AccountFeatures);
exec!(AccountFeatureId);
exec!(AccountFeatureName);