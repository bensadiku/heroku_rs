//Anything related to GET requests for build and it's properties goes here.
use super::{Build, BuildpackInstallation};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Build List
///
/// List existing builds.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#build-list)
pub struct BuildList {
    /// app_id can be the app name or id.
    pub app_id: String,
}

impl BuildList {
    pub fn new(app_id: String) -> BuildList {
        BuildList { app_id }
    }
}

impl HerokuEndpoint<Vec<Build>> for BuildList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/builds", self.app_id)
    }
}

/// Build Info
///
/// Info for existing build.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#build-info)
pub struct BuildDetails {
    /// app_id can be the app name or id.
    pub app_id: String,
    /// build_id is the build identifier which you want to get
    pub build_id: String,
}

impl BuildDetails {
    pub fn new(app_id: String, build_id: String) -> BuildDetails {
        BuildDetails { app_id, build_id }
    }
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
///
/// List an app’s existing buildpack installations.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#buildpack-installations-list)
pub struct BuildPackInstallationList {
    /// app_id can be the app name or id.
    pub app_id: String,
}

impl BuildPackInstallationList {
    pub fn new(app_id: String) -> BuildPackInstallationList {
        BuildPackInstallationList { app_id }
    }
}

impl HerokuEndpoint<Vec<BuildpackInstallation>> for BuildPackInstallationList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/buildpack-installations", self.app_id)
    }
}
