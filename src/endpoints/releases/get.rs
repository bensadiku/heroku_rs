//Anything related to GET requests for releases and it's properties goes here.

use super::Release;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Release List
///
/// List existing releases
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#release-list)
pub struct ReleaseList<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> ReleaseList<'a> {
    pub fn new(app_id: &'a str) -> ReleaseList<'a> {
        ReleaseList { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<Release>> for ReleaseList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/releases", self.app_id)
    }
}

/// Release Info
///
/// Info for existing release
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#release-info)
pub struct ReleaseInfo<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
    /// release_id can be the id or version
    pub release_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> ReleaseInfo<'a> {
    pub fn new(app_id: &'a str, release_id: &'a str) -> ReleaseInfo<'a> {
        ReleaseInfo { app_id, release_id }
    }
}

impl<'a> HerokuEndpoint<Release> for ReleaseInfo<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/releases/{}", self.app_id, self.release_id)
    }
}
