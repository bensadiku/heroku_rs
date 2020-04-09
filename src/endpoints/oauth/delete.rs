//Anything related to DELETE requests for oauth authorizations and it's properties goes here.
use super::{OAuth, OAuthClient, OAuthToken};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// OAuth Authorization Delete
///
/// Delete OAuth authorization.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-authorization-delete)
pub struct OAuthDelete {
    /// unique identifier of OAuth authorization
    pub oauth_id: String,
}

impl OAuthDelete {
    pub fn new(oauth_id: String) -> OAuthDelete {
        OAuthDelete { oauth_id }
    }
}

impl HerokuEndpoint<OAuth, (), ()> for OAuthDelete {
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
pub struct OAuthClientDelete {
    /// unique identifier of OAuth Client authorization
    pub client_id: String,
}

impl OAuthClientDelete {
    pub fn new(client_id: String) -> OAuthClientDelete {
        OAuthClientDelete { client_id }
    }
}

impl HerokuEndpoint<OAuthClient, (), ()> for OAuthClientDelete {
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
pub struct OAuthTokenDelete {
    /// unique identifier of token
    pub token_id: String,
}

impl OAuthTokenDelete {
    pub fn new(token_id: String) -> OAuthTokenDelete {
        OAuthTokenDelete { token_id }
    }
}

impl HerokuEndpoint<OAuthToken, (), ()> for OAuthTokenDelete {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("oauth/tokens/{}", self.token_id)
    }
}
