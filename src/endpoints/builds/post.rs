//Anything related to POST requests for build and it's properties goes here.
use super::Build;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Build Create
///
/// Create a new build.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#build-create)
pub struct BuildCreate<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: BuildCreateParams<'a>,
}

impl<'a> BuildCreate<'a> {
    /// Create a new build only with required parameters
    /// NOTE: Fields that are not passed are sent as NULL to the api.
    pub fn new(app_id: &'a str, source_blob_url: &'a str) -> BuildCreate<'a> {
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

    pub fn checksum(&mut self, checksum: &'a str) -> &mut Self {
        self.params.source_blob.checksum = Some(checksum);
        self
    }
    pub fn version(&mut self, version: &'a str) -> &mut Self {
        self.params.source_blob.version = Some(version);
        self
    }

    pub fn buildpack(&mut self, url: &'a str, name: &'a str) -> &mut Self {
        self.params.buildpacks = Some(vec![BuildpackParam { url, name }]);
        self
    }

    pub fn build(&self) -> BuildCreate<'a> {
        BuildCreate {
            app_id: self.app_id,
            params: BuildCreateParams {
                buildpacks: self.params.buildpacks.clone(),
                source_blob: SourceBlobParam {
                    checksum: self.params.source_blob.checksum,
                    url: self.params.source_blob.url,
                    version: self.params.source_blob.version,
                },
            },
        }
    }
}

/// Create build with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#build-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct BuildCreateParams<'a> {
    /// Buildpacks are optional parameters
    /// https://devcenter.heroku.com/articles/platform-api-reference#build-create-optional-parameters
    pub buildpacks: Option<Vec<BuildpackParam<'a>>>,
    pub source_blob: SourceBlobParam<'a>,
}

#[derive(Serialize, Clone, Debug)]
pub struct SourceBlobParam<'a> {
    /// an optional checksum of the gzipped tarball for verifying its integrity [Nullable]
    pub checksum: Option<&'a str>,
    /// URL where gzipped tar archive of source code for build was downloaded.
    pub url: &'a str,
    /// Version of the gzipped tarball. [Nullable]
    pub version: Option<&'a str>,
}

#[derive(Serialize, Clone, Debug)]
pub struct BuildpackParam<'a> {
    /// location of the buildpack for the app. Either a url (unofficial buildpacks) or an internal urn (heroku official buildpacks).
    pub url: &'a str,
    /// either the Buildpack Registry name or a URL of the buildpack for the app
    pub name: &'a str,
}

impl<'a> HerokuEndpoint<Build, (), BuildCreateParams<'a>> for BuildCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/builds", self.app_id)
    }
    fn body(&self) -> Option<BuildCreateParams<'a>> {
        Some(self.params.clone())
    }
}
