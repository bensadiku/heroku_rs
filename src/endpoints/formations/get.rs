//Anything related to GET requests for formations and it's properties goes here.

use super::Formation;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Formation Info
///
/// Get info for a process type
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#dyno-info)
pub struct FormationDetails<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
    /// formation_id can &'a str the formation id or type
    pub formation_id: &'a str,
}

impl<'a> FormationDetails <'a>{
    pub fn new(app_id: &'a str, formation_id: &'a str) -> FormationDetails <'a>{
        FormationDetails {
            app_id,
            formation_id,
        }
    }
}

impl <'a>HerokuEndpoint<Formation> for FormationDetails <'a>{
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
pub struct FormationList <'a>{
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
}

impl<'a> FormationList <'a>{
    pub fn new(app_id: &'a str) -> FormationList<'a> {
        FormationList { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<Formation>> for FormationList <'a>{
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/formation", self.app_id)
    }
}
