//Anything related to POST requests for review app and it's properties goes here.
use super::{ReviewApp, ReviewAppConfig};
use std::collections::HashMap;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Review App Create
///
/// Create a new review app
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-create)
///
/// # Example:
///
/// ReviewAppCreate takes one required parameter, pipeline_id, and returns the created [`ReviewApp`][response].
/// ```rust
/// use heroku_rs::prelude::*;
/// use std::collections::HashMap;
/// 
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let mut env = HashMap::new();
/// env.insert("FOO", "baz");
/// env.insert("BAZ", "qux");
/// 
/// let branch = "master";
/// let pipeline = "PIPELINE_ID";
/// let source_blob_url = "https://nodejs.org/dist/v0.10.20/node-v0.10.20-linux-x64.tar.gz";
/// let response = api_client.request(
///     &ReviewAppCreate::new(branch, pipeline, source_blob_url)
///         .fork_repo_id(123)
///         .enviroment(env)
///         .pr_number(122)
///         .source_blob_version("v1.2.0")
///         .build(),
/// );
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
/// [response]: ../struct.ReviewApp.html
pub struct ReviewAppCreate<'a> {
    /// The parameters to pass to the Heroku API
    pub params: ReviewAppCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> ReviewAppCreate<'a> {
    pub fn new(
        branch: &'a str,
        pipeline_id: &'a str,
        source_blob_url: &'a str,
    ) -> ReviewAppCreate<'a> {
        ReviewAppCreate {
            params: ReviewAppCreateParams {
                branch: branch,
                pr_number: None,
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

    /// # source_blob_version: The version number (or SHA) of the code to build.
    pub fn source_blob_version(&mut self, source_blob_version: &'a str) -> &mut Self {
        self.params.source_blob.version = Some(source_blob_version);
        self
    }

    /// # source_blob_url: URL where gzipped tar archive of source code for build was downloaded.
    pub fn source_blob_url(&mut self, source_blob_url: &'a str) -> &mut Self {
        self.params.source_blob.url = source_blob_url;
        self
    }

    /// # enviroment: A set of key value pairs which will be put into the environment of the spawned review app process.
    pub fn enviroment(&mut self, enviroment: HashMap<&'a str, &'a str>) -> &mut Self {
        self.params.enviroment = Some(enviroment);
        self
    }

    /// # pr_number: pull request number the review app is built for
    pub fn pr_number(&mut self, pr_number: u32) -> &mut Self {
        self.params.pr_number = Some(pr_number);
        self
    }

    /// # fork_repo_id: repository id of the fork the branch resides in
    pub fn fork_repo_id(&mut self, fork_repo_id: i64) -> &mut Self {
        self.params.fork_repo_id = Some(fork_repo_id);
        self
    }

    /// * `branch` - the branch of the repository which the review app is based on.
    /// * `pipeline` - unique identifier of pipeline.
    /// * `source_blob_url` - URL where gzipped tar archive of source code for build was downloaded.
    /// * `source_blob_version` - Optional, The version number (or SHA) of the code to build.
    /// * `pr_number` - Optional, pull request number the review app is built for
    /// * `enviroment` - Optional, A set of key value pairs which will be put into the environment of the spawned review app process.
    /// * `fork_repo_id` - Optional, repository id of the fork the branch resides in.
    pub fn build(&self) -> ReviewAppCreate<'a> {
        ReviewAppCreate {
            params: ReviewAppCreateParams {
                branch: self.params.branch,
                pr_number: self.params.pr_number,
                pipeline: self.params.pipeline,
                source_blob: SourceBlob {
                    url: self.params.source_blob.url,
                    version: self.params.source_blob.version,
                },
                enviroment: self.params.enviroment.clone(),
                fork_repo_id: self.params.fork_repo_id,
            },
        }
    }
}

/// Create a new review app with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct ReviewAppCreateParams<'a> {
    /// the branch of the repository which the review app is based on
    pub branch: &'a str,
    /// pull request number the review app is built for
    pub pr_number: Option<u32>,
    /// unique identifier of pipeline
    pub pipeline: &'a str,
    /// source blob
    pub source_blob: SourceBlob<'a>,
    /// A set of key value pairs which will be put into the environment of the spawned review app process. [Nullable]
    pub enviroment: Option<HashMap<&'a str, &'a str>>,
    /// repository id of the fork the branch resides in. [Nullable]
    pub fork_repo_id: Option<i64>,
}

#[derive(Serialize, Clone, Debug)]
pub struct SourceBlob<'a> {
    /// URL where gzipped tar archive of source code for build was downloaded.
    pub url: &'a str,
    /// The version number (or SHA) of the code to build. [Nullable]
    pub version: Option<&'a str>,
}

impl<'a> HerokuEndpoint<ReviewApp, (), ReviewAppCreateParams<'a>> for ReviewAppCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("review-apps")
    }
    fn body(&self) -> Option<ReviewAppCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Review App Configuration Enable
///
/// Enable review apps for a pipeline
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-configuration-enable)
///
/// # Example:
///
/// ReviewAppConfigEnable takes two required parameters, pipeline_id and repo, and returns the created [`ReviewAppConfig`][response].
/// ```rust
/// use heroku_rs::prelude::*;
/// 
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let pipeline_id = "PIPELINE_ID";
/// let repo = "heroku/heroku-rs-test";
/// 
/// // `new` method takes only the required parameters
/// let response = api_client.request(
///     &ReviewAppConfigEnable::new(pipeline_id, repo)
///         .base_name("singular-app")
///         .wait_for_ci(true)
///         .deploy_target("^[a-z]{2}$)`","^region$)`")
///         .stale_days("5")
///         .destroy_stale_apps(true)
///         .automatic_review_apps(true)
///         .build(),
/// );
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
/// [response]: ../struct.ReviewAppConfig.html
pub struct ReviewAppConfigEnable<'a> {
    /// unique pipeline identifier
    pub pipeline_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: ReviewAppConfigEnableParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> ReviewAppConfigEnable<'a> {
    pub fn new(pipeline_id: &'a str, repo: &'a str) -> ReviewAppConfigEnable<'a> {
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

    /// # base_name: A unique prefix that will be used to create review app names
    pub fn base_name(&mut self, base_name: &'a str) -> &mut Self {
        self.params.base_name = Some(base_name);
        self
    }

    /// # wait_for_ci: If true, review apps will only be created when CI passes
    pub fn wait_for_ci(&mut self, wait_for_ci: bool) -> &mut Self {
        self.params.wait_for_ci = Some(wait_for_ci);
        self
    }

    /// # deploy_target: Provides a key/value pair to specify whether to use a common runtime or a private space
    /// 
    /// ## id: unique identifier of deploy target
    /// 
    /// `pattern`:  pattern: `(^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$
    /// 
    /// ## type of deploy target
    /// 
    /// `pattern`:  pattern: `(^space$
    pub fn deploy_target(&mut self, id: &'a str, t_type: &'a str) -> &mut Self {
        self.params.deploy_target = Some(DeployTarget {
            id: id,
            type_field: t_type,
        });
        self
    }

    /// # stale_days: If destroy_stale_apps is true, then apps will be destroyed after this many days
    pub fn stale_days(&mut self, stale_days: &'a str) -> &mut Self {
        self.params.stale_days = Some(stale_days);
        self
    }

    /// # destroy_stale_apps: If true, this will trigger automatic deletion of review apps when they’re stale
    pub fn destroy_stale_apps(&mut self, destroy_stale_apps: bool) -> &mut Self {
        self.params.destroy_stale_apps = Some(destroy_stale_apps);
        self
    }

    /// # automatic_review_apps: If true, this will trigger the creation of review apps when pull-requests are opened in the repo.
    pub fn automatic_review_apps(&mut self, automatic_review_apps: bool) -> &mut Self {
        self.params.automatic_review_apps = Some(automatic_review_apps);
        self
    }

    pub fn build(&self) -> ReviewAppConfigEnable<'a> {
        ReviewAppConfigEnable {
            pipeline_id: self.pipeline_id,
            params: ReviewAppConfigEnableParams {
                repo: self.params.repo,
                automatic_review_apps: self.params.automatic_review_apps,
                destroy_stale_apps: self.params.destroy_stale_apps,
                stale_days: self.params.stale_days,
                deploy_target: self.params.deploy_target.clone(),
                wait_for_ci: self.params.wait_for_ci,
                base_name: self.params.base_name,
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
