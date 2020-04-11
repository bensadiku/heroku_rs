//Anything related to PUT requests for account and it's properties goes here.
use super::InvoiceAddress;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Invoice Address update
///
/// Update invoice address for an account.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#invoice-address-update)
pub struct InvoiceAddressUpdate<'a> {
    pub params: InvoiceAddressUpdateParams<'a>,
}

impl<'a> InvoiceAddressUpdate<'a> {
    pub fn new(
        address_1: Option<&'a str>,
        address_2: Option<&'a str>,
        city: Option<&'a str>,
        country: Option<&'a str>,
        other: Option<&'a str>,
        postal_code: Option<&'a str>,
        state: Option<&'a str>,
        use_invoice_address: bool,
    ) -> InvoiceAddressUpdate<'a> {
        InvoiceAddressUpdate {
            params: InvoiceAddressUpdateParams {
                address_1,
                address_2,
                city,
                country,
                other,
                postal_code,
                state,
                use_invoice_address,
            },
        }
    }
}

/// Update account invoice address with optional parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#invoice-address-update-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct InvoiceAddressUpdateParams<'a> {
    /// invoice street address line 1
    pub address_1: Option<&'a str>,
    /// invoice street address line 2
    pub address_2: Option<&'a str>,
    /// invoice city
    pub city: Option<&'a str>,
    /// country
    pub country: Option<&'a str>,
    /// metadata / additional information to go on invoice
    pub other: Option<&'a str>,
    /// invoice zip code
    pub postal_code: Option<&'a str>,
    /// invoice state
    pub state: Option<&'a str>,
    /// flag to use the invoice address for an account or not
    pub use_invoice_address: bool,
}
impl<'a> HerokuEndpoint<InvoiceAddress, (), InvoiceAddressUpdateParams<'a>>
    for InvoiceAddressUpdate<'a>
{
    fn method(&self) -> Method {
        Method::Put
    }
    fn path(&self) -> String {
        format!("account/invoice-address")
    }
    fn body(&self) -> Option<InvoiceAddressUpdateParams<'a>> {
        Some(self.params.clone())
    }
}
