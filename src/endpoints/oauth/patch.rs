//Anything related to PATCH requests for oauth authorizations and it's properties goes here.
use super::OAuthClient;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// OAuth Client Update
///
/// Update OAuth client
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-client-update)
pub struct OAuthClientUpdate {
    /// unique identifier of OAuth Client authorization
    pub client_id: String,
    /// The parameters to pass to the Heroku API
    pub params: OAuthClientUpdateParams,
}

/// Update existing oauth client with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-client-update-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct OAuthClientUpdateParams {
    /// OAuth client name
    pub name: String,
    /// endpoint for redirection after authorization with OAuth client
    pub redirect_uri: String,
}

impl HerokuEndpoint<OAuthClient, (), OAuthClientUpdateParams> for OAuthClientUpdate {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("oauth/clients/{}", self.client_id)
    }
    fn body(&self) -> Option<OAuthClientUpdateParams> {
        Some(self.params.clone())
    }
}
