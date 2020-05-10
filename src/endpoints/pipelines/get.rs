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
pub struct PipelineDetails<'a> {
    /// unique pipeline identifier.
    pub pipeline_id: &'a str,
}

impl<'a> PipelineDetails<'a> {
    pub fn new(pipeline_id: &'a str) -> PipelineDetails<'a> {
        PipelineDetails { pipeline_id }
    }
}

impl<'a> HerokuEndpoint<Pipeline> for PipelineDetails<'a> {
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
pub struct PipelineLatestBuildsList<'a> {
    /// unique pipeline identifier.
    pub pipeline_id: &'a str,
}

impl<'a> PipelineLatestBuildsList<'a> {
    pub fn new(pipeline_id: &'a str) -> PipelineLatestBuildsList<'a> {
        PipelineLatestBuildsList { pipeline_id }
    }
}

impl<'a> HerokuEndpoint<Vec<PipelineBuild>> for PipelineLatestBuildsList<'a> {
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
pub struct PipelineCouplingByPipelineList<'a> {
    /// unique pipeline identifier.
    pub pipeline_id: &'a str,
}

impl<'a> PipelineCouplingByPipelineList<'a> {
    pub fn new(pipeline_id: &'a str) -> PipelineCouplingByPipelineList<'a> {
        PipelineCouplingByPipelineList { pipeline_id }
    }
}

impl<'a> HerokuEndpoint<Vec<PipelineCoupling>> for PipelineCouplingByPipelineList<'a> {
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
pub struct PipelineCouplingByTeamList<'a> {
    /// unique team identifier.
    pub team_id: &'a str,
}

impl<'a> PipelineCouplingByTeamList<'a> {
    pub fn new(team_id: &'a str) -> PipelineCouplingByTeamList<'a> {
        PipelineCouplingByTeamList { team_id }
    }
}

impl<'a> HerokuEndpoint<Vec<PipelineCoupling>> for PipelineCouplingByTeamList<'a> {
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
pub struct PipelineCouplingByAppDetails<'a> {
    /// unique app identifier.
    pub app_id: &'a str,
}

impl<'a> PipelineCouplingByAppDetails<'a> {
    pub fn new(app_id: &'a str) -> PipelineCouplingByAppDetails<'a> {
        PipelineCouplingByAppDetails { app_id }
    }
}

impl<'a> HerokuEndpoint<PipelineCoupling> for PipelineCouplingByAppDetails<'a> {
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
pub struct PipelineCouplingDetails<'a> {
    /// unique pipeline coupling identifier.
    pub coupling_id: &'a str,
}

impl<'a> PipelineCouplingDetails<'a> {
    pub fn new(coupling_id: &'a str) -> PipelineCouplingDetails<'a> {
        PipelineCouplingDetails { coupling_id }
    }
}

impl<'a> HerokuEndpoint<PipelineCoupling> for PipelineCouplingDetails<'a> {
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
pub struct PipelineDeploymentList<'a> {
    /// unique pipeline identifier.
    pub pipeline_id: &'a str,
}

impl<'a> PipelineDeploymentList<'a> {
    pub fn new(pipeline_id: &'a str) -> PipelineDeploymentList<'a> {
        PipelineDeploymentList { pipeline_id }
    }
}

impl<'a> HerokuEndpoint<Vec<PipelineDeployment>> for PipelineDeploymentList<'a> {
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
pub struct PipelinePromotionDetails<'a> {
    /// unique pipeline identifier.
    pub promotion_id: &'a str,
}

impl<'a> PipelinePromotionDetails<'a> {
    pub fn new(promotion_id: &'a str) -> PipelinePromotionDetails<'a> {
        PipelinePromotionDetails { promotion_id }
    }
}

impl<'a> HerokuEndpoint<PipelinePromotion> for PipelinePromotionDetails<'a> {
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
pub struct PipelinePromotionTargetList<'a> {
    /// unique pipeline identifier.
    pub promotion_id: &'a str,
}

impl<'a> PipelinePromotionTargetList<'a> {
    pub fn new(promotion_id: &'a str) -> PipelinePromotionTargetList<'a> {
        PipelinePromotionTargetList { promotion_id }
    }
}

impl<'a> HerokuEndpoint<Vec<PipelinePromotionTarget>> for PipelinePromotionTargetList<'a> {
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
pub struct PipelineLatestReleaseList<'a> {
    /// unique pipeline identifier.
    pub pipeline_id: &'a str,
}

impl<'a> PipelineLatestReleaseList<'a> {
    pub fn new(pipeline_id: &'a str) -> PipelineLatestReleaseList<'a> {
        PipelineLatestReleaseList { pipeline_id }
    }
}

impl<'a> HerokuEndpoint<Vec<PipelineRelease>> for PipelineLatestReleaseList<'a> {
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
pub struct PipelineStackDetails<'a> {
    /// unique pipeline identifier.
    pub pipeline_id: &'a str,
}

impl<'a> PipelineStackDetails<'a> {
    pub fn new(pipeline_id: &'a str) -> PipelineStackDetails<'a> {
        PipelineStackDetails { pipeline_id }
    }
}

impl<'a> HerokuEndpoint<PipelineStack> for PipelineStackDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines/{}/pipeline-stack", self.pipeline_id)
    }
}
