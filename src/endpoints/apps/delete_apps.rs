//Anything related to deleting apps and it's properties goes here.
use super::App;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Delete an existing app.
/// app_identifier is required to delete an app.
/// app_identifier can be the app id or app name.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-delete
pub struct AppDelete {
    pub app_identifier: String,
}

impl HerokuEndpoint<App> for AppDelete {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}", self.app_identifier)
    }
}

/// Disable ACM flag for an app
/// app_identifier is required to disable app acm.
/// app_identifier can be the app id or name.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-disable-acm
pub struct AppDisableAcm {
    pub app_identifier: String,
}

impl HerokuEndpoint<App> for AppDisableAcm {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/acm", self.app_identifier)
    }
}
