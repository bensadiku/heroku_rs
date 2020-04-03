//Anything related to PATCH requests for Teams and it's variations goes here.
use super::Teams;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Team Update
///
/// Update team properties.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-update)
pub struct TeamUpdate {
    /// team_id is the unique team identifier.
    pub team_id: String,
    /// The parameters to pass to the Heroku API
    pub params_optional: Option<TeamUpdateParams>,
}

/// Update team properties.
///
/// [See Heroku documentation for more information about these optional parameters](https://devcenter.heroku.com/articles/platform-api-reference#team-update-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct TeamUpdateParams {
    /// whether to use this team when none is specified
    pub default: bool,
    /// unique name of team
    pub name: String,
}

impl HerokuEndpoint<Teams, (), TeamUpdateParams> for TeamUpdate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("teams/{}", self.team_id)
    }
    fn body(&self) -> Option<TeamUpdateParams> {
        self.params_optional.clone()
    }
}
