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
/// 
/// # Example:
///
/// PipelineDetails takes one required parameter, pipeline_id, and returns a [`Pipeline`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&PipelineDetails::new("PIPELINE_ID"));
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
/// [response]: ../struct.Pipeline.html
pub struct PipelineDetails<'a> {
    /// unique pipeline identifier.
    pub pipeline_id: &'a str,
}

#[cfg(feature = "builder")]
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
/// 
/// # Example:
///
/// PipelineList takes no required parameters, and returns a list of [`Pipelines`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&PipelineList::new());
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
/// [response]: ../struct.Pipeline.html
pub struct PipelineList {}

#[cfg(feature = "builder")]
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
/// 
/// # Example:
///
/// PipelineLatestBuildsList takes one required parameter, pipeline_id, and returns a list of [`PipelineBuild`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&PipelineLatestBuildsList::new("PIPELINE_ID"));
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
/// [response]: ../struct.PipelineBuild.html
pub struct PipelineLatestBuildsList<'a> {
    /// unique pipeline identifier.
    pub pipeline_id: &'a str,
}

#[cfg(feature = "builder")]
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
/// 
/// # Example:
///
/// PipelineCouplingByPipelineList takes one required parameter, pipeline_id, and returns a list of [`PipelineCoupling`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&PipelineCouplingByPipelineList::new("PIPELINE_ID"));
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
/// [response]: ../struct.PipelineCoupling.html
pub struct PipelineCouplingByPipelineList<'a> {
    /// unique pipeline identifier.
    pub pipeline_id: &'a str,
}

#[cfg(feature = "builder")]
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
/// 
/// # Example:
///
/// PipelineCouplingByUserList takes no required parameters, and returns a list of [`PipelineCoupling`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&PipelineCouplingByUserList::new());
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
/// [response]: ../struct.PipelineCoupling.html
pub struct PipelineCouplingByUserList {}

#[cfg(feature = "builder")]
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
/// 
/// # Example:
///
/// PipelineCouplingByTeamList takes one required parameter, team_id, and returns a list of [`PipelineCoupling`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&PipelineCouplingByTeamList::new("TEAM_ID"));
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
/// [response]: ../struct.PipelineCoupling.html
pub struct PipelineCouplingByTeamList<'a> {
    /// unique team identifier.
    pub team_id: &'a str,
}

#[cfg(feature = "builder")]
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
/// 
/// # Example:
///
/// PipelineCouplingByAppDetails takes one required parameter, app_id, and returns a [`PipelineCoupling`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&PipelineCouplingByAppDetails::new("APP_ID"));
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
/// [response]: ../struct.PipelineCoupling.html
pub struct PipelineCouplingByAppDetails<'a> {
    /// unique app identifier.
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
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
/// 
/// # Example:
///
/// PipelineCouplingList takes no required parameters, and returns a list of [`PipelineCoupling`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&PipelineCouplingList::new());
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
/// [response]: ../struct.PipelineCoupling.html
pub struct PipelineCouplingList {}

#[cfg(feature = "builder")]
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
/// 
/// # Example:
///
/// PipelineCouplingDetails takes no required parameters, and returns a [`PipelineCoupling`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&PipelineCouplingDetails::new("COUPLING_ID"));
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
/// [response]: ../struct.PipelineCoupling.html
pub struct PipelineCouplingDetails<'a> {
    /// unique pipeline coupling identifier.
    pub coupling_id: &'a str,
}

#[cfg(feature = "builder")]
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
/// 
/// # Example:
///
/// PipelineDeploymentList takes one required parameter, pipeline_id, and returns a list of [`PipelineDeployment`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&PipelineDeploymentList::new("PIPELINE_ID"));
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
/// [response]: ../struct.PipelineDeployment.html
pub struct PipelineDeploymentList<'a> {
    /// unique pipeline identifier.
    pub pipeline_id: &'a str,
}

#[cfg(feature = "builder")]
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
/// 
/// # Example:
///
/// PipelinePromotionDetails takes one required parameter, promotion_id, and returns a [`PipelinePromotion`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&PipelinePromotionDetails::new("PROMOTION_ID"));
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
/// [response]: ../struct.PipelinePromotion.html
pub struct PipelinePromotionDetails<'a> {
    /// unique pipeline identifier.
    pub promotion_id: &'a str,
}

#[cfg(feature = "builder")]
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
/// 
/// # Example:
///
/// PipelinePromotionTargetList takes one required parameter, promotion_id, and returns a list of [`PipelinePromotionTarget`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&PipelinePromotionTargetList::new("PROMOTION_ID"));
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
/// [response]: ../struct.PipelinePromotionTarget.html
pub struct PipelinePromotionTargetList<'a> {
    /// unique pipeline identifier.
    pub promotion_id: &'a str,
}

#[cfg(feature = "builder")]
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
/// 
/// # Example:
///
/// PipelineLatestReleaseList takes one required parameter, pipeline_id, and returns a list of [`PipelineRelease`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&PipelineLatestReleaseList::new("PIPELINE_ID"));
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
/// [response]: ../struct.PipelineRelease.html
pub struct PipelineLatestReleaseList<'a> {
    /// unique pipeline identifier.
    pub pipeline_id: &'a str,
}

#[cfg(feature = "builder")]
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
/// 
/// # Example:
///
/// PipelineStackDetails takes one required parameter, pipeline_id, and returns a [`PipelineStack`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&PipelineStackDetails::new("PIPELINE_ID"));
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
/// [response]: ../struct.PipelineStack.html
pub struct PipelineStackDetails<'a> {
    /// unique pipeline identifier.
    pub pipeline_id: &'a str,
}

#[cfg(feature = "builder")]
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
