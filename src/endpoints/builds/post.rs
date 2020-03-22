//Anything related to POST requests for build and it's properties goes here.
use super::{Build, Buildpack, SourceBlob};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Build Create
/// Create a new build.
/// https://devcenter.heroku.com/articles/platform-api-reference#build-create
pub struct BuildCreate {
    /// This app_idetifier can be the app name or the app id
    pub app_identifier: String,
    /// The parameters to pass to the Heroku API
    pub params: BuildCreateParams,
}

/// Create build with parameters.
/// https://devcenter.heroku.com/articles/platform-api-reference#build-create-required-parameters
#[derive(Serialize, Clone, Debug)]
pub struct BuildCreateParams {
    /// Buildpacks are optional parameters
    /// https://devcenter.heroku.com/articles/platform-api-reference#build-create-optional-parameters
    pub buildpacks: Option<Vec<Buildpack>>,
    pub source_blob: SourceBlob,
}

impl HerokuEndpoint<Build, (), BuildCreateParams> for BuildCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!(
            "apps/{}/builds",
            self.app_identifier
        )
    }
    fn body(&self) -> Option<BuildCreateParams> {
        Some(self.params.clone())
    }
}
