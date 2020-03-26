use crate::framework::response::ApiResult;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::{PipelineCouplingDelete, PipelineDelete};
pub use get::{
    PipelineCouplingByAppDetails, PipelineCouplingByPipelineList, PipelineCouplingByTeamList,
    PipelineCouplingByUserList, PipelineCouplingDetails, PipelineCouplingList,
    PipelineDeploymentList, PipelineDetails, PipelineLatestBuildsList, PipelineList,
    PipelinePromotionDetails,
};
pub use patch::{
    PipelineCouplingUpdate, PipelineCouplingUpdateParams, PipelineUpdate, PipelineUpdateParams,
};
pub use post::{
    PipelineCouplingCreate, PipelineCouplingCreateParams, PipelineCreate, PipelineCreateParams,
    PipelinePromotionCreate, PipelinePromotionCreateParams,
};

pub use pipeline::Pipeline;
pub use pipeline_builds::PipelineBuild;
pub use pipeline_couplings::PipelineCoupling;
pub use pipeline_deployement::PipelineDeployment;
pub use pipeline_promotions::PipelinePromotion;

impl ApiResult for Pipeline {}
impl ApiResult for Vec<Pipeline> {}

impl ApiResult for PipelineBuild {}
impl ApiResult for Vec<PipelineBuild> {}

impl ApiResult for PipelineCoupling {}
impl ApiResult for Vec<PipelineCoupling> {}

impl ApiResult for PipelineDeployment {}
impl ApiResult for Vec<PipelineDeployment> {}

impl ApiResult for PipelinePromotion {}
impl ApiResult for Vec<PipelinePromotion> {}

// pipeline submodule, anything from /pipelines goes here.
mod pipeline {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Pipeline
    ///
    /// Stability: production
    ///
    /// A pipeline allows grouping of apps into different stages.
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline)
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Pipeline {
        /// when pipeline was created
        pub created_at: DateTime<Utc>,
        /// unique identifier of pipeline
        pub id: String,
        /// name of pipeline. pattern: ^[a-z][a-z0-9-]{2,29}$
        pub name: String,
        /// Owner of a pipeline.
        pub owner: Option<Owner>,
        /// when pipeline was updated
        pub updated_at: DateTime<Utc>,
    }

    /// Pipeline owner
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Owner {
        /// unique identifier of a pipeline owner
        pub id: String,
        /// type of pipeline owner. pattern: `(^team$
        #[serde(rename = "type")]
        pub type_field: String,
    }
}
// pipeline build submodule, anything from /pipelines/{pipeline_id_or_name}/latest-builds goes here.
mod pipeline_builds {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Pipeline Builds
    ///
    /// Stability: production
    ///
    /// Information about latest builds of apps in a pipeline.
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-build)
    // TODO (ben): The nullability of these properties is not documentented. If something we're to fail due to a `invalid type: null, expected a string.` update these.
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct PipelineBuild {
        /// the app that the build is ran for
        pub app: App,
        /// buildpacks it's using
        pub buildpacks: Vec<Buildpack>,
        /// when build was created
        pub created_at: DateTime<Utc>,
        /// unique identifier
        pub id: String,
        /// Build process output will be available from this URL as a stream.
        pub output_stream_url: Option<String>,
        /// Source blob struct containing url, checksum and version
        pub source_blob: SourceBlob,
        /// A release represents a combination of code, config vars and add-ons for an app on Heroku.
        pub release: Option<Release>,
        /// slug created by this build
        pub slug: Option<Slug>,
        /// Pipeline Stack
        pub stack: String,
        /// pipeline status
        pub status: String,
        /// when the pipeline was last updated
        pub updated_at: DateTime<Utc>,
        /// user account
        pub user: User,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct App {
        /// unique identifier of the app
        pub id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Buildpack {
        /// bp url
        pub url: String,
        /// unique name
        pub name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct SourceBlob {
        /// an optional checksum of the gzipped tarball for verifying its integrity
        pub checksum: Option<String>,
        /// The download location for the source code to be tested
        pub url: String,
        /// Version of the gzipped tarball.
        pub version: Option<String>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Release {
        /// unique identifier of the release
        pub id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Slug {
        /// unique identifier of the slug
        pub id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct User {
        /// unique identifier of the user
        pub id: String,
        /// email of the user
        pub email: String,
    }
}

// pipeline couplings submodule, anything for pipeline couplings goes here.
mod pipeline_couplings {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Pipeline Coupling
    ///
    /// Stability: production
    ///
    /// Information about an app’s coupling to a pipeline
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling)
    // TODO (ben): The nullability of these properties is not documentented. If something we're to fail due to a `invalid type: null, expected a string.` update these.
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct PipelineCoupling {
        /// the app that this pipeline coupling belongs to
        pub app: App,
        /// when pipeline coupling was created
        pub created_at: DateTime<Utc>,
        /// unique identifier of pipeline coupling
        pub id: String,
        /// the pipeline that this pipeline coupling belongs to
        pub pipeline: Pipeline,
        /// target pipeline stage. one of:"test" or "review" or "development" or "staging" or "production"
        pub stage: String,
        /// when pipeline coupling was updated
        pub updated_at: DateTime<Utc>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct App {
        /// unique identifier of the app
        pub id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Pipeline {
        /// unique identifier of the pipeline
        pub id: String,
    }
}

// pipeline deployment submodule, anything for pipeline deployment goes here.
mod pipeline_deployement {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Pipeline Deployment
    ///
    /// Stability: production
    ///
    /// Information about latest deployments of apps in a pipeline.
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-deployment)
    // TODO (ben): The nullability of these properties is not documentented. If something we're to fail due to a `invalid type: null, expected a value.` update these.
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct PipelineDeployment {
        pub addon_plan_names: Vec<String>,
        pub app: App,
        pub created_at: DateTime<Utc>,
        pub description: String,
        pub id: String,
        pub updated_at: DateTime<Utc>,
        pub slug: Slug,
        pub status: String,
        pub user: User,
        pub version: i64,
        pub current: bool,
        pub output_stream_url: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct App {
        /// app name
        pub name: String,
        /// unique identifier of the app
        pub id: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Slug {
        /// unique identifier of the slug
        pub id: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct User {
        /// unique identifier of the user
        pub id: String,
        /// user email
        pub email: String,
    }
}

// pipeline promotions submodule, anything for pipeline promotions goes here.
mod pipeline_promotions {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Pipeline Promotion
    ///
    /// Stability: production
    ///
    /// Promotions allow you to move code from an app in a pipeline to all targets
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-promotion)
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct PipelinePromotion {
        /// when promotion was created
        pub created_at: DateTime<Utc>,
        /// unique identifier of promotion
        pub id: String,
        /// pipeline
        pub pipeline: Pipeline,
        /// source
        pub source: Source,
        /// status of promotion. one of:"pending" or "completed"
        pub status: String,
        /// when promotion was updated
        pub updated_at: Option<DateTime<Utc>>,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Pipeline {
        pub id: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Source {
        pub app: App,
        pub release: Release,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct App {
        pub id: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Release {
        pub id: String,
    }
}
