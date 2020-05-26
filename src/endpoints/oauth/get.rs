//Anything related to GET requests for oauth authorizations and it's properties goes here.
use super::{OAuth, OAuthClient};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// OAuth Authorization Info
///
/// Info for an OAuth authorization.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-authorization-info)
///
/// # Example:
///
/// OAuthDetails takes one required parameter, oauth_id, and returns the [`OAuth`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&OAuthDetails::new("OAUTH_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
///
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.OAuth.html
pub struct OAuthDetails<'a> {
    /// oauth_id is the unique identifier.
    pub oauth_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> OAuthDetails<'a> {
    pub fn new(oauth_id: &'a str) -> OAuthDetails<'a> {
        OAuthDetails { oauth_id }
    }
}

impl<'a> HerokuEndpoint<OAuth> for OAuthDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("oauth/authorizations/{}", self.oauth_id)
    }
}

/// OAuth Authorization List
///
/// List OAuth authorizations.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-authorization-list)
/// 
/// # Example:
///
/// OAuthList takes no required parameters, and returns a list of [`OAuth`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&OAuthList::new());
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
///
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.OAuth.html
pub struct OAuthList {}

#[cfg(feature = "builder")]
impl OAuthList {
    pub fn new() -> OAuthList {
        OAuthList {}
    }
}

impl HerokuEndpoint<Vec<OAuth>> for OAuthList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("oauth/authorizations")
    }
}

/// OAuth Client Info
///
/// Info for an OAuth client
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-client-info)
///
/// # Example:
///
/// OAuthClientDetails takes one required parameter, client_id, and returns the [`OAuthClient`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&OAuthClientDetails::new("CLIENT_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
///
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.OAuthClient.html
pub struct OAuthClientDetails<'a> {
    /// unique identifier of OAuth Client authorization
    pub client_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> OAuthClientDetails<'a> {
    pub fn new(client_id: &'a str) -> OAuthClientDetails<'a> {
        OAuthClientDetails { client_id }
    }
}

impl<'a> HerokuEndpoint<OAuthClient> for OAuthClientDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("oauth/clients/{}", self.client_id)
    }
}

/// OAuth Client List
///
/// List OAuth clients
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-client-list)
/// 
/// # Example:
///
/// OAuthClientList takes no required parameters, and returns a list of [`OAuthClient`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&OAuthClientList::new());
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
///
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.OAuthClient.html
pub struct OAuthClientList {}

#[cfg(feature = "builder")]
impl OAuthClientList {
    pub fn new() -> OAuthClientList {
        OAuthClientList {}
    }
}

impl HerokuEndpoint<Vec<OAuthClient>> for OAuthClientList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("oauth/clients")
    }
}
