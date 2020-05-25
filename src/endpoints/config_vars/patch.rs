//Anything related to PATCH requests for config vars and it's variations goes here.

use crate::framework::endpoint::{HerokuEndpoint, Method};
use std::collections::HashMap;

/// Config Vars Update
///
/// Update config-vars for app. You can update existing config-vars by setting them again.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#config-vars-update)
///
/// # Example:
///
/// AppConfigVarUpdate takes two required parameters, app_id and params, and returns a `HashMap<String, String>`.
/// ```rust
/// use heroku_rs::prelude::*;
/// use std::collections::HashMap;
///
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let mut cvar = HashMap::new();
/// let cvar_key = String::from("config_var_key"); // config var value key
/// let cvar_value = String::from("updated_config_var_value"); // config var value to update
///
/// cvar.insert(cvar_key, cvar_value);
///
/// let response = api_client.request(&AppConfigVarUpdate::new("APP_ID", cvar));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
pub struct AppConfigVarUpdate<'a> {
    /// app_id is the unique app identifier.
    pub app_id: &'a str,
    /**
     * If you're coming from the Heroku docs, you'll notice that DELETE is implemented by setting the `value` of the config var to null and sending it as a PATCH request.
     * I didn't want to do both PATCH and DELETE on the same `AppConfigVarUpdate` struct. The delete request is moved to it's own file `AppConfigVarDelete`.
     *
     */
    /// The parameters to pass to the Heroku API
    pub params: HashMap<String, String>,
}

#[cfg(feature = "builder")]
impl<'a> AppConfigVarUpdate<'a> {
    pub fn new(app_id: &'a str, params: HashMap<String, String>) -> AppConfigVarUpdate {
        AppConfigVarUpdate { app_id, params }
    }
}

impl<'a> HerokuEndpoint<HashMap<String, String>, (), HashMap<String, String>>
    for AppConfigVarUpdate<'a>
{
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
///
/// # Example:
///
/// PipelineConfigVarUpdate takes three required parameters, app_id and stage_id and the config vars keys as a HashMap object, and returns a `HashMap<String, String>`.
/// ```rust
/// use heroku_rs::prelude::*;
/// use std::collections::HashMap;
///
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let mut cvar = HashMap::new();
/// let cvar_key = String::from("config_var_key"); // config var value key
/// let cvar_value = String::from("updated_config_var_value"); // config var value to update
///
/// cvar.insert(cvar_key, cvar_value);
///
/// let response = api_client.request(&PipelineConfigVarUpdate::new(
///     "PIPELINE_ID",
///     "STAGE_ID",
///     cvar,
/// ));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
pub struct PipelineConfigVarUpdate<'a> {
    /// pipeline_id is the unique pipeline identifier.
    pub pipeline_id: &'a str,
    /// pipeline coupling stage
    pub stage_id: &'a str,
    /**
     * If you're coming from the Heroku docs, you'll notice that DELETE is implemented by setting the `value` of the config var to null and sending it as a PATCH request.
     * I didn't want to do both PATCH and DELETE on the same `PipelineConfigVarUpdate` struct. The delete request is moved to it's own file `PipelineConfigVarDelete`.
     *
     */
    /// The parameters to pass to the Heroku API
    pub params: HashMap<String, String>,
}

#[cfg(feature = "builder")]
impl<'a> PipelineConfigVarUpdate<'a> {
    pub fn new(
        pipeline_id: &'a str,
        stage_id: &'a str,
        params: HashMap<String, String>,
    ) -> PipelineConfigVarUpdate<'a> {
        PipelineConfigVarUpdate {
            pipeline_id,
            stage_id,
            params,
        }
    }
}

impl<'a> HerokuEndpoint<HashMap<String, String>, (), HashMap<String, String>>
    for PipelineConfigVarUpdate<'a>
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
