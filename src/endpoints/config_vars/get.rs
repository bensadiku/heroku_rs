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
