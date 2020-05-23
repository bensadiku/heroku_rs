//Anything related to PUT requests for account and it's properties goes here.
use super::InvoiceAddress;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Invoice Address update
///
/// Update invoice address for an account.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#invoice-address-update)
///
/// # Example:
///
/// InvoiceAddressUpdate has no required parameters, and returns the [`InvoiceAddress`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
/// 
/// let update_invoice = &InvoiceAddressUpdate::new()
///     .address_1("Grove Street")
///     .address_2("Not Grove Street")
///     .city("Seattle")
///     .country("US")
///     .other("Company ABC Inc. VAT 903820")
///     .postal_code("98101")
///     .state("WA")
///     .use_invoice_address(true)
///     .build();
/// let response = api_client.request(update_invoice);
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
pub struct InvoiceAddressUpdate<'a> {
    /// The parameters to pass to the Heroku API
    pub params: InvoiceAddressUpdateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> InvoiceAddressUpdate<'a> {
    pub fn new() -> InvoiceAddressUpdate<'a> {
        InvoiceAddressUpdate {
            params: InvoiceAddressUpdateParams {
                address_1: None,
                address_2: None,
                city: None,
                country: None,
                other: None,
                postal_code: None,
                state: None,
                use_invoice_address: None,
            },
        }
    }

    /// # address_1: invoice street address line 1
    pub fn address_1(&mut self, address_1: &'a str) -> &mut Self {
        self.params.address_1 = Some(address_1);
        self
    }
    /// # address_2: invoice street address line 2
    pub fn address_2(&mut self, address_2: &'a str) -> &mut Self {
        self.params.address_2 = Some(address_2);
        self
    }
    /// # city: invoice city
    pub fn city(&mut self, city: &'a str) -> &mut Self {
        self.params.city = Some(city);
        self
    }
    /// # country: country
    pub fn country(&mut self, country: &'a str) -> &mut Self {
        self.params.country = Some(country);
        self
    }
    /// # other: metadata / additional information to go on invoice
    pub fn other(&mut self, other: &'a str) -> &mut Self {
        self.params.country = Some(other);
        self
    }
    /// # postal_code: invoice zip code
    pub fn postal_code(&mut self, postal_code: &'a str) -> &mut Self {
        self.params.postal_code = Some(postal_code);
        self
    }
    /// # state: invoice state
    pub fn state(&mut self, state: &'a str) -> &mut Self {
        self.params.state = Some(state);
        self
    }
    /// # use_invoice_address: flag to use the invoice address for an account or not
    pub fn use_invoice_address(&mut self, use_invoice_address: bool) -> &mut Self {
        self.params.use_invoice_address = Some(use_invoice_address);
        self
    }

    pub fn build(&self) -> InvoiceAddressUpdate<'a> {
        InvoiceAddressUpdate {
            params: InvoiceAddressUpdateParams {
                address_1: self.params.address_1,
                address_2: self.params.address_2,
                city: self.params.city,
                country: self.params.country,
                other: self.params.other,
                postal_code: self.params.postal_code,
                state: self.params.state,
                use_invoice_address: self.params.use_invoice_address,
            },
        }
    }
}

/// Update account invoice address with optional parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#invoice-address-update-optional-parameters)
#[serde_with::skip_serializing_none]
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
    pub use_invoice_address: Option<bool>,
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
