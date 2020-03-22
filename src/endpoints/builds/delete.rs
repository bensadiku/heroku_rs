//Anything related to DELETE requests for build and it's properties goes here.
use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Build Delete cache
/// Destroy a build cache.
/// https://devcenter.heroku.com/articles/platform-api-reference#build-delete-cache
pub struct BuildDelete {
    pub app_identifier: String,
}

impl HerokuEndpoint<> for BuildDelete {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/build-cache", self.app_identifier)
    }
}
