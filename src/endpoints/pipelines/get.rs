//Anything related to GET requests for pipelines and it's properties goes here.
use super::{
    Pipeline, PipelineBuild, PipelineCoupling, PipelineDeployment, PipelinePromotion,
    PipelinePromotionTarget, PipelineRelease, PipelineStack,
};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Pipeline Info
///
/// Info for existing pipeline.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-info)
pub struct PipelineDetails {
    /// unique pipeline identifier.
    pub pipeline_id: String,
}

impl PipelineDetails {
    pub fn new(pipeline_id: String) -> PipelineDetails {
        PipelineDetails { pipeline_id }
    }
}

impl HerokuEndpoint<Pipeline> for PipelineDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines/{}", self.pipeline_id)
    }
}

/// Pipeline List
///
/// List existing pipelines.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-list)
pub struct PipelineList {}

impl PipelineList {
    pub fn new() -> PipelineList {
        PipelineList {}
    }
}

impl HerokuEndpoint<Vec<Pipeline>> for PipelineList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines")
    }
}

/// Pipeline Build List
///
/// List latest builds for each app in a pipeline
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-build-list)
pub struct PipelineLatestBuildsList {
    /// unique pipeline identifier.
    pub pipeline_id: String,
}

impl PipelineLatestBuildsList {
    pub fn new(pipeline_id: String) -> PipelineLatestBuildsList {
        PipelineLatestBuildsList { pipeline_id }
    }
}

impl HerokuEndpoint<Vec<PipelineBuild>> for PipelineLatestBuildsList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines/{}/latest-builds", self.pipeline_id)
    }
}

/// Pipeline Coupling List By Pipeline
///
/// List couplings for a pipeline
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-list-by-pipeline)
pub struct PipelineCouplingByPipelineList {
    /// unique pipeline identifier.
    pub pipeline_id: String,
}

impl PipelineCouplingByPipelineList {
    pub fn new(pipeline_id: String) -> PipelineCouplingByPipelineList {
        PipelineCouplingByPipelineList { pipeline_id }
    }
}

impl HerokuEndpoint<Vec<PipelineCoupling>> for PipelineCouplingByPipelineList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines/{}/pipeline-couplings", self.pipeline_id)
    }
}

/// Pipeline Coupling List By Current User
///
/// List pipeline couplings for the current user.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-list-by-current-user)
pub struct PipelineCouplingByUserList {}

impl PipelineCouplingByUserList {
    pub fn new() -> PipelineCouplingByUserList {
        PipelineCouplingByUserList {}
    }
}

impl HerokuEndpoint<Vec<PipelineCoupling>> for PipelineCouplingByUserList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("users/~/pipeline-couplings")
    }
}

/// Pipeline Coupling List By Team
///
/// List pipeline couplings for a team.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-list-by-team)
pub struct PipelineCouplingByTeamList {
    /// unique team identifier.
    pub team_id: String,
}

impl PipelineCouplingByTeamList {
    pub fn new(team_id: String) -> PipelineCouplingByTeamList {
        PipelineCouplingByTeamList { team_id }
    }
}

impl HerokuEndpoint<Vec<PipelineCoupling>> for PipelineCouplingByTeamList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("teams/{}/pipeline-couplings", self.team_id)
    }
}

/// Pipeline Coupling Info By App
///
/// Info for an existing pipeline coupling.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-info-by-app)
pub struct PipelineCouplingByAppDetails {
    /// unique app identifier.
    pub app_id: String,
}

impl PipelineCouplingByAppDetails {
    pub fn new(app_id: String) -> PipelineCouplingByAppDetails {
        PipelineCouplingByAppDetails { app_id }
    }
}

impl HerokuEndpoint<PipelineCoupling> for PipelineCouplingByAppDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/pipeline-couplings", self.app_id)
    }
}

/// Pipeline Coupling List
///
/// List pipeline couplings.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-list)
pub struct PipelineCouplingList {}

impl PipelineCouplingList {
    pub fn new() -> PipelineCouplingList {
        PipelineCouplingList {}
    }
}

impl HerokuEndpoint<Vec<PipelineCoupling>> for PipelineCouplingList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipeline-couplings")
    }
}

/// Pipeline Coupling Info
///
/// Info for an existing pipeline coupling.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-info)
pub struct PipelineCouplingDetails {
    /// unique pipeline coupling identifier.
    pub coupling_id: String,
}

impl PipelineCouplingDetails {
    pub fn new(coupling_id: String) -> PipelineCouplingDetails {
        PipelineCouplingDetails { coupling_id }
    }
}

impl HerokuEndpoint<PipelineCoupling> for PipelineCouplingDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipeline-couplings/{}", self.coupling_id)
    }
}

/// Pipeline Deployment List
///
/// List latest slug releases for each app in a pipeline
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-deployment-list)
pub struct PipelineDeploymentList {
    /// unique pipeline identifier.
    pub pipeline_id: String,
}

impl PipelineDeploymentList {
    pub fn new(pipeline_id: String) -> PipelineDeploymentList {
        PipelineDeploymentList { pipeline_id }
    }
}

impl HerokuEndpoint<Vec<PipelineDeployment>> for PipelineDeploymentList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines/{}/latest-deployments", self.pipeline_id)
    }
}

/// Pipeline Promotion Info
///
/// Info for existing pipeline promotion.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-promotion-info)
pub struct PipelinePromotionDetails {
    /// unique pipeline identifier.
    pub promotion_id: String,
}

impl PipelinePromotionDetails {
    pub fn new(promotion_id: String) -> PipelinePromotionDetails {
        PipelinePromotionDetails { promotion_id }
    }
}

impl HerokuEndpoint<PipelinePromotion> for PipelinePromotionDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipeline-promotions/{}", self.promotion_id)
    }
}

/// Pipeline Promotion Target List
///
/// List promotion targets belonging to an existing promotion.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-promotion-target-list)
pub struct PipelinePromotionTargetList {
    /// unique pipeline identifier.
    pub promotion_id: String,
}

impl PipelinePromotionTargetList {
    pub fn new(promotion_id: String) -> PipelinePromotionTargetList {
        PipelinePromotionTargetList { promotion_id }
    }
}

impl HerokuEndpoint<Vec<PipelinePromotionTarget>> for PipelinePromotionTargetList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "pipeline-promotions/{}/promotion-targets",
            self.promotion_id
        )
    }
}

/// Pipeline Release
///
/// Information about latest releases of apps in a pipeline.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-release)
pub struct PipelineLatestReleaseList {
    /// unique pipeline identifier.
    pub pipeline_id: String,
}

impl PipelineLatestReleaseList {
    pub fn new(pipeline_id: String) -> PipelineLatestReleaseList {
        PipelineLatestReleaseList { pipeline_id }
    }
}

impl HerokuEndpoint<Vec<PipelineRelease>> for PipelineLatestReleaseList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines/{}/latest-releases", self.pipeline_id)
    }
}

/// Pipeline Stack
///
/// Information about latest releases of apps in a pipeline.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-release)
pub struct PipelineStackDetails {
    /// unique pipeline identifier.
    pub pipeline_id: String,
}

impl PipelineStackDetails {
    pub fn new(pipeline_id: String) -> PipelineStackDetails {
        PipelineStackDetails { pipeline_id }
    }
}

impl HerokuEndpoint<PipelineStack> for PipelineStackDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines/{}/pipeline-stack", self.pipeline_id)
    }
}
