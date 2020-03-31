use crate::framework::response::ApiResult;
use std::collections::HashMap;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

/// Config Vars
///
/// Stability: production
///
/// Config Vars allow you to manage the configuration information provided to an app on Heroku.
///
/// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#config-vars)
pub use get::{AppConfigVarDetails, PipelineConfigVarDetails};
pub use patch::{AppConfigVarUpdate, PipelineConfigVarUpdate};
pub use delete::{AppConfigVarDelete, PipelineConfigVarDelete};

impl ApiResult for HashMap<String, Option<String>> {}
impl ApiResult for Vec<HashMap<String, Option<String>>> {}

impl ApiResult for HashMap<String, String> {}
impl ApiResult for Vec<HashMap<String, String>> {}
