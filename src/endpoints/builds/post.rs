//Anything related to POST requests for build and it's properties goes here.
use super::Build;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Build Create
///
/// Create a new build.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#build-create)
pub struct BuildCreate {
    /// app_id can be the app name or the app id
    pub app_id: String,
    /// The parameters to pass to the Heroku API
    pub params: BuildCreateParams,
}

impl BuildCreate {
    /// Create a new build with required and optional parameters
    /// NOTE: Fields that are not passed are sent as NULL to the api.
    pub fn new(
        app_id: String,
        source_blob_checksum: Option<String>,
        source_blob_url: String,
        source_blob_version: Option<String>,
        buildpacks: Option<Vec<BuildpackParam>>,
    ) -> BuildCreate {
        BuildCreate {
            app_id,
            params: BuildCreateParams {
                buildpacks: buildpacks,
                source_blob: SourceBlobParam {
                    checksum: source_blob_checksum,
                    url: source_blob_url,
                    version: source_blob_version,
                },
            },
        }
    }

    /// Create a new build only with required parameters
    pub fn create(app_id: String, source_blob_url: String) -> BuildCreate {
        BuildCreate {
            app_id,
            params: BuildCreateParams {
                buildpacks: None,
                source_blob: SourceBlobParam {
                    checksum: None,
                    url: source_blob_url,
                    version: None,
                },
            },
        }
    }
}

/// Create build with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#build-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct BuildCreateParams {
    /// Buildpacks are optional parameters
    /// https://devcenter.heroku.com/articles/platform-api-reference#build-create-optional-parameters
    pub buildpacks: Option<Vec<BuildpackParam>>,
    pub source_blob: SourceBlobParam,
}

#[derive(Serialize, Clone, Debug)]
pub struct SourceBlobParam {
    /// an optional checksum of the gzipped tarball for verifying its integrity
    pub checksum: Option<String>,
    /// URL where gzipped tar archive of source code for build was downloaded.
    pub url: String,
    /// Version of the gzipped tarball.
    pub version: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct BuildpackParam {
    /// location of the buildpack for the app. Either a url (unofficial buildpacks) or an internal urn (heroku official buildpacks).
    pub url: String,
    /// either the Buildpack Registry name or a URL of the buildpack for the app
    pub name: String,
}

impl HerokuEndpoint<Build, (), BuildCreateParams> for BuildCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/builds", self.app_id)
    }
    fn body(&self) -> Option<BuildCreateParams> {
        Some(self.params.clone())
    }
}
