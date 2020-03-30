//Anything related to DELETE requests for config vars and it's variations goes here.

use crate::framework::endpoint::{HerokuEndpoint, Method};
use std::collections::HashMap;

/// Config Vars DELETE
///
/// Delete config-vars for an app. You delete the config vars by setting the value to `None`.
///
/// There is no endpoint for this DELETE request, because it's done through a [PATCH](https://devcenter.heroku.com/articles/platform-api-reference#config-vars-update) request, by just setting the `value` to null/None. Separated into it's own file for clarity sakes.
pub struct AppConfigVarDelete {
    /// app_id is the unique app identifier.
    pub app_id: String,
    /// The parameters to pass to the Heroku API
    pub params: HashMap<String, Option<String>>,
}

impl HerokuEndpoint<HashMap<String, Option<String>>, (), HashMap<String, Option<String>>>
    for AppConfigVarDelete
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
