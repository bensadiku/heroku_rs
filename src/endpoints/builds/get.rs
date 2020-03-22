//Anything related to GET requests for build and it's properties goes here.
use super::{Build, BuildpackInstallation};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Get info for existing builds.
/// List existing builds.
/// https://devcenter.heroku.com/articles/platform-api-reference#build-list
pub struct BuildList {
    /// app_identifier can be the app name or id.
    pub app_identifier: String,
}

impl HerokuEndpoint<Vec<Build>> for BuildList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/builds", self.app_identifier)
    }
}

/// Get info for existing build.
/// Info for existing build.
/// https://devcenter.heroku.com/articles/platform-api-reference#build-info
pub struct BuildDetails {
    /// app_identifier can be the app name or id.
    pub app_identifier: String,
    /// The parameters to pass to the Heroku API
    pub build_identifier: String,
}

impl HerokuEndpoint<Build> for BuildDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "apps/{}/builds/{}",
            self.app_identifier, self.build_identifier
        )
    }
}

/// Buildpack Installations List
/// List an app’s existing buildpack installations.
/// https://devcenter.heroku.com/articles/platform-api-reference#buildpack-installations-list
pub struct BuildPackInstallationList {
    /// app_identifier can be the app name or id.
    pub app_identifier: String,
}

impl HerokuEndpoint<Vec<BuildpackInstallation>> for BuildPackInstallationList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/buildpack-installations", self.app_identifier)
    }
}
