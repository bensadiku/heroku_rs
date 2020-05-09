//Anything related to POST requests for dynos and it's properties goes here.

use super::Dyno;

use crate::framework::endpoint::{HerokuEndpoint, Method};
use std::collections::HashMap;

/// Dyno Stop
///
/// Stop dyno.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference##dyno-stop)
pub struct DynoActionStop<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
    /// dyno_id can be the dyno name or the dyno id
    pub dyno_id: &'a str,
}

impl<'a> DynoActionStop<'a> {
    pub fn new(app_id: &'a str, dyno_id: &'a str) -> DynoActionStop<'a> {
        DynoActionStop { app_id, dyno_id }
    }
}

impl<'a> HerokuEndpoint<Dyno> for DynoActionStop<'a> {
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
pub struct DynoCreate<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: DynoCreateParams<'a>,
}

impl<'a> DynoCreate<'a> {
    pub fn new(app_id: &'a str, command: &'a str) -> DynoCreate<'a> {
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

    pub fn attach(&mut self, attach: bool) -> &mut Self {
        self.params.attach = Some(attach);
        self
    }

    pub fn env(&mut self, env: HashMap<&'a str, &'a str>) -> &mut Self {
        self.params.env = Some(env);
        self
    }

    pub fn force_no_tty(&mut self, force_no_tty: bool) -> &mut Self {
        self.params.force_no_tty = Some(force_no_tty);
        self
    }

    pub fn size(&mut self, size: &'a str) -> &mut Self {
        self.params.size = Some(size);
        self
    }

    pub fn time_to_live(&mut self, time_to_live: i32) -> &mut Self {
        self.params.time_to_live = Some(time_to_live);
        self
    }

    pub fn dyno_type(&mut self, dyno_type: &'a str) -> &mut Self {
        self.params.r#type = Some(dyno_type);
        self
    }

    pub fn build(&self) -> DynoCreate<'a> {
        DynoCreate {
            app_id: self.app_id,
            params: DynoCreateParams {
                command: self.params.command,
                attach: self.params.attach,
                env: self.params.env.clone(),
                force_no_tty: self.params.force_no_tty,
                size: self.params.size,
                time_to_live: self.params.time_to_live,
                r#type: self.params.r#type,
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
pub struct DynoCreateParams<'a> {
    /// command used to start process
    pub command: &'a str,
    /// whether to stream output or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<bool>,
    /// custom environment to add to the dyno config vars
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<&'a str, &'a str>>,
    /// force an attached on-off dyno to not run in a tty [Nullable]
    pub force_no_tty: Option<bool>,
    /// dyno size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<&'a str>,
    /// seconds until dyno expires
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<i32>,
    /// type of process
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<&'a str>,
}

impl<'a> HerokuEndpoint<Dyno, (), DynoCreateParams<'a>> for DynoCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/dynos", self.app_id)
    }
    fn body(&self) -> Option<DynoCreateParams<'a>> {
        Some(self.params.clone())
    }
}
