//Anything related to PATCH requests for spaces goes here.
use super::{Space, SpaceAccess};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Space Update
///
/// Update an existing space.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-update)
///
/// # Example:
///
/// SpaceUpdate takes one required parameter, space_id, and returns the updated [`Space`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let space_id = "myspacename";
/// let space = &SpaceUpdate::new(space_id)
///     .name("mynewspacename")
///     .build();
/// let response = api_client.request(space);
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
/// [response]: ../struct.Space.html
pub struct SpaceUpdate<'a> {
    /// unique space identifier, either space name or space id
    pub space_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: SpaceUpdateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> SpaceUpdate<'a> {
    pub fn new(space_id: &'a str) -> SpaceUpdate<'a> {
        SpaceUpdate {
            space_id,
            params: SpaceUpdateParams { name: None },
        }
    }

    /// # name: unique name of space
    /// 
    /// `pattern`:  pattern: `^[a-z0-9](?:[a-z0-9]
    pub fn name(&mut self, _name: &'a str) -> &mut Self {
        self.params.name = Some(_name);
        self
    }

    pub fn build(&self) -> SpaceUpdate<'a> {
        SpaceUpdate {
            space_id: self.space_id,
            params: SpaceUpdateParams {
                name: self.params.name,
            },
        }
    }
}

/// Update spaces with parameters
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-update-optional-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct SpaceUpdateParams<'a> {
    /// unique name of space
    ///  pattern: `^[a-z0-9](?:[a-z0-9]
    pub name: Option<&'a str>,
}

impl<'a> HerokuEndpoint<Space, (), SpaceUpdateParams<'a>> for SpaceUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("spaces/{}", self.space_id)
    }
    fn body(&self) -> Option<SpaceUpdateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Space Access Update
///
/// Update an existing user’s set of permissions on a space.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-access-update)
///
/// # Example:
///
/// SpaceAccessUpdate takes three required parameters, space_id and account_id and permission name, and returns the updated [`SpaceAccess`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let space = &SpaceAccessUpdate::new("SPACE_ID", "ACCOUNT_ID", "example");
/// let response = api_client.request(space);
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
/// [response]: ../struct.SpaceAccess.html
pub struct SpaceAccessUpdate<'a> {
    /// unique space identifier, either space name or space id
    pub space_id: &'a str,
    /// unique account identifier, either account email or account if
    pub account_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: SpaceAccessUpdateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> SpaceAccessUpdate<'a> {
    pub fn new(space_id: &'a str, account_id: &'a str, _name: &'a str) -> SpaceAccessUpdate<'a> {
        SpaceAccessUpdate {
            space_id,
            account_id,
            params: SpaceAccessUpdateParams {
                permissions: vec![Permissions { name: _name }],
            },
        }
    }
}

/// Update spaces with parameters
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-access-update-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct SpaceAccessUpdateParams<'a> {
    pub permissions: Vec<Permissions<'a>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct Permissions<'a> {
    /// unique name of space
    ///  pattern: `^[a-z0-9](?:[a-z0-9]
    pub name: &'a str,
}

impl<'a> HerokuEndpoint<SpaceAccess, (), SpaceAccessUpdateParams<'a>> for SpaceAccessUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("spaces/{}/members/{}", self.space_id, self.account_id)
    }
    fn body(&self) -> Option<SpaceAccessUpdateParams<'a>> {
        Some(self.params.clone())
    }
}
