//Anything related to getting apps and it's properties goes here.
use super::{App, AppFeature};

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

/// List existing apps.
/// No parameters required to get this list.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-list
pub struct AppList {}

impl HerokuEndpoint<Vec<App>> for AppList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps")
    }
}

/// List owned and collaborated apps (excludes team apps).
/// account_identifier is required to delete an app.
/// account_identifier can be the account email, id or self.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-list-owned-and-collaborated
pub struct AccountAppList {
    pub account_identifier: String,
}

impl HerokuEndpoint<Vec<App>> for AccountAppList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("users/{}/apps", self.account_identifier)
    }
}

/// Info for an existing app feature.
/// app_identifier and feature_identifier are required.
/// app_identifier can be the app name or id.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-feature-info
pub struct AppFeatureDetails {
    pub app_identifier: String,
    pub feature_identifier: String,
}

impl HerokuEndpoint<AppFeature> for AppFeatureDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "apps/{}/features/{}",
            self.app_identifier, self.feature_identifier
        )
    }
}

/// List existing app features.
/// app_identifier is required.
/// app_identifier can be the app name or id.
/// https://devcenter.heroku.com/articles/platform-api-reference#app-feature-list
pub struct AppFeatureList {
    pub app_identifier: String,
}

impl HerokuEndpoint<Vec<AppFeature>> for AppFeatureList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/features", self.app_identifier)
    }
}
