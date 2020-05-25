//Anything related to GET requests for formations and it's properties goes here.

use super::Formation;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Formation Info
///
/// Get info for a process type
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#dyno-info)
///
/// # Example:
///
/// FormationDetails takes two required parameters, app_id and formation_id, and returns the [`Formation`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&FormationDetails::new("APP_ID", "FORMATION_ID"));
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
/// [response]: ../struct.Formation.html
pub struct FormationDetails<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
    /// formation_id can &'a str the formation id or type
    pub formation_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> FormationDetails<'a> {
    pub fn new(app_id: &'a str, formation_id: &'a str) -> FormationDetails<'a> {
        FormationDetails {
            app_id,
            formation_id,
        }
    }
}

impl<'a> HerokuEndpoint<Formation> for FormationDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/formation/{}", self.app_id, self.formation_id)
    }
}

/// Formation List
///
/// List process type formation
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#formation-list)
///
/// # Example:
///
/// FormationList takes one required parameter, app_id, and returns a list of [`Formations`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&FormationList::new("APP_ID"));
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
/// [response]: ../struct.Formation.html
pub struct FormationList<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> FormationList<'a> {
    pub fn new(app_id: &'a str) -> FormationList<'a> {
        FormationList { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<Formation>> for FormationList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/formation", self.app_id)
    }
}
