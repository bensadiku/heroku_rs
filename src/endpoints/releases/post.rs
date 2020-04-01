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
    fn body(&self) -> Option<ReleaseCreateParams> {
        Some(self.params.clone())
    }
}

/// Release Rollback
///
/// Rollback to an existing release
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#release-rollback)
#[derive(Serialize)]
pub struct ReleaseRollback {
    /// app_id can be the app name or the app id
    pub app_id: String,
    /// The parameters to pass to the Heroku API
    pub params: ReleaseRollbackParams, 
}

/// Rollback a release with parameters.
///
/// Release parameter is required
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#release-rollback-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct ReleaseRollbackParams {
    /// unique identifier of release
    pub release: String,
}

impl HerokuEndpoint<Release, (), ReleaseRollbackParams> for ReleaseRollback {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/releases", self.app_id)
    }
    fn body(&self) -> Option<ReleaseRollbackParams> {
        Some(self.params.clone())
    }
}