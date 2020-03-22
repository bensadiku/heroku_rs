//Anything related to DELETE requests for dynos and it's properties goes here.

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Dyno Restart
/// Restart dyno.
/// https://devcenter.heroku.com/articles/platform-api-reference#dyno-restart
pub struct DynoRestart {
    /// app_id can be the app name or the app id
    pub app_id: String,
    /// dyno_id can be the dyno name or the dyno id
    pub dyno_id: String,
}

impl HerokuEndpoint for DynoRestart {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/dynos/{}", self.app_id, self.dyno_id)
    }
}

/// Dyno Restart all
/// Restart all dynos.
/// https://devcenter.heroku.com/articles/platform-api-reference#dyno-restart-all
pub struct DynoAllRestart {
    /// app_id can be the app name or the app id
    pub app_id: String,
}

impl HerokuEndpoint for DynoAllRestart {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/dynos", self.app_id)
    }
}
