//Anything related to DELETE requests for pipelines and it's properties goes here.
use super::{Pipeline, PipelineCoupling};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Pipeline Delete
///
/// Delete an existing pipeline.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-delete)
pub struct PipelineDelete {
    /// unique pipeline identifier.
    pub pipeline_id: String,
}

impl PipelineDelete {
    pub fn new(pipeline_id: String) -> PipelineDelete {
        PipelineDelete { pipeline_id }
    }
}

impl HerokuEndpoint<Pipeline, (), ()> for PipelineDelete {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("pipelines/{}", self.pipeline_id)
    }
}

/// Pipeline Coupling Delete
///
/// Delete an existing pipeline coupling.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-delete)
pub struct PipelineCouplingDelete {
    /// unique pipeline coupling identifier.
    pub coupling_id: String,
}

impl PipelineCouplingDelete {
    pub fn new(coupling_id: String) -> PipelineCouplingDelete {
        PipelineCouplingDelete { coupling_id }
    }
}

impl HerokuEndpoint<PipelineCoupling, (), ()> for PipelineCouplingDelete {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("pipeline-couplings/{}", self.coupling_id)
    }
}
