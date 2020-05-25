//Anything related to GET requests for dynos and it's properties goes here.

use super::{Dyno, DynoSize};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Dyno Info
///
/// Get info for existing dyno.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#dyno-info)
/// 
/// # Example:
///
/// DynoDetails takes two required parameters, app_id and dyno_id, and returns a [`Dyno`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&DynoDetails::new("APP_ID", "DYNO_ID"));
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
/// [response]: ../struct.Dyno.html
pub struct DynoDetails<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
    /// dyno_id can be the dyno name or the dyno id
    pub dyno_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> DynoDetails<'a> {
    pub fn new(app_id: &'a str, dyno_id: &'a str) -> DynoDetails<'a> {
        DynoDetails { app_id, dyno_id }
    }
}

impl<'a> HerokuEndpoint<Dyno> for DynoDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/dynos/{}", self.app_id, self.dyno_id)
    }
}

/// Dyno List
///
/// List existing dynos.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#dyno-list)
/// 
/// # Example:
///
/// DynoList takes one required parameter, app_id, and returns a list of [`Dynos`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&DynoList::new("APP_ID"));
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
/// [response]: ../struct.Dyno.html
pub struct DynoList<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> DynoList<'a> {
    pub fn new(app_id: &'a str) -> DynoList<'a> {
        DynoList { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<Dyno>> for DynoList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/dynos", self.app_id)
    }
}

/// Dyno Size List
///
/// List existing dyno sizes.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#dyno-size-list)
/// 
/// # Example:
///
/// DynoSizeList takes no parameters, and returns a list of [`DynoSize`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&DynoSizeList::new());
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
/// [response]: ../struct.DynoSize.html
pub struct DynoSizeList {}

#[cfg(feature = "builder")]
impl DynoSizeList {
    pub fn new() -> DynoSizeList {
        DynoSizeList {}
    }
}

impl HerokuEndpoint<Vec<DynoSize>> for DynoSizeList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("dyno-sizes")
    }
}

/// Dyno Size Info
///
/// Info for existing dyno size.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#dyno-size-info)
/// 
/// # Example:
///
/// DynoSizeDetails takes one parameter, size_id and returns a [DynoSize`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&DynoSizeDetails::new("SIZE_ID"));
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
/// [response]: ../struct.DynoSize.html
pub struct DynoSizeDetails<'a> {
    /// unique dyno size identifier
    pub size_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> DynoSizeDetails<'a> {
    pub fn new(size_id: &'a str) -> DynoSizeDetails {
        DynoSizeDetails { size_id }
    }
}

impl<'a> HerokuEndpoint<DynoSize> for DynoSizeDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("dyno-sizes/{}", self.size_id)
    }
}
