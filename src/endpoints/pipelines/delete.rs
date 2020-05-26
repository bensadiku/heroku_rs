//Anything related to DELETE requests for pipelines and it's properties goes here.
use super::{Pipeline, PipelineCoupling};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Pipeline Delete
///
/// Delete an existing pipeline.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-delete)
/// 
/// # Example:
///
/// PipelineDelete takes one required parameter, pipeline_id, and returns the deleted [`Pipeline`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&PipelineDelete::new("PIPELINE_ID"));
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
pub struct PipelineDelete<'a> {
    /// unique pipeline identifier.
    pub pipeline_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> PipelineDelete<'a> {
    pub fn new(pipeline_id: &'a str) -> PipelineDelete<'a> {
        PipelineDelete { pipeline_id }
    }
}

impl<'a> HerokuEndpoint<Pipeline> for PipelineDelete<'a> {
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
/// 
/// # Example:
///
/// PipelineCouplingDelete takes one required parameter, coupling_id, and returns the deleted [`PipelineCoupling`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&PipelineCouplingDelete::new("COUPLING_ID"));
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
pub struct PipelineCouplingDelete<'a> {
    /// unique pipeline coupling identifier.
    pub coupling_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> PipelineCouplingDelete<'a> {
    pub fn new(coupling_id: &'a str) -> PipelineCouplingDelete<'a> {
        PipelineCouplingDelete { coupling_id }
    }
}

impl<'a> HerokuEndpoint<PipelineCoupling> for PipelineCouplingDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("pipeline-couplings/{}", self.coupling_id)
    }
}
