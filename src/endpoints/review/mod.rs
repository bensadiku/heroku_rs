use crate::framework::response::ApiResult;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::ReviewAppDelete;
pub use get::{ReviewAppByAppDetails, ReviewAppByPipelineList, ReviewAppDetails};
pub use post::{ReviewAppCreate, ReviewAppCreateParams};

impl ApiResult for ReviewApp {}
impl ApiResult for Vec<ReviewApp> {}

pub use review_app::ReviewApp;

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
