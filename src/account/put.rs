//! Access the Account portion of the Heroku API
imports!();
use crate::client::PutQueryBuilder;

// Declaration of types representing the various items under /account and users/account
new_type!(
    Account
    AccountInvoiceAddress
);

// From implementations for conversion
from!(
    @PutQueryBuilder
        -> Account = "account"
    @Account 
        -> AccountInvoiceAddress = "invoice-address"
);

// impls of each type
impl_macro!(
    @Account
        |=> account_invoice_address -> AccountInvoiceAddress
        |
);

exec!(Account);
exec!(AccountInvoiceAddress);
 