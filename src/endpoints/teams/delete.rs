//Anything related to GET requests for Teams and it's variations goes here.
use super::{Team, TeamInvitation, TeamMember};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Team Delete
///
/// Delete an existing team.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-delete)
///
/// # Example:
///
/// AppDelete takes one required parameter, team_id, and returns the deleted [`Team`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&TeamDelete::new("TEAM_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.Team.html
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
///
/// # Example:
///
/// TeamInvitationRevoke takes two required parameters, team_id and invitation_id, and returns the revoked [`TeamInvitation`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&TeamInvitationRevoke::new("TEAM_ID", "INVITATION_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.TeamInvitation.html
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
///
/// # Example:
///
/// TeamMemberDelete takes two required parameters, team_id and member_id, and returns the deleted [`TeamMember`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&TeamMemberDelete::new("TEAM_ID", "MEMBER_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
//
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.TeamMember.html
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
