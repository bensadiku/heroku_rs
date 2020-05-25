//Anything related to POST requests for domains and it's properties goes here.
use super::Domain;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Domain Create
///
/// Create a new domain.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#domain-create)
///
/// # Example:
///
/// DomainCreate takes two required parameters, app_id and hostname, and returns the created [`Domain`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create(&"API_KEY").unwrap();
/// 
/// let hostname = "heroku_rs.docs.com";
/// 
/// let response = api_client.request(&DomainCreate::new("APP_ID", hostname));
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
pub struct DomainCreate<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: DomainCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> DomainCreate<'a> {
    pub fn new(app_id: &'a str, hostname: &'a str) -> DomainCreate<'a> {
        DomainCreate {
            app_id,
            params: DomainCreateParams { hostname },
        }
    }
}

/// Create domain with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#domain-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct DomainCreateParams<'a> {
    /// full hostname
    pub hostname: &'a str,
}

impl<'a> HerokuEndpoint<Domain, (), DomainCreateParams<'a>> for DomainCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/domains", self.app_id)
    }
    fn body(&self) -> Option<DomainCreateParams<'a>> {
        Some(self.params.clone())
    }
}
