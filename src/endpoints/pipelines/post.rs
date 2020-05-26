//Anything related to POST requests for pipelines and it's properties goes here.
use super::{Pipeline, PipelineCoupling, PipelinePromotion, PipelineTransfer};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Pipeline Create
///
/// Create a new pipeline.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-create)
/// 
/// # Example:
///
/// PipelineCreate takes one required parameter, pipeline_name, and returns the created [`Pipeline`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&PipelineCreate::new("MYCOOLPIPELINE"));
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
pub struct PipelineCreate<'a> {
    /// The parameters to pass to the Heroku API
    pub params: PipelineCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> PipelineCreate<'a> {
    pub fn new(pipeline_name: &'a str) -> PipelineCreate<'a> {
        PipelineCreate {
            params: PipelineCreateParams {
                name: pipeline_name,
            },
        }
    }
}

/// Create a new pipeline with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct PipelineCreateParams<'a> {
    /// name of pipeline. pattern: ^[a-z][a-z0-9-]{2,29}$
    pub name: &'a str,
}

impl<'a> HerokuEndpoint<Pipeline, (), PipelineCreateParams<'a>> for PipelineCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("pipelines")
    }
    fn body(&self) -> Option<PipelineCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Pipeline Coupling Create
///
/// Create a new pipeline coupling.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-create)
/// 
/// # Example:
///
/// PipelineCouplingCreate takes three required parameters,app_id, pipeline_id and pipeline_stage and returns the created [`PipelineCoupling`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let app = "APP_ID"; // can be app name or app id
/// let pipeline = "PIPELINE_ID"; // pipeline id
/// let stage = "test"; // target pipeline stage
/// 
/// let response = api_client.request(&PipelineCouplingCreate::new(
///     app, pipeline, stage,
/// ));
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
pub struct PipelineCouplingCreate<'a> {
    /// The parameters to pass to the Heroku API
    pub params: PipelineCouplingCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> PipelineCouplingCreate<'a> {
    pub fn new(
        app_id: &'a str,
        pipeline_id: &'a str,
        pipeline_stage: &'a str,
    ) -> PipelineCouplingCreate<'a> {
        PipelineCouplingCreate {
            params: PipelineCouplingCreateParams {
                app: app_id,
                pipeline: pipeline_id,
                stage: pipeline_stage,
            },
        }
    }
}

/// Create a new pipeline coupling with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct PipelineCouplingCreateParams<'a> {
    /// unique identifier or name of app
    pub app: &'a str,
    /// unique identifier of pipeline
    pub pipeline: &'a str,
    /// target pipeline stage. one of:"test" or "review" or "development" or "staging" or "production"
    pub stage: &'a str,
}

impl<'a> HerokuEndpoint<PipelineCoupling, (), PipelineCouplingCreateParams<'a>>
    for PipelineCouplingCreate<'a>
{
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("pipeline-couplings")
    }
    fn body(&self) -> Option<PipelineCouplingCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Pipeline Promotion Create
///
/// Create a new promotion.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-promotion-create)
/// 
/// # Example:
///
/// PipelinePromotionCreate takes three required parameters,pipeline_id, source_app_id and target_app_id and returns the created [`PipelinePromotion`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let pipeline_id = "PIPELINE_ID";
/// let source_app_id = "SOURCE_APP_ID";
/// let target_app_id = "TARGET_APP_ID";
/// 
/// let response = api_client.request(&PipelinePromotionCreate::new(
///     pipeline_id,
///     source_app_id,
///     target_app_id,
/// ));
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
pub struct PipelinePromotionCreate<'a> {
    /// The parameters to pass to the Heroku API
    pub params: PipelinePromotionCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> PipelinePromotionCreate<'a> {
    pub fn new(
        pipeline_id: &'a str,
        source_app_id: &'a str,
        target_app_id: &'a str,
    ) -> PipelinePromotionCreate<'a> {
        PipelinePromotionCreate {
            params: PipelinePromotionCreateParams {
                pipeline: PipelineParam { id: pipeline_id },
                source: SourceParam {
                    app: AppParam { id: source_app_id },
                },
                targets: vec![TargetParam {
                    app: AppParam { id: target_app_id },
                }],
            },
        }
    }
}

/// Create a new pipeline promotion with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-promotion-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct PipelinePromotionCreateParams<'a> {
    pub pipeline: PipelineParam<'a>,
    pub source: SourceParam<'a>,
    pub targets: Vec<TargetParam<'a>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct PipelineParam<'a> {
    pub id: &'a str,
}

#[derive(Serialize, Clone, Debug)]
pub struct SourceParam<'a> {
    pub app: AppParam<'a>,
}

#[derive(Serialize, Clone, Debug)]
pub struct AppParam<'a> {
    pub id: &'a str,
}

#[derive(Serialize, Clone, Debug)]
pub struct TargetParam<'a> {
    pub app: AppParam<'a>,
}

impl<'a> HerokuEndpoint<PipelinePromotion, (), PipelinePromotionCreateParams<'a>>
    for PipelinePromotionCreate<'a>
{
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("pipeline-promotions")
    }
    fn body(&self) -> Option<PipelinePromotionCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Pipeline Transfer
///
/// A pipeline transfer is the process of changing pipeline ownership along with the contained apps.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-transfer)
/// 
/// # Example:
///
/// PipelinePromotionCreate takes three required parameters,pipeline_id, source_app_id and target_app_id and returns the created [`PipelinePromotion`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let pipeline_id = "PIPELINE_ID";
/// let owner_type = "user";
/// let new_owner_id = "OWNER_ID";
/// 
/// let response = api_client.request(&PipelineTransferCreate::new(
///     pipeline_id,
///     new_owner_id,
///     owner_type,
/// ));
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
pub struct PipelineTransferCreate<'a> {
    pub params: PipelineTransferCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> PipelineTransferCreate<'a> {
    pub fn new(
        pipeline_id: &'a str,
        new_owner_id: &'a str,
        new_owner_type: &'a str,
    ) -> PipelineTransferCreate<'a> {
        PipelineTransferCreate {
            params: PipelineTransferCreateParams {
                pipeline: PipelineParam { id: pipeline_id },
                new_owner: NewOwner {
                    id: new_owner_id,
                    type_field: new_owner_type,
                },
            },
        }
    }
}

/// Create a new pipeline transfer with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-transfer-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct PipelineTransferCreateParams<'a> {
    pub pipeline: PipelineParam<'a>,
    pub new_owner: NewOwner<'a>,
}

#[derive(Serialize, Clone, Debug)]
pub struct NewOwner<'a> {
    /// unique identifier of a pipeline owner
    pub id: &'a str,
    /// type of pipeline owner
    /// pattern: `(^team$
    #[serde(rename = "type")]
    pub type_field: &'a str,
}

impl<'a> HerokuEndpoint<PipelineTransfer, (), PipelineTransferCreateParams<'a>>
    for PipelineTransferCreate<'a>
{
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("pipeline-transfers")
    }
    fn body(&self) -> Option<PipelineTransferCreateParams<'a>> {
        Some(self.params.clone())
    }
}
