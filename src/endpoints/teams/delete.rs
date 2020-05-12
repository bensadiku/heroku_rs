//Anything related to GET requests for Teams and it's variations goes here.
use super::{Team, TeamInvitation, TeamMember};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Team Delete
///
/// Delete an existing team.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-delete)
pub struct TeamDelete<'a> {
    /// unique team identifier.
    pub team_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> TeamDelete<'a> {
    pub fn new(team_id: &'a str) -> TeamDelete {
        TeamDelete { team_id }
    }
}

impl<'a> HerokuEndpoint<Team> for TeamDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("teams/{}", self.team_id)
    }
}

/// Team Invitation Revoke
///
/// Revoke a team invitation.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-invitation-revoke)
pub struct TeamInvitationRevoke<'a> {
    /// unique team identifier.
    pub team_id: &'a str,
    /// unique invitation identifier
    pub invitation_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> TeamInvitationRevoke<'a> {
    pub fn new(team_id: &'a str, invitation_id: &'a str) -> TeamInvitationRevoke<'a> {
        TeamInvitationRevoke {
            team_id,
            invitation_id,
        }
    }
}

impl<'a> HerokuEndpoint<TeamInvitation> for TeamInvitationRevoke<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("teams/{}/invitations/{}", self.team_id, self.invitation_id)
    }
}

/// Team Member Delete
///
/// Remove a member from the team.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-member-delete)
pub struct TeamMemberDelete<'a> {
    /// unique team identifier.
    pub team_id: &'a str,
    /// unique member identifier
    pub member_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> TeamMemberDelete<'a> {
    pub fn new(team_id: &'a str, member_id: &'a str) -> TeamMemberDelete<'a> {
        TeamMemberDelete { team_id, member_id }
    }
}

impl<'a> HerokuEndpoint<TeamMember> for TeamMemberDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("teams/{}/members/{}", self.team_id, self.member_id)
    }
}
