//Anything related to DELETE requests for dynos and it's properties goes here.

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Dyno Restart
///
/// Restart dyno.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#dyno-restart)
pub struct DynoRestart<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
    /// dyno_id can be the dyno name or the dyno id
    pub dyno_id: &'a str,
}

impl<'a> DynoRestart<'a> {
    pub fn new(app_id: &'a str, dyno_id: &'a str) -> DynoRestart<'a> {
        DynoRestart { app_id, dyno_id }
    }
}

impl<'a> HerokuEndpoint for DynoRestart<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/dynos/{}", self.app_id, self.dyno_id)
    }
}

/// Dyno Restart all
///
/// Restart all dynos.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#dyno-restart-all)
pub struct DynoAllRestart<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
}

impl<'a> DynoAllRestart<'a> {
    pub fn new(app_id: &'a str) -> DynoAllRestart<'a> {
        DynoAllRestart { app_id }
    }
}

impl<'a> HerokuEndpoint for DynoAllRestart<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/dynos", self.app_id)
    }
}
