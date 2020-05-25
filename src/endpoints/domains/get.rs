//Anything related to GET requests for domains and it's properties goes here.
use super::Domain;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Domain Info
///
/// Info for existing domain.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#domain-info)
///
/// # Example:
///
/// DomainDetails takes two required parameters, app_id and domain_id, and returns the [`Domain`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&DomainDetails::new("APP_ID", "DOMAIN_ID"));
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
pub struct DomainDetails<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// domain_id can be the domain hostname or id.
    pub domain_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> DomainDetails<'a> {
    pub fn new(app_id: &'a str, domain_id: &'a str) -> DomainDetails<'a> {
        DomainDetails { app_id, domain_id }
    }
}

impl<'a> HerokuEndpoint<Domain> for DomainDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/domains/{}", self.app_id, self.domain_id)
    }
}

/// Domain List
///
/// List existing domains.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#domain-list)
///
/// # Example:
///
/// DomainDetails takes one required parameter, app_id, and returns a list of [`Domains`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
///
/// let response = api_client.request(&DomainList::new("APP_ID"));
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
pub struct DomainList<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> DomainList<'a> {
    pub fn new(app_id: &'a str) -> DomainList<'a> {
        DomainList { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<Domain>> for DomainList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/domains", self.app_id)
    }
}
