//Anything related to GET requests for config vars and it's variations goes here.
use std::collections::HashMap;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Config Vars Info for App
///
/// Get config-vars for app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#config-vars-info-for-app)
pub struct AppConfigVarDetails {
    /// unique app identifier.
    pub app_id: String,
}

impl HerokuEndpoint<HashMap<String, Option<String>>> for AppConfigVarDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/config-vars", self.app_id)
    }
}

/// Config Vars Info for App Release
///
/// Get config-vars for an app release.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#config-vars-info-for-app)
pub struct ReleaseConfigVarDetails {
    /// unique app identifier.
    pub app_id: String,
    // unique release identifier, release id or release version
    pub release_id: String,
}

impl HerokuEndpoint<HashMap<String, Option<String>>> for ReleaseConfigVarDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "apps/{}/releases/{}/config-vars",
            self.app_id, self.release_id
        )
    }
}

/// Config Vars Info for Pipeline
///
/// Pipeline Config Vars allow you to manage the configuration information provided to a pipeline.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-config-vars)
pub struct PipelineConfigVarDetails {
    /// unique pipeline identifier.
    pub pipeline_id: String,
    /// pipeline stage
    pub stage_id: String,
}

impl HerokuEndpoint<HashMap<String, Option<String>>> for PipelineConfigVarDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "pipelines/{}/stage/{}/config-vars",
            self.pipeline_id, self.stage_id
        )
    }
}
