//Anything related to GET requests for Teams and it's variations goes here.
use super::{Team, TeamApp};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Team Info
///
/// Info for a team.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-info)
pub struct TeamDetails<'a> {
    /// unique team identifier.
    pub team_id: &'a str,
}

impl<'a> TeamDetails<'a> {
    pub fn new(team_id: &'a str) -> TeamDetails {
        TeamDetails { team_id }
    }
}

impl<'a> HerokuEndpoint<Team> for TeamDetails<'a> {
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

impl TeamList {
    pub fn new() -> TeamList {
        TeamList {}
    }
}

impl HerokuEndpoint<Vec<Team>> for TeamList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("teams")
    }
}

/// Team List by Enterprise Account
///
/// List teams for an enterprise account.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-list-by-enterprise-account)
pub struct TeamListByEA<'a> {
    pub account_id: &'a str,
}

impl<'a> TeamListByEA<'a> {
    pub fn new(account_id: &'a str) -> TeamListByEA {
        TeamListByEA { account_id }
    }
}

impl<'a> HerokuEndpoint<Vec<Team>> for TeamListByEA<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("enterprise-accounts/{}/teams", self.account_id)
    }
}

/// Team App Info
///
/// Info for a team app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-app-info)
pub struct TeamAppDetails<'a> {
    pub app_id: &'a str,
}

impl<'a> TeamAppDetails<'a> {
    pub fn new(app_id: &'a str) -> TeamAppDetails {
        TeamAppDetails { app_id }
    }
}

impl<'a> HerokuEndpoint<TeamApp> for TeamAppDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("teams/apps/{}", self.app_id)
    }
}

/// Team App List By Team
///
/// List team apps.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-app-list-by-team)
pub struct TeamAppList<'a> {
    pub team_id: &'a str,
}

impl<'a> TeamAppList<'a> {
    pub fn new(team_id: &'a str) -> TeamAppList {
        TeamAppList { team_id }
    }
}

impl<'a> HerokuEndpoint<Vec<TeamApp>> for TeamAppList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("teams/apps/{}", self.team_id)
    }
}
