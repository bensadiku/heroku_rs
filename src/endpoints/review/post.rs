//Anything related to POST requests for review app and it's properties goes here.
use super::{ReviewApp, ReviewAppConfig};
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

impl ReviewAppCreate {
    pub fn new(
        branch: String,
        pipeline_id: String,
        source_blob_url: String,
        source_blob_version: Option<String>,
        enviroment: Option<HashMap<String, String>>,
        fork_repo_id: Option<i64>,
    ) -> ReviewAppCreate {
        ReviewAppCreate {
            params: ReviewAppCreateParams {
                branch: branch,
                pipeline: pipeline_id,
                source_blob: SourceBlob {
                    url: source_blob_url,
                    version: source_blob_version,
                },
                enviroment: enviroment,
                fork_repo_id: fork_repo_id,
            },
        }
    }

    pub fn create(branch: String, pipeline_id: String, source_blob_url: String) -> ReviewAppCreate {
        ReviewAppCreate {
            params: ReviewAppCreateParams {
                branch: branch,
                pipeline: pipeline_id,
                source_blob: SourceBlob {
                    url: source_blob_url,
                    version: None,
                },
                enviroment: None,
                fork_repo_id: None,
            },
        }
    }
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
    /// A set of key value pairs which will be put into the environment of the spawned review app process. [Nullable]
    pub enviroment: Option<HashMap<String, String>>,
    /// repository id of the fork the branch resides in. [Nullable]
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
    /// The version number (or SHA) of the code to build. [Nullable]
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

/// Review App Configuration Enable
///
/// Enable review apps for a pipeline
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-configuration-enable)
pub struct ReviewAppConfigEnable<'a> {
    pub pipeline_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: ReviewAppConfigEnableParams<'a>,
}

impl<'a> ReviewAppConfigEnable<'a> {
    pub fn new(
        pipeline_id: &'a str,
        repo: &'a str,
        automatic_review_apps: Option<bool>,
        destroy_stale_apps: Option<bool>,
        stale_days: Option<&'a str>,
        deploy_target_id: Option<&'a str>,
        deploy_target_type: Option<&'a str>,
        wait_for_ci: Option<bool>,
        base_name: Option<&'a str>,
    ) -> ReviewAppConfigEnable<'a> {
        // tl;dr DeployTarget is nullable in the API and both fields should either provided or not.
        let deploy_type: Option<DeployTarget> = match (deploy_target_id, deploy_target_type) {
            (Some(target_id), Some(type_id)) => Some(DeployTarget {
                id: target_id,
                type_field: type_id
            }),
            (None, None) =>  None,
            _ => panic!("deploy_target_id and deploy_target_type have to be either both Some or both None, but not in-between!"),
        };

        ReviewAppConfigEnable {
            pipeline_id,
            params: ReviewAppConfigEnableParams {
                repo: repo,
                automatic_review_apps: automatic_review_apps,
                destroy_stale_apps: destroy_stale_apps,
                stale_days: stale_days,
                deploy_target: deploy_type,
                wait_for_ci: wait_for_ci,
                base_name: base_name,
            },
        }
    }

    pub fn create(pipeline_id: &'a str, repo: &'a str) -> ReviewAppConfigEnable<'a> {
        ReviewAppConfigEnable {
            pipeline_id,
            params: ReviewAppConfigEnableParams {
                repo: repo,
                automatic_review_apps: None,
                destroy_stale_apps: None,
                stale_days: None,
                deploy_target: None,
                wait_for_ci: None,
                base_name: None,
            },
        }
    }
}

/// Enable review app configurations with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-configuration-enable-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct ReviewAppConfigEnableParams<'a> {
    /// The full_name of the repository that you want enable review-apps from.
    pub repo: &'a str,
    /// If true, this will trigger the creation of review apps when pull-requests are opened in the repo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_review_apps: Option<bool>,
    /// If true, this will trigger automatic deletion of review apps when they’re stale
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_stale_apps: Option<bool>,
    /// If destroy_stale_apps is true, then apps will be destroyed after this many days
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale_days: Option<&'a str>,
    /// Provides a key/value pair to specify whether to use a common runtime or a private space. [Nullable]
    pub deploy_target: Option<DeployTarget<'a>>,
    /// If true, review apps will only be created when CI passes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_ci: Option<bool>,
    /// A unique prefix that will be used to create review app names. [Nullable]
    pub base_name: Option<&'a str>,
}

#[derive(Serialize, Clone, Debug)]
pub struct DeployTarget<'a> {
    /// unique identifier of deploy target
    ///  pattern: `(^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$
    pub id: &'a str,
    #[serde(rename = "type")]
    /// type of deploy target
    ///  pattern: `(^space$
    pub type_field: &'a str,
}

impl<'a> HerokuEndpoint<ReviewAppConfig, (), ReviewAppConfigEnableParams<'a>>
    for ReviewAppConfigEnable<'a>
{
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("pipelines/{}/review-app-config", self.pipeline_id)
    }
    fn body(&self) -> Option<ReviewAppConfigEnableParams<'a>> {
        Some(self.params.clone())
    }
}
