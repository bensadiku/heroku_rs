//Anything related to POST requests for pipelines and it's properties goes here.
use super::{Pipeline, PipelineCoupling, PipelinePromotion, PipelineTransfer};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Pipeline Create
///
/// Create a new pipeline.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-create)
pub struct PipelineCreate {
    /// The parameters to pass to the Heroku API
    pub params: PipelineCreateParams,
}

impl PipelineCreate {
    pub fn new(pipeline_name: String) -> PipelineCreate {
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
pub struct PipelineCreateParams {
    /// name of pipeline. pattern: ^[a-z][a-z0-9-]{2,29}$
    pub name: String,
}

impl HerokuEndpoint<Pipeline, (), PipelineCreateParams> for PipelineCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("pipelines")
    }
    fn body(&self) -> Option<PipelineCreateParams> {
        Some(self.params.clone())
    }
}

/// Pipeline Coupling Create
///
/// Create a new pipeline coupling.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-coupling-create)
pub struct PipelineCouplingCreate {
    /// The parameters to pass to the Heroku API
    pub params: PipelineCouplingCreateParams,
}

impl PipelineCouplingCreate {
    pub fn new(
        app_id: String,
        pipeline_id: String,
        pipeline_stage: String,
    ) -> PipelineCouplingCreate {
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
pub struct PipelineCouplingCreateParams {
    /// unique identifier or name of app
    pub app: String,
    /// unique identifier of pipeline
    pub pipeline: String,
    /// target pipeline stage. one of:"test" or "review" or "development" or "staging" or "production"
    pub stage: String,
}

impl HerokuEndpoint<PipelineCoupling, (), PipelineCouplingCreateParams> for PipelineCouplingCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("pipeline-couplings")
    }
    fn body(&self) -> Option<PipelineCouplingCreateParams> {
        Some(self.params.clone())
    }
}

///
///
/// Pipeline Promotion Create
///
/// Create a new promotion.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-promotion-create)
pub struct PipelinePromotionCreate {
    /// The parameters to pass to the Heroku API
    pub params: PipelinePromotionCreateParams,
}

impl PipelinePromotionCreate {
    pub fn new(
        pipeline_id: String,
        source_app_id: String,
        target_app_id: String,
    ) -> PipelinePromotionCreate {
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
pub struct PipelinePromotionCreateParams {
    pub pipeline: PipelineParam,
    pub source: SourceParam,
    pub targets: Vec<TargetParam>,
}

impl PipelinePromotionCreateParams {
    pub fn new(
        pipeline_id: String,
        source_app_id: String,
        target_app_id: String,
    ) -> PipelinePromotionCreateParams {
        Self {
            pipeline: PipelineParam { id: pipeline_id },
            source: SourceParam {
                app: AppParam { id: source_app_id },
            },
            targets: vec![TargetParam {
                app: AppParam { id: target_app_id },
            }],
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct PipelineParam {
    pub id: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct SourceParam {
    pub app: AppParam,
}

#[derive(Serialize, Clone, Debug)]
pub struct AppParam {
    pub id: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct TargetParam {
    pub app: AppParam,
}

impl HerokuEndpoint<PipelinePromotion, (), PipelinePromotionCreateParams>
    for PipelinePromotionCreate
{
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("pipeline-promotions")
    }
    fn body(&self) -> Option<PipelinePromotionCreateParams> {
        Some(self.params.clone())
    }
}

/// Pipeline Transfer
///
/// A pipeline transfer is the process of changing pipeline ownership along with the contained apps.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-transfer)
pub struct PipelineTransferCreate {
    pub params: PipelineTransferCreateParams,
}

impl PipelineTransferCreate {
    pub fn new(
        pipeline_id: String,
        new_owner_id: String,
        new_owner_type: String,
    ) -> PipelineTransferCreate {
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
pub struct PipelineTransferCreateParams {
    pub pipeline: PipelineParam,
    pub new_owner: NewOwner,
}

#[derive(Serialize, Clone, Debug)]
pub struct NewOwner {
    /// unique identifier of a pipeline owner
    pub id: String,
    /// type of pipeline owner
    /// pattern: `(^team$
    #[serde(rename = "type")]
    pub type_field: String,
}

impl PipelineTransferCreateParams {
    pub fn new(
        pipeline_id: String,
        new_owner_id: String,
        new_owner_type: String,
    ) -> PipelineTransferCreateParams {
        Self {
            pipeline: PipelineParam { id: pipeline_id },
            new_owner: NewOwner {
                id: new_owner_id,
                type_field: new_owner_type,
            },
        }
    }
}
impl HerokuEndpoint<PipelineTransfer, (), PipelineTransferCreateParams> for PipelineTransferCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("pipeline-transfers")
    }
    fn body(&self) -> Option<PipelineTransferCreateParams> {
        Some(self.params.clone())
    }
}
