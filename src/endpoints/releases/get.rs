//Anything related to GET requests for releases and it's properties goes here.

use super::Release;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Release List
///
/// List existing releases
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#release-list)
pub struct ReleaseList {
    /// app_id can be the app name or the app id
    pub app_id: String,
}

impl HerokuEndpoint<Vec<Release>> for ReleaseList {
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
pub struct ReleaseInfo {
    /// app_id can be the app name or the app id
    pub app_id: String,
    /// release_id can be the id or version 
    pub release_id: String,
}

impl HerokuEndpoint<Release> for ReleaseInfo {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/releases/{}", self.app_id, self.release_id)
    }
}
