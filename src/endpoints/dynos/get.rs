//Anything related to GET requests for dynos and it's properties goes here.

use super::Dyno;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Dyno Info
///
/// Get info for existing dyno.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#dyno-info)
pub struct DynoDetails {
    /// app_id can be the app name or the app id
    pub app_id: String,
    /// dyno_id can be the dyno name or the dyno id
    pub dyno_id: String,
}

impl DynoDetails {
    pub fn new(app_id: String, dyno_id: String) -> DynoDetails {
        DynoDetails { app_id, dyno_id }
    }
}

impl HerokuEndpoint<Dyno> for DynoDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/dynos/{}", self.app_id, self.dyno_id)
    }
}

/// Dyno List
///
/// List existing dynos.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#dyno-list)
pub struct DynoList {
    /// app_id can be the app name or the app id
    pub app_id: String,
}

impl DynoList {
    pub fn new(app_id: String) -> DynoList {
        DynoList { app_id }
    }
}

impl HerokuEndpoint<Vec<Dyno>> for DynoList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/dynos", self.app_id)
    }
}
