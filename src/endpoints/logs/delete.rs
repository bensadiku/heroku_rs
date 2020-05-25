//Anything related to DELETE requests for heroku logs and it's properties goes here.
use super::LogDrain;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Log Drain Delete
///
/// Delete an existing log drain. Log drains added by add-ons can only be removed by removing the add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#log-drain-delete)
///
/// # Example:
///
/// LogDrainDelete takes two required parameters, app_id and drain_id, and returns the deleted [`LogDrain`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&LogDrainDelete::new("APP_ID", "DRAIN_ID"));
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
/// [response]: ../struct.LogDrain.html
pub struct LogDrainDelete<'a> {
    /// unique app identifier, either app name, or app id
    pub app_id: &'a str,
    /// unique log drain identifier, either drain id, url or token
    pub drain_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> LogDrainDelete<'a> {
    pub fn new(app_id: &'a str, drain_id: &'a str) -> LogDrainDelete<'a> {
        LogDrainDelete { app_id, drain_id }
    }
}

impl<'a> HerokuEndpoint<LogDrain> for LogDrainDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/log-drains/{}", self.app_id, self.drain_id)
    }
}
