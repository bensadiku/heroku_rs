//Anything related to GET requests for releases and it's variations goes here.
use super::AppRelease;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Release Info
///
/// Info for existing release.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#release-info)
pub struct ReleaseDetails<'a> {
    /// app_id is the unique app identifier.
    pub app_id: &'a str,
    /// release_id is the unique release identifier.
    pub release_id: &'a str,
}

impl<'a> HerokuEndpoint<AppRelease> for ReleaseDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/releases/{}", self.app_id, self.release_id)
    }
}

/// Release List
///
/// List existing releases for an app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#release-list)
pub struct ReleaseList<'a> {
    /// app_id is the unique app identifier.
    pub app_id: &'a str,
}

impl<'a> HerokuEndpoint<Vec<AppRelease>> for ReleaseList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/releases", self.app_id)
    }
}
