//Anything related to GET requests for build and it's properties goes here.
use super::{Build, BuildpackInstallation};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Build List
///
/// List existing builds.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#build-list)
pub struct BuildList<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
}

impl<'a> BuildList<'a> {
    pub fn new(app_id: &'a str) -> BuildList<'a> {
        BuildList { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<Build>> for BuildList<'a> {
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
pub struct BuildDetails<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// build_id is the build identifier which you want to get
    pub build_id: &'a str,
}

impl<'a> BuildDetails<'a> {
    pub fn new(app_id: &'a str, build_id: &'a str) -> BuildDetails<'a> {
        BuildDetails { app_id, build_id }
    }
}

impl<'a> HerokuEndpoint<Build> for BuildDetails<'a> {
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
pub struct BuildPackInstallationList<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
}

impl<'a> BuildPackInstallationList<'a> {
    pub fn new(app_id: &'a str) -> BuildPackInstallationList<'a> {
        BuildPackInstallationList { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<BuildpackInstallation>> for BuildPackInstallationList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/buildpack-installations", self.app_id)
    }
}
