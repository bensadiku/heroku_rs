//Anything related to POST requests for dynos and it's properties goes here.

use super::Dyno;

use crate::framework::endpoint::{HerokuEndpoint, Method};

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