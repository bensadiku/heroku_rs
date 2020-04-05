//Anything related to DELETE requests for collaborators and it's properties goes here.
use super::{Collaborator, TeamCollaborator};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Collaborator Delete
///
/// Delete an existing collaborator.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#collaborator-delete)
pub struct CollaboratorDelete {
    /// app_id can be the app name or id.
    pub app_id: String,
    /// collaborator_id can be the collaborator email or id.
    pub collaborator_id: String,
}

impl HerokuEndpoint<Collaborator> for CollaboratorDelete {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!(
            "apps/{}/collaborators/{}",
            self.app_id, self.collaborator_id
        )
    }
}

/// Team App Collaborator Delete
///
/// Delete an existing collaborator from a team app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-app-collaborator-delete)
pub struct TeamCollaboratorDelete<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// collaborator_id can be the collaborator email or id.
    pub collaborator_id: &'a str,
}

impl<'a> TeamCollaboratorDelete<'a> {
    pub fn new(app_id: &'a str, collaborator_id: &'a str) -> TeamCollaboratorDelete<'a> {
        TeamCollaboratorDelete {
            app_id,
            collaborator_id,
        }
    }
}

impl<'a> HerokuEndpoint<TeamCollaborator> for TeamCollaboratorDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!(
            "teams/apps/{}/collaborators/{}",
            self.app_id, self.collaborator_id
        )
    }
}
