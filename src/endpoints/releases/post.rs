//Anything related to POST requests for releases and it's variations goes here.
use super::AppRelease;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Release Create
///
/// Create new release.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#release-create)
pub struct ReleaseCreate<'a> {
    /// app_id is the unique app identifier.
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: ReleaseCreateParams<'a>,
}

/// Create a new release with parameters.
///
/// [See Heroku documentation for more information about these required parameters](https://devcenter.heroku.com/articles/platform-api-reference#release-create-required-parameters)
///
/// [See Heroku documentation for more information about these optional parameters](https://devcenter.heroku.com/articles/platform-api-reference#release-create-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct ReleaseCreateParams<'a> {
    /// unique identifier of slug
    pub slug: &'a str,
    /// human-friendly description of changes in this release
    pub description: Option<&'a str>,
}

impl<'a> HerokuEndpoint<AppRelease, (), ReleaseCreateParams<'a>> for ReleaseCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/releases", self.app_id)
    }
    fn body(&self) -> Option<ReleaseCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Release Rollback
///
/// Rollback to an existing release.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#release-rollback)
pub struct ReleaseRollback <'a> {
    /// app_id is the unique app identifier.
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: ReleaseRollbackParams<'a>,
}

/// Rollback a release with parameters.
///
/// [See Heroku documentation for more information about these required parameters](https://devcenter.heroku.com/articles/platform-api-reference#release-rollback-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct ReleaseRollbackParams <'a>{
    /// unique identifier of the release you want to roll back
    pub release: &'a str,
}

impl<'a> HerokuEndpoint<AppRelease, (), ReleaseRollbackParams<'a>> for ReleaseRollback <'a>{
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/releases", self.app_id)
    }
    fn body(&self) -> Option<ReleaseRollbackParams<'a>> {
        Some(self.params.clone())
    }
}
