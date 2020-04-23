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

impl DynoActionStop {
    pub fn new(app_id: String, dyno_id: String) -> DynoActionStop {
        DynoActionStop { app_id, dyno_id }
    }
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

impl DynoCreate {
    pub fn new(
        app_id: String,
        command: String,
        attach: Option<bool>,
        env: Option<HashMap<String, String>>,
        force_no_tty: Option<bool>,
        size: Option<String>,
        time_to_live: Option<i32>,
        process_type: Option<String>,
    ) -> DynoCreate {
        DynoCreate {
            app_id,
            params: DynoCreateParams {
                command: command,
                attach: attach,
                env: env,
                force_no_tty: force_no_tty,
                size: size,
                time_to_live: time_to_live,
                r#type: process_type,
            },
        }
    }
    pub fn create(app_id: String, command: String) -> DynoCreate {
        DynoCreate {
            app_id,
            params: DynoCreateParams {
                command: command,
                attach: None,
                env: None,
                force_no_tty: None,
                size: None,
                time_to_live: None,
                r#type: None,
            },
        }
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<bool>,
    /// custom environment to add to the dyno config vars
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    /// force an attached on-off dyno to not run in a tty [Nullable]
    pub force_no_tty: Option<bool>,
    /// dyno size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// seconds until dyno expires
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<i32>,
    /// type of process
    #[serde(skip_serializing_if = "Option::is_none")]
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
