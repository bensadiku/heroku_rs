use crate::framework::response::ApiResult;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::{OAuthClientDelete, OAuthDelete, OAuthTokenDelete};
pub use get::{OAuthClientDetails, OAuthClientList, OAuthDetails, OAuthList};
pub use patch::{OAuthClientUpdate, OAuthClientUpdateParams};
pub use post::{
    OAuthClientCreate, OAuthClientCreateParams, OAuthClientRotateCredentials, OAuthCreate,
    OAuthCreateParams, OAuthRegenerate, OAuthTokenCreate, OAuthTokenCreateParams,
};

impl ApiResult for auth::OAuth {}
impl ApiResult for Vec<auth::OAuth> {}

impl ApiResult for client::OAuthClient {}
impl ApiResult for Vec<client::OAuthClient> {}

impl ApiResult for token::OAuthToken {}
impl ApiResult for Vec<token::OAuthToken> {}

pub use auth::OAuth;
pub use client::OAuthClient;
pub use token::OAuthToken;

// auth submodule, anything from /oauth/authorizations goes here.
mod auth {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// OAuth Authorization
    ///
    /// Stability: production
    ///
    /// OAuth authorizations represent clients that a Heroku user has authorized to automate, customize or extend their usage of the platform.
    ///
    /// [For more information please refer to the Heroku OAuth documentation](https://devcenter.heroku.com/articles/oauth)
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct OAuth {
        /// access token for this authorization
        pub access_token: Option<AccessToken>,
        /// identifier of the client that obtained this authorization, if any
        pub client: Option<Client>,
        /// when OAuth authorization was created
        pub created_at: DateTime<Utc>,
        /// this authorization’s grant
        pub grant: Option<Grant>,
        /// unique identifier of OAuth authorization
        pub id: String,
        /// refresh token for this authorization
        pub refresh_token: Option<RefreshToken>,
        /// The scope of access OAuth authorization allows
        pub scope: Vec<String>,
        /// when OAuth authorization was updated
        pub updated_at: DateTime<Utc>,
        /// User account
        pub user: User,
    }

    /// OAuth Token
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct AccessToken {
        /// seconds until OAuth token expires; may be null for tokens with indefinite lifetime
        pub expires_in: Option<i64>,
        /// unique identifier of OAuth token
        pub id: String,
        /// contents of the token to be used for authorization
        pub token: String,
    }

    /// OAuth Client
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Client {
        /// unique identifier of this OAuth client
        pub id: String,
        /// OAuth client name
        pub name: String,
        /// endpoint for redirection after authorization with OAuth client
        pub redirect_uri: String,
    }

    /// OAuth Grant
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Grant {
        /// grant code received from OAuth web application authorization
        pub code: String,
        /// seconds until OAuth grant expires
        pub expires_in: i64,
        /// unique identifier of OAuth grant
        pub id: String,
    }

    /// OAuth Token
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct RefreshToken {
        /// refresh token for this authorization
        pub expires_in: Option<i64>,
        /// unique identifier of OAuth token
        pub id: String,
        /// contents of the token to be used for authorization
        pub token: String,
    }

    /// Account
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct User {
        /// identifier of an account
        pub id: String,
        /// unique email address
        pub email: String,
        /// full name of the account owner
        pub full_name: Option<String>,
    }
}

// oauth client submodule, anything from /oauth/clients goes here.
mod client {
    use chrono::offset::Utc;
    use chrono::DateTime;
    /// OAuth Client
    ///
    /// Stability: production
    ///
    /// OAuth clients are applications that Heroku users can authorize to automate, customize or extend their usage of the platform.
    ///
    /// [For more information please refer to the Heroku OAuth documentation](https://devcenter.heroku.com/articles/oauth)
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct OAuthClient {
        /// when OAuth client was created
        pub created_at: DateTime<Utc>,
        /// unique identifier of this OAuth client
        pub id: String,
        /// whether the client is still operable given a delinquent account
        pub ignores_delinquent: Option<bool>,
        /// OAuth client name
        pub name: String,
        /// endpoint for redirection after authorization with OAuth client
        pub redirect_uri: String,
        /// secret used to obtain OAuth authorizations under this client
        pub secret: String,
        /// when OAuth client was updated
        pub updated_at: DateTime<Utc>,
    }
}

// token submodule, anything from /oauth/tokens goes here.
mod token {
    use chrono::offset::Utc;
    use chrono::DateTime;
    /// OAuth Token
    ///
    /// Stability: production
    ///
    /// OAuth tokens provide access for authorized clients to act on behalf of a Heroku user to automate, customize or extend their usage of the platform.
    ///
    /// [For more information please refer to the Heroku OAuth documentation](https://devcenter.heroku.com/articles/oauth)
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct OAuthToken {
        /// AccessToken
        pub access_token: AccessToken,
        /// Authorization
        pub authorization: Authorization,
        /// OAuth client secret used to obtain token
        pub client: Option<Client>,
        /// when OAuth token was created
        pub created_at: DateTime<Utc>,
        /// Grant
        pub grant: Grant,
        /// unique identifier of OAuth token
        pub id: String,
        /// RefreshToken
        pub refresh_token: RefreshToken,
        /// Session
        pub session: Session,
        /// when OAuth token was updated
        pub updated_at: DateTime<Utc>,
        /// Account
        pub user: User,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct AccessToken {
        /// seconds until OAuth token expires; may be null for tokens with indefinite lifetime
        pub expires_in: Option<i64>,
        /// unique identifier of OAuth token
        pub id: String,
        /// contents of the token to be used for authorization
        pub token: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Authorization {
        /// unique identifier of OAuth authorization
        pub id: String,
    }

    /// OAuth client secret used to obtain token
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Client {
        /// secret used to obtain OAuth authorizations under this client
        pub secret: String,
    }

    /// Grant
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Grant {
        /// grant code received from OAuth web application authorization
        pub code: String,
        /// type of grant requested, one of authorization_code or refresh_token
        #[serde(rename = "type")]
        pub type_field: String,
    }

    /// RefreshToken
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct RefreshToken {
        /// seconds until OAuth token expires; may be null for tokens with indefinite lifetime
        pub expires_in: Option<i64>,
        /// unique identifier of OAuth token
        pub id: String,
        /// contents of the token to be used for authorization
        pub token: String,
    }

    /// Session
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Session {
        /// unique identifier of OAuth token
        pub id: String,
    }

    /// Account
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct User {
        /// identifier of an account
        pub id: String,
    }
}
