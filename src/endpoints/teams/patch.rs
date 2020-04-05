//Anything related to PATCH requests for Teams and it's variations goes here.
use super::{Team, TeamApp};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Team Update
///
/// Update team properties.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-update)
pub struct TeamUpdate<'a> {
    /// team_id is the unique team identifier.
    pub team_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params_optional: Option<TeamUpdateParams<'a>>,
}

impl<'a> TeamUpdate<'a> {
    // cares for optional parameters
    pub fn new(team_id: &'a str, default: Option<bool>, name: Option<&'a str>) -> TeamUpdate<'a> {
        TeamUpdate {
            team_id,
            params_optional: Some(TeamUpdateParams {
                default: default,
                name: name,
            }),
        }
    }

    // only required parameters to be passed
    pub fn create(team_id: &'a str) -> TeamUpdate<'a> {
        TeamUpdate {
            team_id,
            params_optional: None,
        }
    }
}

/// Update team properties.
///
/// [See Heroku documentation for more information about these optional parameters](https://devcenter.heroku.com/articles/platform-api-reference#team-update-optional-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct TeamUpdateParams<'a> {
    /// whether to use this team when none is specified
    pub default: Option<bool>,
    /// unique name of team
    pub name: Option<&'a str>,
}

impl<'a> HerokuEndpoint<Team, (), TeamUpdateParams<'a>> for TeamUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("teams/{}", self.team_id)
    }
    fn body(&self) -> Option<TeamUpdateParams<'a>> {
        self.params_optional.clone()
    }
}

/// Team App Update Locked
///
/// Lock or unlock a team app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-app-update-locked)
pub struct TeamAppUpdateLocked<'a> {
    /// team_id is the unique team identifier.
    pub team_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: TeamAppUpdateLockedParams,
}

impl<'a> TeamAppUpdateLocked<'a> {
    pub fn new(team_id: &'a str, locked: bool) -> TeamAppUpdateLocked<'a> {
        TeamAppUpdateLocked {
            team_id,
            params: TeamAppUpdateLockedParams { locked },
        }
    }
}

/// Update team app properties.
///
/// [See Heroku documentation for more information about these required parameters](https://devcenter.heroku.com/articles/platform-api-reference#team-app-update-locked-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct TeamAppUpdateLockedParams {
    /// are other team members forbidden from joining this app.
    pub locked: bool,
}

impl<'a> HerokuEndpoint<Team, (), TeamAppUpdateLockedParams> for TeamAppUpdateLocked<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("teams/apps/{}", self.team_id)
    }
    fn body(&self) -> Option<TeamAppUpdateLockedParams> {
        Some(self.params.clone())
    }
}

/// Team App Transfer to Account or Team
///
/// Transfer an existing team app to another Heroku account or another Heroku Team.
///
/// [See Heroku documentation for more information about the account transfer](https://devcenter.heroku.com/articles/platform-api-reference#team-app-transfer-to-account)
///
/// [See Heroku documentation for more information about the team transfer](https://devcenter.heroku.com/articles/platform-api-reference#team-app-transfer-to-team)
pub struct TeamAppTransfer<'a> {
    /// team_id is the unique team identifier.
    pub team_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: TeamAppTransferParams<'a>,
}

impl<'a> TeamAppTransfer<'a> {
    pub fn new(team_id: &'a str, owner_id: &'a str) -> TeamAppTransfer<'a> {
        TeamAppTransfer {
            team_id: team_id,
            params: TeamAppTransferParams { owner: owner_id },
        }
    }
}

/// Transfer a team app to another account or team
///
/// Note: The distinction between transferring an app to an account is the `owner` field.
/// If you pass an email adress or account identifier it will transfer the app to a account.
/// If you pass the unique name of a team, it will transfer the app to a team.
///
/// [See Heroku documentation for account transferring](https://devcenter.heroku.com/articles/platform-api-reference#team-app-transfer-to-account-required-parameters)
///
/// [See Heroku documentation for team transferring](https://devcenter.heroku.com/articles/platform-api-reference#team-app-transfer-to-team-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct TeamAppTransferParams<'a> {
    /// unique email address, identifier of an account or implicit reference to currently authorized user
    /// or unique name of team
    pub owner: &'a str,
}

impl<'a> HerokuEndpoint<TeamApp, (), TeamAppTransferParams<'a>> for TeamAppTransfer<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("teams/apps/{}", self.team_id)
    }
    fn body(&self) -> Option<TeamAppTransferParams<'a>> {
        Some(self.params.clone())
    }
}
