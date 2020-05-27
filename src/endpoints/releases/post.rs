//Anything related to creating apps and it's properties goes here.
use super::Release;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Release Create
///
/// Create a new release
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#release-create)
/// 
/// # Example:
///
/// ReleaseCreate takes two required parameters, app_id and slug, and returns the new [`Release`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&ReleaseCreate::new("APP_ID", "SLUG_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
///
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.Release.html
#[derive(Serialize)]
pub struct ReleaseCreate<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: ReleaseCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> ReleaseCreate<'a> {
    pub fn new(app_id: &'a str, slug: &'a str) -> ReleaseCreate<'a> {
        ReleaseCreate {
            app_id,
            params: ReleaseCreateParams {
                slug: slug,
                description: None,
            },
        }
    }

    /// # description: description of changes in this release
    pub fn description(&mut self, description: &'a str) -> &mut Self {
        self.params.description = Some(description);
        self
    }

    pub fn build(&self) -> ReleaseCreate<'a> {
        ReleaseCreate {
            app_id: self.app_id,
            params: ReleaseCreateParams {
                slug: self.params.slug,
                description: self.params.description,
            },
        }
    }
}

/// Create a new release with parameters.
///
/// Slug parameter is required
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#release-create-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct ReleaseCreateParams<'a> {
    /// unique identifier of slug
    pub slug: &'a str,
    /// description of changes in release
    pub description: Option<&'a str>,
}

impl<'a> HerokuEndpoint<Release, (), ReleaseCreateParams<'a>> for ReleaseCreate<'a> {
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
/// Rollback to an existing release
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#release-rollback)
/// 
/// # Example:
///
/// ReleaseRollback takes two required parameters, app_id and release_id, and returns the [`Release`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&ReleaseRollback::new("APP_ID", "RELEASE_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
///
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.Release.html
#[derive(Serialize)]
pub struct ReleaseRollback<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: ReleaseRollbackParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> ReleaseRollback<'a> {
    pub fn new(app_id: &'a str, release_id: &'a str) -> ReleaseRollback<'a> {
        ReleaseRollback {
            app_id,
            params: ReleaseRollbackParams {
                release: release_id,
            },
        }
    }
}

/// Rollback a release with parameters.
///
/// Release parameter is required
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#release-rollback-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct ReleaseRollbackParams<'a> {
    /// unique identifier of release
    pub release: &'a str,
}

impl<'a> HerokuEndpoint<Release, (), ReleaseRollbackParams<'a>> for ReleaseRollback<'a> {
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
