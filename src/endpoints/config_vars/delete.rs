//Anything related to DELETE requests for config vars and it's variations goes here.

use crate::framework::endpoint::{HerokuEndpoint, Method};
use std::collections::HashMap;

/// Config Vars DELETE
///
/// Delete config-vars for an app. You delete the config vars by setting the value to `None`.
///
/// There is no endpoint for this DELETE request, because it's done through a [PATCH](https://devcenter.heroku.com/articles/platform-api-reference#config-vars-update) request, by just setting the `value` to null/None. Separated into it's own file for clarity sakes.
pub struct AppConfigVarDelete<'a> {
    /// app_id is the unique app identifier.
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: HashMap<String, Option<String>>,
}

impl<'a> HerokuEndpoint<HashMap<String, Option<String>>, (), HashMap<String, Option<String>>>
    for AppConfigVarDelete<'a>
{
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("apps/{}/config-vars", self.app_id)
    }
    fn body(&self) -> Option<HashMap<String, Option<String>>> {
        Some(self.params.clone())
    }
}

/// Pipeline Config Vars DELETE
///
/// Delete config-vars for a pipeline. You delete the config vars by setting the value to `None`.
///
/// There is no endpoint for this DELETE request, because it's done through a [PATCH](https://devcenter.heroku.com/articles/platform-api-reference#pipeline-config-vars-update) request, by just setting the `value` to null/None. Separated into it's own file for clarity sakes.
pub struct PipelineConfigVarDelete<'a> {
    /// pipeline_id is the unique pipeline identifier.
    pub pipeline_id: &'a str,
    /// pipeline coupling stage
    pub stage_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: HashMap<String, Option<String>>,
}

impl<'a> HerokuEndpoint<HashMap<String, Option<String>>, (), HashMap<String, Option<String>>>
    for PipelineConfigVarDelete<'a>
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
    fn body(&self) -> Option<HashMap<String, Option<String>>> {
        Some(self.params.clone())
    }
}
