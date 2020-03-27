//Anything related to POST requests for dynos and it's properties goes here.

use super::Dyno;

use crate::framework::endpoint::{HerokuEndpoint, Method};
use std::collections::HashMap;

/// Dyno Stop
///
/// Stop dyno.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference##dyno-stop)
pub struct DynoActionStop {
    /// app_id can be the app name or the app id
    pub app_id: String,
    /// dyno_id can be the dyno name or the dyno id
    pub dyno_id: String,
}

impl HerokuEndpoint<Dyno> for DynoActionStop {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/dynos/{}/actions/stop", self.app_id, self.dyno_id)
    }
}
/// Dyno Create
///
/// Create a new dyno associated with an application
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#dyno-create)
#[derive(Serialize)]
pub struct DynoCreate {
    /// app_id can be the app name or the app id
    pub app_id: String,
    /// The parameters to pass to the Heroku API
    pub params: DynoCreateParams,
}

/// Create a new dyno with parameters.
///
/// Command parameter is required
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#dyno-create-required-parameters)
///
/// All other paramemters are optional.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-create-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct DynoCreateParams {
    /// command used to start process
    pub command: String,
    /// whether to stream output or not
    pub attach: Option<bool>,
    /// custom environment to add to the dyno config vars
    pub env: Option<HashMap<String, String>>,
    /// force an attached on-off dyno to not run in a tty
    pub force_no_tty: Option<bool>,
    /// dyno size
    pub size: Option<String>,
    /// seconds until dyno expires
    pub time_to_live: Option<i32>,
    /// type of process
    pub r#type: Option<String>,
}

impl HerokuEndpoint<Dyno, (), DynoCreateParams> for DynoCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/dynos", self.app_id)
    }
    fn body(&self) -> Option<DynoCreateParams> {
        Some(self.params.clone())
    }
}
