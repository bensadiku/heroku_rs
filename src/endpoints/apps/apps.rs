use super::App;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Get info for existing app.
/// identifier can be the app id or app name.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-info
pub struct AppDetails {
    pub identifier: String,
}

impl HerokuEndpoint<App> for AppDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}", self.identifier)
    }
}
