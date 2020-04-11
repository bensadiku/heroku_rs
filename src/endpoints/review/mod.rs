use crate::framework::response::ApiResult;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::{ReviewAppConfigDelete, ReviewAppDelete};
pub use get::{
    ReviewAppByAppDetails, ReviewAppByPipelineList, ReviewAppConfigDetails, ReviewAppDetails,
};
pub use patch::{ReviewAppConfigUpdate, ReviewAppConfigUpdateParams};
pub use post::{
    ReviewAppConfigEnable, ReviewAppConfigEnableParams, ReviewAppCreate, ReviewAppCreateParams,
};

impl ApiResult for ReviewApp {}
impl ApiResult for Vec<ReviewApp> {}

impl ApiResult for ReviewAppConfig {}
impl ApiResult for Vec<ReviewAppConfig> {}

pub use review_app::ReviewApp;
pub use review_app_config::ReviewAppConfig;

// review app submodule, anything from /review-apps goes here.
mod review_app {
    use chrono::offset::Utc;
    use chrono::DateTime;
    use serde_json::Value;

    /// Review App
    ///
    /// Stability: production
    ///
    /// An ephemeral app to review a set of changes
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#review-app)
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ReviewApp {
        /// the Heroku app associated to this review app
        pub app: Option<App>,
        /// the app setup for this review app
        pub app_setup: Option<AppSetup>,
        /// the branch of the repository which the review app is based on
        pub branch: String,
        /// when test run was created
        pub created_at: DateTime<Utc>,
        /// unique identifier of the review app
        pub id: String,
        /// the pipeline associated to the review app
        pub pipeline: Pipeline,
        /// current state of the review app
        ///  one of:"pending" or "creating" or "created" or "deleting" or "deleted" or "errored"
        pub status: String,
        /// when review app was updated
        pub updated_at: DateTime<Utc>,
        /// The user who created the review app
        pub creator: Value,
        /// wait for ci before building the app
        pub wait_for_ci: bool,
        /// error message from creating the review app if any
        pub error_status: Option<String>,
        /// message from creating the review app if any
        pub message: Option<String>,
        /// fork repo
        pub fork_repo: Option<ForkRepo>,
        /// GitHub Pull Request number if the Review app was created automatically
        pub pr_number: Option<i64>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct App {
        /// unique identifier
        pub id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct AppSetup {
        /// unique identifier of app setup
        pub id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Pipeline {
        /// unique identifier of pipeline
        pub id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct ForkRepo {
        /// repository id of the fork the branch resides in
        pub id: Option<String>,
    }
}

mod review_app_config {
    /// Review App Configuration
    ///
    /// Stability: production
    ///
    /// Review apps can be configured for pipelines.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#review-app-configuration)
    // TODO(ben): Heroku docs have the wrong response in the documentation on the pipeline field. 
    //      It's represented as a `pipeline_id: String` field, but in fact in a Pipeline object with the id field. Double check edge cases.
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ReviewAppConfig {
        /// repo
        pub repo: Repo,
        /// enable automatic review apps for pull requests
        pub automatic_review_apps: Option<bool>,
        /// the deploy target for the review apps of a pipeline
        pub deploy_target: Option<DeployTarget>,
        /// automatically destroy review apps when they haven’t been deployed for a number of days
        pub destroy_stale_apps: bool,
        /// number of days without a deployment after which to consider a review app stale
        pub stale_days: i32,
        /// unique identifier of pipeline
        pub pipeline: Pipeline,
        /// If true, review apps are created only when CI passes
        pub wait_for_ci: bool,
        /// A unique prefix that will be used to create review app names
        pub base_name: Option<String>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct Repo {
        /// repository id
        pub id: i32,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct Pipeline {
        /// pipeline id
        pub id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DeployTarget {
        /// unique identifier of deploy target
        ///  pattern: `(^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$
        pub id: String,
        #[serde(rename = "type")]
        /// type of deploy target
        ///  pattern: `(^space$
        pub type_field: String,
    }
}
