//Anything related to PATCH requests for pipelines and it's properties goes here.
use super::{Pipeline, PipelineCoupling};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Pipeline Update
///
/// Update an existing pipeline.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-update)
pub struct PipelineUpdate {
    /// unique pipeline identifier.
    pub pipeline_id: String,
    /// The parameters to pass to the Heroku API
    pub params: PipelineUpdateParams,
}

impl PipelineUpdate {
    pub fn new(pipeline_id: String, name: Option<String>) -> PipelineUpdate {
        PipelineUpdate {
            pipeline_id,
            params: PipelineUpdateParams { name: name },
        }
    }
}

/// Update pipeline with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-update-optional-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct PipelineUpdateParams {
    /// name of pipeline. pattern: ^[a-z][a-z0-9-]{2,29}$
    pub name: Option<String>,
}

impl HerokuEndpoint<Pipeline, (), PipelineUpdateParams> for PipelineUpdate {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("pipelines/{}", self.pipeline_id)
    }
    fn body(&self) -> Option<PipelineUpdateParams> {
        Some(self.params.clone())
    }
}

/// Pipeline Coupling Update
///
/// Update an existing pipeline coupling.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-update)
pub struct PipelineCouplingUpdate {
    /// unique pipeline coupling identifier.
    pub coupling_id: String,
    /// The parameters to pass to the Heroku API
    pub params: PipelineCouplingUpdateParams,
}

impl PipelineCouplingUpdate {
    pub fn new(coupling_id: String, stage: String) -> PipelineCouplingUpdate {
        PipelineCouplingUpdate {
            coupling_id,
            params: PipelineCouplingUpdateParams { stage },
        }
    }
}

/// Update pipeline coupling with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-update-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct PipelineCouplingUpdateParams {
    /// target pipeline stage. one of:"test" or "review" or "development" or "staging" or "production"
    pub stage: String,
}

impl HerokuEndpoint<PipelineCoupling, (), PipelineCouplingUpdateParams> for PipelineCouplingUpdate {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("pipeline-couplings/{}", self.coupling_id)
    }
    fn body(&self) -> Option<PipelineCouplingUpdateParams> {
        Some(self.params.clone())
    }
}
