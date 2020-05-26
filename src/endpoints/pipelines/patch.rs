//Anything related to PATCH requests for pipelines and it's properties goes here.
use super::{Pipeline, PipelineCoupling};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Pipeline Update
///
/// Update an existing pipeline.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-update)
/// 
/// # Example:
///
/// PipelineUpdate takes one required parameter, pipeline_id, and returns the updated [`Pipeline`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&PipelineUpdate::new("PIPELINE_ID"));
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
pub struct PipelineUpdate<'a> {
    /// unique pipeline identifier.
    pub pipeline_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: PipelineUpdateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> PipelineUpdate<'a> {
    pub fn new(pipeline_id: &'a str) -> PipelineUpdate<'a> {
        PipelineUpdate {
            pipeline_id,
            params: PipelineUpdateParams { name: None },
        }
    }

    /// # name: name of pipeline
    /// 
    /// `pattern`: ^[a-z][a-z0-9-]{2,29}$ 
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.params.name = Some(name);
        self
    }

    pub fn build(&self) -> PipelineUpdate<'a> {
        PipelineUpdate {
            pipeline_id: self.pipeline_id,
            params: PipelineUpdateParams {
                name: self.params.name,
            },
        }
    }
}

/// Update pipeline with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-update-optional-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct PipelineUpdateParams<'a> {
    /// name of pipeline. pattern: ^[a-z][a-z0-9-]{2,29}$
    pub name: Option<&'a str>,
}

impl<'a> HerokuEndpoint<Pipeline, (), PipelineUpdateParams<'a>> for PipelineUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("pipelines/{}", self.pipeline_id)
    }
    fn body(&self) -> Option<PipelineUpdateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Pipeline Coupling Update
///
/// Update an existing pipeline coupling.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-update)
/// 
/// # Example:
///
/// PipelineCouplingUpdate takes one required parameter, coupling_id, and returns the updated [`PipelineCoupling`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(
///     &PipelineCouplingUpdate::new("COUPLING_ID")
///         .stage("development")
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
/// [response]: ../struct.PipelineCoupling.html
pub struct PipelineCouplingUpdate<'a> {
    /// unique pipeline coupling identifier.
    pub coupling_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: PipelineCouplingUpdateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> PipelineCouplingUpdate<'a> {
    pub fn new(coupling_id: &'a str) -> PipelineCouplingUpdate {
        PipelineCouplingUpdate {
            coupling_id,
            params: PipelineCouplingUpdateParams { stage: None },
        }
    }

    /// # stage: target pipeline stage
    /// 
    /// `one of` : "test" or "review" or "development" or "staging" or "production" 
    pub fn stage(&mut self, stage: &'a str) -> &mut Self {
        self.params.stage = Some(stage);
        self
    }

    pub fn build(&self) -> PipelineCouplingUpdate<'a> {
        PipelineCouplingUpdate {
            coupling_id: self.coupling_id,
            params: PipelineCouplingUpdateParams {
                stage: self.params.stage,
            },
        }
    }
}

/// Update pipeline coupling with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-update-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct PipelineCouplingUpdateParams<'a> {
    /// target pipeline stage. one of:"test" or "review" or "development" or "staging" or "production"
    pub stage: Option<&'a str>,
}

impl<'a> HerokuEndpoint<PipelineCoupling, (), PipelineCouplingUpdateParams<'a>>
    for PipelineCouplingUpdate<'a>
{
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("pipeline-couplings/{}", self.coupling_id)
    }
    fn body(&self) -> Option<PipelineCouplingUpdateParams<'a>> {
        Some(self.params.clone())
    }
}
