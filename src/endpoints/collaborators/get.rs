//Anything related to GET requests for collaborators and it's properties goes here.
use super::{Collaborator, TeamCollaborator};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Collaborator List
///
/// List existing collaborators.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#collaborator-list)
pub struct CollaboratorList {
    /// app_id can be the app name or id.
    pub app_id: String,
}

impl CollaboratorList {
    pub fn new(app_id: String) -> CollaboratorList {
        CollaboratorList { app_id }
    }
}

impl HerokuEndpoint<Vec<Collaborator>> for CollaboratorList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/collaborators", self.app_id)
    }
}

/// Collaborator Info
///
/// Info for existing collaborator.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#collaborator-info)
///
pub struct CollaboratorDetails {
    /// app_id can be the app name or id.
    pub app_id: String,
    /// collaborator_id can be the collaborator email or id.
    pub collaborator_id: String,
}

impl CollaboratorDetails {
    pub fn new(app_id: String, collaborator_id: String) -> CollaboratorDetails {
        CollaboratorDetails {
            app_id,
            collaborator_id,
        }
    }
}

impl HerokuEndpoint<Collaborator> for CollaboratorDetails {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "apps/{}/collaborators/{}",
            self.app_id, self.collaborator_id
        )
    }
}

/// Team App Collaborator List
///
/// List collaborators on a team app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-app-collaborator-list)
pub struct TeamCollaboratorList<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
}
impl<'a> TeamCollaboratorList<'a> {
    pub fn new(app_id: &'a str) -> TeamCollaboratorList<'a> {
        TeamCollaboratorList { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<TeamCollaborator>> for TeamCollaboratorList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("teams/apps/{}/collaborators", self.app_id)
    }
}

/// Team App Collaborator Info
///
/// Info for a collaborator on a team app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-app-collaborator-info)
pub struct TeamCollaboratorDetails<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// collaborator_id can be the collaborator email or id.
    pub collaborator_id: &'a str,
}

impl<'a> TeamCollaboratorDetails<'a> {
    pub fn new(app_id: &'a str, collaborator_id: &'a str) -> TeamCollaboratorDetails<'a> {
        TeamCollaboratorDetails {
            app_id,
            collaborator_id,
        }
    }
}

impl<'a> HerokuEndpoint<TeamCollaborator> for TeamCollaboratorDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "teams/apps/{}/collaborators/{}",
            self.app_id, self.collaborator_id
        )
    }
}
