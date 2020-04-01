//Anything related to creating apps and it's properties goes here.
use super::{Release};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Release Create
///
/// Create a new release
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#release-create)
#[derive(Serialize)]
pub struct ReleaseCreate {
    /// app_id can be the app name or the app id
    pub app_id: String,
    /// The parameters to pass to the Heroku API
    pub params: ReleaseCreateParams,
}

/// Create a new release with parameters.
///
/// Slug parameter is required
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#release-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct ReleaseCreateParams {
    /// unique identifier of slug
    pub slug: String,
    /// description of changes in release
    pub description: Option<String>
}

impl HerokuEndpoint<Release, (), ReleaseCreateParams> for ReleaseCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/releases", self.app_id)
    }
    fn body(&self) -> Option<DynoCreateParams> {
        Some(self.params.clone())
    }
}
