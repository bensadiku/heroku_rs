//Anything related to POST requests for review app and it's properties goes here.
use super::ReviewApp;
use std::collections::HashMap;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Review App Create
///
/// Create a new review app
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-create)
pub struct ReviewAppCreate {
    /// The parameters to pass to the Heroku API
    pub params: ReviewAppCreateParams,
}

/// Create a new review app with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct ReviewAppCreateParams {
    /// the branch of the repository which the review app is based on
    pub branch: String,
    /// unique identifier of pipeline
    pub pipeline: String,
    /// source blob
    pub source_blob: SourceBlob,
    /// A set of key value pairs which will be put into the environment of the spawned review app process.
    pub enviroment: Option<HashMap<String, String>>,
    /// repository id of the fork the branch resides in
    pub fork_repo_id: Option<i64>,
}

impl ReviewAppCreateParams {
    /// Method `new` which takes the following arguments.
    ///
    /// * `branch` - the branch of the repository which the review app is based on.
    /// * `pipeline` - unique identifier of pipeline.
    /// * `source_blob_url` - URL where gzipped tar archive of source code for build was downloaded.
    /// * `source_blob_version` - Optional, The version number (or SHA) of the code to build.
    /// * `enviroment` - Optional, A set of key value pairs which will be put into the environment of the spawned review app process.
    /// * `fork_repo_id` - Optional, repository id of the fork the branch resides in.
    pub fn new(
        branch: String,
        pipeline: String,
        source_blob_url: String,
        source_blob_version: Option<String>,
        enviroment: Option<HashMap<String, String>>,
        fork_repo_id: Option<i64>,
    ) -> ReviewAppCreateParams {
        ReviewAppCreateParams {
            branch: branch,
            pipeline: pipeline,
            source_blob: SourceBlob {
                url: source_blob_url,
                version: source_blob_version,
            },
            enviroment: enviroment,
            fork_repo_id: fork_repo_id,
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct SourceBlob {
    /// URL where gzipped tar archive of source code for build was downloaded.
    pub url: String,
    /// The version number (or SHA) of the code to build.
    pub version: Option<String>,
}

impl HerokuEndpoint<ReviewApp, (), ReviewAppCreateParams> for ReviewAppCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("review-apps")
    }
    fn body(&self) -> Option<ReviewAppCreateParams> {
        Some(self.params.clone())
    }
}
