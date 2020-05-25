//Anything related to DELETE requests for dynos and it's properties goes here.

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Dyno Restart
///
/// Restart dyno.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#dyno-restart)
///
/// # Example:
///
/// DynoRestart takes two required parameters, app_id and dyno_id, and returns nothing.
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&DynoRestart::new("APP_ID", "DYNO_ID"));
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
pub struct DynoRestart<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
    /// dyno_id can be the dyno name or the dyno id
    pub dyno_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> DynoRestart<'a> {
    pub fn new(app_id: &'a str, dyno_id: &'a str) -> DynoRestart<'a> {
        DynoRestart { app_id, dyno_id }
    }
}

impl<'a> HerokuEndpoint for DynoRestart<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/dynos/{}", self.app_id, self.dyno_id)
    }
}

/// Dyno Restart all
///
/// Restart all dynos.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#dyno-restart-all)
///
/// # Example:
///
/// DynoAllRestart takes two required parameters, app_id, and returns nothing.
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&DynoAllRestart::new("APP_ID"));
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
pub struct DynoAllRestart<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> DynoAllRestart<'a> {
    pub fn new(app_id: &'a str) -> DynoAllRestart<'a> {
        DynoAllRestart { app_id }
    }
}

impl<'a> HerokuEndpoint for DynoAllRestart<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/dynos", self.app_id)
    }
}
