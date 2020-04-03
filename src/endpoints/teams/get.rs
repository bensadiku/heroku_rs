//Anything related to GET requests for Teams and it's variations goes here.
use super::Teams;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Team Info
///
/// Info for a team.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-info)
pub struct TeamDetails {
    /// unique team identifier.
    pub team_id: String,
}

impl HerokuEndpoint<Teams> for TeamDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("teams/{}", self.team_id)
    }
}

/// Team List
///
/// List teams in which you are a member.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-list)
pub struct TeamList {}

impl HerokuEndpoint<Teams> for TeamList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("teams")
    }
}
