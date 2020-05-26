//Anything related to POST requests for oauth authorizations and it's properties goes here.
use super::{OAuth, OAuthClient, OAuthToken};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// OAuth Authorization Create
///
/// Create a new OAuth authorization.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-authorization-create)
/// 
/// # Example:
///
/// OAuthCreate takes one required parameter, scope, and returns the created [`OAuth`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let auth_scope = vec!["global"];
/// let response = api_client.request(
///     &OAuthCreate::new(auth_scope)
///         .description("Global oauth token")
///         .client("CLIENT_ID")
///         .expires_in(2592000) //null for indefinite lifetime
///         .build(),
/// );
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
pub struct OAuthCreate<'a> {
    /// The parameters to pass to the Heroku API
    pub params: OAuthCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> OAuthCreate<'a> {
    pub fn new(scope: Vec<&'a str>) -> OAuthCreate<'a> {
        OAuthCreate {
            params: OAuthCreateParams {
                scope: scope,
                client: None,
                description: None,
                expires_in: None,
            },
        }
    }

    /// # client: unique identifier of this OAuth client
    pub fn client(&mut self, client: &'a str) -> &mut Self {
        self.params.client = Some(client);
        self
    }

    /// # description: human-friendly description of this OAuth authorization
    pub fn description(&mut self, description: &'a str) -> &mut Self {
        self.params.description = Some(description);
        self
    }

    /// # expires_in: seconds until OAuth token expires; may be null for tokens with indefinite lifetime
    pub fn expires_in(&mut self, expires_in: u32) -> &mut Self {
        self.params.expires_in = Some(expires_in);
        self
    }

    pub fn build(&self) -> OAuthCreate<'a> {
        OAuthCreate {
            params: OAuthCreateParams {
                scope: self.params.scope.clone(),
                client: self.params.client,
                description: self.params.description,
                expires_in: self.params.expires_in,
            },
        }
    }
}

/// Create a new authorization with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-authorization-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct OAuthCreateParams<'a> {
    /// The scope of access OAuth authorization allows
    pub scope: Vec<&'a str>,
    /// unique identifier of this OAuth client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<&'a str>,
    /// human-friendly description of this OAuth authorization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// seconds until OAuth token expires; may be null for tokens with indefinite lifetime [Nullable]
    pub expires_in: Option<u32>,
}

impl<'a> HerokuEndpoint<OAuth, (), OAuthCreateParams<'a>> for OAuthCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("oauth/authorizations")
    }
    fn body(&self) -> Option<OAuthCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// OAuth Authorization Regenerate
///
/// Regenerate OAuth tokens. This endpoint is only available to direct authorizations or privileged OAuth clients.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-authorization-regenerate)
/// 
/// # Example:
///
/// OAuthRegenerate takes one required parameter, oauth_id, and returns the regenerated [`OAuth`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(
///     &OAuthRegenerate::new("OAUTH_ID"));
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
pub struct OAuthRegenerate<'a> {
    /// unique identifier of OAuth authorization
    pub oauth_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> OAuthRegenerate<'a> {
    pub fn new(oauth_id: &'a str) -> OAuthRegenerate<'a> {
        OAuthRegenerate { oauth_id }
    }
}

impl<'a> HerokuEndpoint<OAuth> for OAuthRegenerate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!(
            "oauth/authorizations/{}/actions/regenerate-tokens",
            self.oauth_id
        )
    }
}

/// OAuth Client Create
///
/// Create a new OAuth client.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-client-create)
/// 
/// # Example:
///
/// OAuthClientCreate takes one required parameter, oauth_id, and returns the created [`OAuthClient`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let name = "Client Name";
/// let redirect_uri = "https://www.redirecting_site_here.dev";
/// let response = api_client.request(&OAuthClientCreate::new(name, redirect_uri));
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
pub struct OAuthClientCreate<'a> {
    /// The parameters to pass to the Heroku API
    pub params: OAuthClientCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> OAuthClientCreate<'a> {
    pub fn new(name: &'a str, redirect_uri: &'a str) -> OAuthClientCreate<'a> {
        OAuthClientCreate {
            params: OAuthClientCreateParams { name, redirect_uri },
        }
    }
}

/// Create a new oauth client with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-client-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct OAuthClientCreateParams<'a> {
    /// OAuth client name
    pub name: &'a str,
    /// endpoint for redirection after authorization with OAuth client
    pub redirect_uri: &'a str,
}

impl<'a> HerokuEndpoint<OAuthClient, (), OAuthClientCreateParams<'a>> for OAuthClientCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("oauth/clients")
    }
    fn body(&self) -> Option<OAuthClientCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// OAuth Client Rotate Credentials
///
/// Rotate credentials for an OAuth client
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-client-rotate-credentials)
/// 
/// # Example:
///
/// OAuthClientRotateCredentials takes one required parameter, oauth_id, and returns the [`OAuthClient`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&OAuthClientRotateCredentials::new("CLIENT_ID"));
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
pub struct OAuthClientRotateCredentials<'a> {
    /// unique identifier of OAuth Client authorization
    pub client_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> OAuthClientRotateCredentials<'a> {
    pub fn new(client_id: &'a str) -> OAuthClientRotateCredentials<'a> {
        OAuthClientRotateCredentials { client_id }
    }
}

impl<'a> HerokuEndpoint<OAuthClient> for OAuthClientRotateCredentials<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!(
            "oauth/clients/{}/actions/rotate-credentials",
            self.client_id
        )
    }
}

/// OAuth Token Create
///
/// Create a new OAuth token.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-token-create)
/// 
/// # Example:
///
/// OAuthTokenCreate takes one required parameter, oauth_id, and returns the created [`OAuthToken`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let client_secret = "01234567-89ab-cdef-0123-456789abcdef";
/// let grant_code = "01234567-89ab-cdef-0123-456789abcdef";
/// let grant_type = "authorization_code";
/// let refresh_token = "01234567-89ab-cdef-0123-456789abcdef";
/// 
/// let response = api_client.request(&OAuthTokenCreate::new(
///     client_secret,
///     grant_code,
///     grant_type,
///     refresh_token,
/// ));
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
/// [response]: ../struct.OAuthToken.html
pub struct OAuthTokenCreate<'a> {
    /// The parameters to pass to the Heroku API
    pub params: OAuthTokenCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> OAuthTokenCreate<'a> {
    pub fn new(
        client_secret: &'a str,
        grant_code: &'a str,
        grant_type: &'a str,
        refresh_token: &'a str,
    ) -> OAuthTokenCreate<'a> {
        OAuthTokenCreate {
            params: OAuthTokenCreateParams {
                client: Client {
                    secret: client_secret,
                },
                grant: Grant {
                    code: grant_code,
                    type_field: grant_type,
                },
                refresh_token: RefreshToken {
                    token: refresh_token,
                },
            },
        }
    }
}

/// Create a new oauth token with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#oauth-token-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct OAuthTokenCreateParams<'a> {
    /// OAuth client
    pub client: Client<'a>,
    /// OAuth grant
    pub grant: Grant<'a>,
    /// endpoint for redirection after authorization with OAuth client
    pub refresh_token: RefreshToken<'a>,
}

// TODO(ben): Find a better solution than this
///RefreshToken
#[derive(Serialize, Clone, Debug)]
pub struct RefreshToken<'a> {
    /// contents of the token to be used for authorization
    pub token: &'a str,
}

// TODO(ben): Find a better solution than this
/// Grant
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Grant<'a> {
    /// grant code received from OAuth web application authorization
    pub code: &'a str,
    /// type of grant requested, one of authorization_code or refresh_token
    #[serde(rename = "type")]
    pub type_field: &'a str,
}

// TODO(ben): Find a better solution than this
/// OAuth client secret used to obtain token
#[derive(Serialize, Clone, Debug)]
pub struct Client<'a> {
    /// secret used to obtain OAuth authorizations under this client
    pub secret: &'a str,
}

impl<'a> HerokuEndpoint<OAuthToken, (), OAuthTokenCreateParams<'a>> for OAuthTokenCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("oauth/tokens")
    }
    fn body(&self) -> Option<OAuthTokenCreateParams<'a>> {
        Some(self.params.clone())
    }
}
