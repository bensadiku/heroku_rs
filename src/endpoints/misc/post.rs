//Anything related to POST requests for mixed endpoints goes here.
use super::SourceBlob;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Source Create
///
/// Create URLs for uploading and downloading source.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#source-create)
///
/// # Example:
///
/// SourceCreate takes no required parameters, and returns the [`SourceBlob`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&SourceCreate::new());
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
/// [response]: ../struct.SourceBlob.html
pub struct SourceCreate {}

#[cfg(feature = "builder")]
impl SourceCreate {
    pub fn new() -> SourceCreate {
        SourceCreate {}
    }
}

impl HerokuEndpoint<SourceBlob> for SourceCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("sources")
    }
}
