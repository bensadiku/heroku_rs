//Anything related to DELETE requests for domains and it's properties goes here.
use super::Domain;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Domain Delete
///
/// Delete an existing domain
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#domain-delete)
///
/// # Example:
///
/// DomainDelete takes two required parameters, app_id and domain_id, and returns the deleted [`Domain`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&DomainDelete::new("APP_ID", "DOMAIN_ID"));
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
/// [response]: ../struct.Domain.html
pub struct DomainDelete<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// domain_id can be the domain hostname or id.
    pub domain_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> DomainDelete<'a> {
    pub fn new(app_id: &'a str, domain_id: &'a str) -> DomainDelete<'a> {
        DomainDelete { app_id, domain_id }
    }
}

impl<'a> HerokuEndpoint<Domain> for DomainDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/domains/{}", self.app_id, self.domain_id)
    }
}
