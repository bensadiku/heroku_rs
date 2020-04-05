//Anything related to PUT requests for Teams and it's variations goes here.
use super::TeamInvitation;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Team Invitation Create
///
/// Create Team Invitation
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-invitation-create)
pub struct TeamInvitationCreate<'a> {
    /// unique team identifier
    pub team_id: &'a str,
    /// parameters to pass to Heroku
    pub params: TeamInvitationCreateParams<'a>,
}

impl<'a> TeamInvitationCreate<'a> {
    pub fn new(
        team_id: &'a str,
        email: &'a str,
        role: Option<&'a str>,
    ) -> TeamInvitationCreate<'a> {
        TeamInvitationCreate {
            team_id,
            params: TeamInvitationCreateParams { email, role },
        }
    }
}

/// Create a new team invitation with parameters
///
/// role parameter is nullable, meaning, if you pass (None), it will be sent as `null` to the Heroku API
///
/// [See Heroku documentation for more information about these paramters](https://devcenter.heroku.com/articles/platform-api-reference#team-invitation-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct TeamInvitationCreateParams<'a> {
    /// unique email address
    pub email: &'a str,
    /// Even though marked with `Option`, this parameter is NOT optional.
    /// role in the team
    /// one of:"admin" or "collaborator" or "member" or "owner" or null
    pub role: Option<&'a str>,
}

impl<'a> HerokuEndpoint<TeamInvitation, (), TeamInvitationCreateParams<'a>>
    for TeamInvitationCreate<'a>
{
    fn method(&self) -> Method {
        Method::Put
    }
    fn path(&self) -> String {
        format!("teams/{}/invitations", self.team_id)
    }
    fn body(&self) -> Option<TeamInvitationCreateParams<'a>> {
        Some(self.params.clone())
    }
}
