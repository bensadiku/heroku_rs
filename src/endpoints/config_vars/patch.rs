//Anything related to PATCH requests for config vars and it's variations goes here.

use crate::framework::endpoint::{HerokuEndpoint, Method};
use std::collections::HashMap;

/// Config Vars Update
///
/// Update config-vars for app. You can update existing config-vars by setting them again.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#config-vars-update)
pub struct AppConfigVarUpdate {
    /// app_id is the unique app identifier.
    pub app_id: String,
    /**
     * If you're coming from the Heroku docs, you'll notice that DELETE is implemented by setting the `value` of the config var to null and sending it as a PATCH request.
     * I didn't want to do both PATCH and DELETE on the same `AppConfigVarUpdate` struct. The delete request is moved to it's own file `AppConfigVarDelete`.
     *
     */
    /// The parameters to pass to the Heroku API
    pub params: HashMap<String, String>,
}

impl HerokuEndpoint<HashMap<String, String>, (), HashMap<String, String>> for AppConfigVarUpdate {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("apps/{}/config-vars", self.app_id)
    }
    fn body(&self) -> Option<HashMap<String, String>> {
        Some(self.params.clone())
    }
}

/// Pipeline Config Vars Update
///
/// Update config-vars for a pipeline stage. You can update existing config-vars by setting them again.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-config-vars-update)
pub struct PipelineConfigVarUpdate {
    /// pipeline_id is the unique pipeline identifier.
    pub pipeline_id: String,
    /// pipeline coupling stage
    pub stage_id: String,
    /**
     * If you're coming from the Heroku docs, you'll notice that DELETE is implemented by setting the `value` of the config var to null and sending it as a PATCH request.
     * I didn't want to do both PATCH and DELETE on the same `PipelineConfigVarDelete` struct. The delete request is moved to it's own file `PipelineConfigVarDelete`.
     *
     */
    /// The parameters to pass to the Heroku API
    pub params: HashMap<String, String>,
}

impl HerokuEndpoint<HashMap<String, String>, (), HashMap<String, String>>
    for PipelineConfigVarUpdate
{
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!(
            "pipelines/{}/stage/{}/config-vars",
            self.pipeline_id, self.stage_id
        )
    }
    fn body(&self) -> Option<HashMap<String, String>> {
        Some(self.params.clone())
    }
}
