//Anything related to DELETE requests for build and it's properties goes here.
use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Build Delete cache
///
/// Destroy a build cache.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#build-delete-cache)
///
/// # Example:
///
/// BuildDelete takes one required parameter, app_id, and returns nothing.
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&BuildDelete::new("APP_ID_HERE"));
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
pub struct BuildDelete<'a> {
    /// app_id can be the app id or name.
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> BuildDelete<'a> {
    pub fn new(app_id: &'a str) -> BuildDelete<'a> {
        BuildDelete { app_id }
    }
}

impl<'a> HerokuEndpoint for BuildDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/build-cache", self.app_id)
    }
}
