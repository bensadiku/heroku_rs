//Anything related to GET requests for build and it's properties goes here.
use super::{Build, BuildpackInstallation};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Get info for existing builds.
/// List existing builds.
/// https://devcenter.heroku.com/articles/platform-api-reference#build-list
pub struct BuildList {
    /// app_id can be the app name or id.
    pub app_id: String,
}

impl HerokuEndpoint<Vec<Build>> for BuildList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/builds", self.app_id)
    }
}

/// Get info for existing build.
/// Info for existing build.
/// https://devcenter.heroku.com/articles/platform-api-reference#build-info
pub struct BuildDetails {
    /// app_id can be the app name or id.
    pub app_id: String,
    /// build_id is the build identifier which you want to get
    pub build_id: String,
}

impl HerokuEndpoint<Build> for BuildDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/builds/{}", self.app_id, self.build_id)
    }
}

/// Buildpack Installations List
/// List an app’s existing buildpack installations.
/// https://devcenter.heroku.com/articles/platform-api-reference#buildpack-installations-list
pub struct BuildPackInstallationList {
    /// app_id can be the app name or id.
    pub app_id: String,
}

impl HerokuEndpoint<Vec<BuildpackInstallation>> for BuildPackInstallationList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/buildpack-installations", self.app_id)
    }
}
