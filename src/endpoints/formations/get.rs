//Anything related to GET requests for formations and it's properties goes here.

use super::Formation;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Formation Info
/// 
/// Get info for a process type
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#dyno-info)
pub struct FormationDetails {
    /// app_id can be the app name or the app id
    pub app_id: String,
    /// formation_id can be the formation id or type
    pub formation_id: String,
}

impl HerokuEndpoint<Formation> for FormationDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/formation/{}", self.app_id, self.formation_id)
    }
}

/// Formation List
///
/// List process type formation
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#formation-list)
pub struct FormationList {
    /// app_id can be the app name or the app id
    pub app_id: String,
}

impl HerokuEndpoint<Vec<Formation>> for FormationList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/formation", self.app_id)
    }
}