//Anything related to GET requests for slugs and it's variations goes here.
use super::Slug;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Slug Info
///
/// Info for existing slug.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#slug-info)
///
/// # Example:
///
/// SlugDetails takes two required parameters, app_id and slug_id, and returns the [`Slug`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&SlugDetails::new("APP_ID", "SLUG_ID"));
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
/// [response]: ../struct.Slug.html
pub struct SlugDetails<'a> {
    /// unique app identifier.
    pub app_id: &'a str,
    /// unique slug indentifier.
    pub slug_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> SlugDetails<'a> {
    pub fn new(app_id: &'a str, slug_id: &'a str) -> SlugDetails<'a> {
        SlugDetails { app_id, slug_id }
    }
}

impl<'a> HerokuEndpoint<Slug> for SlugDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/slugs/{}", self.app_id, self.slug_id)
    }
}
