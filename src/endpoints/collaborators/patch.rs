//Anything related to PATCH requests for collaborators and it's properties goes here.
use super::TeamCollaborator;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Team App Collaborator Update
///
/// Update an existing collaborator from a team app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-app-collaborator-update)
pub struct TeamCollaboratorUpdate<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
    /// collaborator_id can be the collaborator email or id.
    pub collaborator_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: TeamCollaboratorUpdateParams<'a>,
}

impl<'a> TeamCollaboratorUpdate<'a> {
    pub fn new(
        app_id: &'a str,
        collaborator_id: &'a str,
        permissions: Vec<&'a str>,
    ) -> TeamCollaboratorUpdate<'a> {
        TeamCollaboratorUpdate {
            app_id,
            collaborator_id,
            params: TeamCollaboratorUpdateParams { permissions },
        }
    }
}

/// Update team app collaborator with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-app-collaborator-update-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct TeamCollaboratorUpdateParams<'a> {
    /// An array of permissions to give to the collaborator.
    pub permissions: Vec<&'a str>,
}

impl<'a> HerokuEndpoint<TeamCollaborator, (), TeamCollaboratorUpdateParams<'a>>
    for TeamCollaboratorUpdate<'a>
{
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("teams/apps/{}/collaborators", self.app_id)
    }
    fn body(&self) -> Option<TeamCollaboratorUpdateParams<'a>> {
        Some(self.params.clone())
    }
}
