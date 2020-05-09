//Anything related to PATCH requests for oauth authorizations and it's properties goes here.
use super::OAuthClient;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// OAuth Client Update
///
/// Update OAuth client
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-client-update)
pub struct OAuthClientUpdate<'a> {
    /// unique identifier of OAuth Client authorization
    pub client_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: OAuthClientUpdateParams<'a>,
}

impl<'a> OAuthClientUpdate<'a> {
    pub fn new(client_id: &'a str) -> OAuthClientUpdate<'a> {
        OAuthClientUpdate {
            client_id,
            params: OAuthClientUpdateParams {
                name: None,
                redirect_uri: None,
            },
        }
    }

    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.params.name = Some(name);
        self
    }

    pub fn redirect_uri(&mut self, redirect_uri: &'a str) -> &mut Self {
        self.params.redirect_uri = Some(redirect_uri);
        self
    }

    pub fn build(&self) -> OAuthClientUpdate<'a> {
        OAuthClientUpdate {
            client_id: self.client_id,
            params: OAuthClientUpdateParams {
                name: self.params.name,
                redirect_uri: self.params.redirect_uri,
            },
        }
    }
}

/// Update existing oauth client with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-client-update-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct OAuthClientUpdateParams<'a> {
    /// OAuth client name
    pub name: Option<&'a str>,
    /// endpoint for redirection after authorization with OAuth client
    pub redirect_uri: Option<&'a str>,
}

impl<'a> HerokuEndpoint<OAuthClient, (), OAuthClientUpdateParams<'a>> for OAuthClientUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("oauth/clients/{}", self.client_id)
    }
    fn body(&self) -> Option<OAuthClientUpdateParams<'a>> {
        Some(self.params.clone())
    }
}
