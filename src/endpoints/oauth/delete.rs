//Anything related to DELETE requests for oauth authorizations and it's properties goes here.
use super::{OAuth, OAuthClient, OAuthToken};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// OAuth Authorization Delete
///
/// Delete OAuth authorization.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-authorization-delete)
pub struct OAuthDelete<'a> {
    /// unique identifier of OAuth authorization
    pub oauth_id: &'a str,
}

impl<'a> OAuthDelete<'a> {
    pub fn new(oauth_id: &'a str) -> OAuthDelete<'a> {
        OAuthDelete { oauth_id }
    }
}

impl<'a> HerokuEndpoint<OAuth> for OAuthDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("oauth/authorizations/{}", self.oauth_id)
    }
}

/// OAuth Client Delete
///
/// Delete OAuth client.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-client-delete)
pub struct OAuthClientDelete<'a> {
    /// unique identifier of OAuth Client authorization
    pub client_id: &'a str,
}

impl<'a> OAuthClientDelete<'a> {
    pub fn new(client_id: &'a str) -> OAuthClientDelete<'a> {
        OAuthClientDelete { client_id }
    }
}

impl<'a> HerokuEndpoint<OAuthClient, (), ()> for OAuthClientDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("oauth/clients/{}", self.client_id)
    }
}

/// OAuth Token Delete
///
/// Revoke OAuth access token.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-token-delete)
pub struct OAuthTokenDelete<'a> {
    /// unique identifier of token
    pub token_id: &'a str,
}

impl<'a> OAuthTokenDelete<'a> {
    pub fn new(token_id: &'a str) -> OAuthTokenDelete<'a> {
        OAuthTokenDelete { token_id }
    }
}

impl<'a> HerokuEndpoint<OAuthToken, (), ()> for OAuthTokenDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("oauth/tokens/{}", self.token_id)
    }
}
