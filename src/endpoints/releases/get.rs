//Anything related to GET requests for releases and it's properties goes here.

use super::Release;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Release List
///
/// List existing releases
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#release-list)
/// 
/// # Example:
///
/// ReleaseList takes one required parameter, app_id, and returns a list of [`Releases`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&ReleaseList::new("APP_ID"));
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
/// [response]: ../struct.Release.html
pub struct ReleaseList<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> ReleaseList<'a> {
    pub fn new(app_id: &'a str) -> ReleaseList<'a> {
        ReleaseList { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<Release>> for ReleaseList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/releases", self.app_id)
    }
}

/// Release Info
///
/// Info for existing release
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#release-info)
/// 
/// # Example:
///
/// ReleaseInfo takes two required parameter, app_id and release_id, and returns the [`Release`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&ReleaseInfo::new("APP_ID", "RELEASE_ID"));
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
/// [response]: ../struct.Release.html
pub struct ReleaseInfo<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
    /// release_id can be the id or version
    pub release_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> ReleaseInfo<'a> {
    pub fn new(app_id: &'a str, release_id: &'a str) -> ReleaseInfo<'a> {
        ReleaseInfo { app_id, release_id }
    }
}

impl<'a> HerokuEndpoint<Release> for ReleaseInfo<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/releases/{}", self.app_id, self.release_id)
    }
}
