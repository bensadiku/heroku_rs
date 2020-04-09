//Anything related to GET requests for config vars and it's variations goes here.
use std::collections::HashMap;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Config Vars Info for App
///
/// Get config-vars for app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#config-vars-info-for-app)
pub struct AppConfigVarDetails<'a> {
    /// unique app identifier.
    pub app_id: &'a str,
}

impl <'a>AppConfigVarDetails <'a> {
    pub fn new(app_id: &'a str) -> AppConfigVarDetails {
        AppConfigVarDetails { app_id}
    }
}

impl<'a> HerokuEndpoint<HashMap<String, Option<String>>> for AppConfigVarDetails<'a> {
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
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#config-vars-info-for-app-release)
pub struct ReleaseConfigVarDetails<'a> {
    /// unique app identifier.
    pub app_id: &'a str,
    // unique release identifier, release id or release version
    pub release_id: &'a str,
}

impl <'a>ReleaseConfigVarDetails <'a> {
    pub fn new(app_id: &'a str, release_id: &'a str) -> ReleaseConfigVarDetails<'a> {
        ReleaseConfigVarDetails { app_id, release_id}
    }
}

impl<'a> HerokuEndpoint<HashMap<String, Option<String>>> for ReleaseConfigVarDetails<'a> {
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
pub struct PipelineConfigVarDetails<'a> {
    /// unique pipeline identifier.
    pub pipeline_id: &'a str,
    /// pipeline stage
    pub stage_id: &'a str,
}


impl <'a>PipelineConfigVarDetails <'a> {
    pub fn new(pipeline_id: &'a str, stage_id: &'a str) -> PipelineConfigVarDetails<'a> {
        PipelineConfigVarDetails { pipeline_id, stage_id}
    }
}

impl<'a> HerokuEndpoint<HashMap<String, Option<String>>> for PipelineConfigVarDetails<'a> {
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
