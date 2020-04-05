//Anything related to GET requests for Teams and it's variations goes here.
use super::{Team, TeamApp, TeamAppPermission, TeamFeature, TeamInvitation, TeamInvoice, TeamMember};

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

/// Team App Permission List
///
/// Lists permissions available to teams.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-app-permission-list)
pub struct TeamAppPermissionList {}

impl TeamAppPermissionList {
    pub fn new() -> TeamAppPermissionList {
        TeamAppPermissionList {}
    }
}

impl HerokuEndpoint<Vec<TeamAppPermission>> for TeamAppPermissionList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("teams/permissions")
    }
}

/// Team Feature List
///
/// List existing team features.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-feature-list)
pub struct TeamFeatureList<'a> {
    /// unique team identifier, either name or id
    pub team_id: &'a str,
}

impl<'a> TeamFeatureList<'a> {
    pub fn new(team_id: &'a str) -> TeamFeatureList {
        TeamFeatureList { team_id }
    }
}

impl<'a> HerokuEndpoint<Vec<TeamFeature>> for TeamFeatureList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("teams/{}/features", self.team_id)
    }
}

/// Team Feature Info
///
/// Info for an existing team feature.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-feature-info)
pub struct TeamFeatureDetails<'a> {
    /// unique team identifier, either name or id
    pub team_id: &'a str,
    /// unique feature identifier, either name or id
    pub feature_id: &'a str,
}

impl<'a> TeamFeatureDetails<'a> {
    pub fn new(team_id: &'a str, feature_id: &'a str) -> TeamFeatureDetails<'a> {
        TeamFeatureDetails {
            team_id,
            feature_id,
        }
    }
}

impl<'a> HerokuEndpoint<TeamFeature> for TeamFeatureDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("teams/{}/features/{}", self.team_id, self.feature_id)
    }
}

/// Team Invitation List
///
/// Get a list of a team’s invites
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-invitation-list)
pub struct TeamInvitationList<'a> {
    /// unique team identifier, either name or id
    pub team_id: &'a str,
}

impl<'a> TeamInvitationList<'a> {
    pub fn new(team_id: &'a str) -> TeamInvitationList {
        TeamInvitationList { team_id }
    }
}

impl<'a> HerokuEndpoint<Vec<TeamInvitation>> for TeamInvitationList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("teams/{}/invitations", self.team_id)
    }
}

/// Team Invitation Get
///
/// Get an invitation by its token
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-invitation-get)
pub struct TeamInvitationDetails<'a> {
    /// unique token identifier
    pub token_id: &'a str,
}

impl<'a> TeamInvitationDetails<'a> {
    pub fn new(token_id: &'a str) -> TeamInvitationDetails {
        TeamInvitationDetails { token_id }
    }
}

impl<'a> HerokuEndpoint<TeamInvitation> for TeamInvitationDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("teams/invitations/{}", self.token_id)
    }
}

/// Team Invoice List
///
/// List existing invoices.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-invitation-list)
pub struct TeamInvoiceList<'a> {
    /// unique team identifier, either name or id
    pub team_id: &'a str,
}

impl<'a> TeamInvoiceList<'a> {
    pub fn new(team_id: &'a str) -> TeamInvoiceList {
        TeamInvoiceList { team_id }
    }
}

impl<'a> HerokuEndpoint<Vec<TeamInvoice>> for TeamInvoiceList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("teams/{}/invoices", self.team_id)
    }
}

/// Team Invoice Info
///
/// Info for existing invoice.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-invoice-info)
pub struct TeamInvoiceDetails<'a> {
    /// unique token identifier
    pub team_id: &'a str,
    /// invoice number
    pub invoice_id: &'a str,
}

impl<'a> TeamInvoiceDetails<'a> {
    pub fn new(team_id: &'a str, invoice_id: &'a str) -> TeamInvoiceDetails<'a> {
        TeamInvoiceDetails {
            team_id,
            invoice_id,
        }
    }
}

impl<'a> HerokuEndpoint<TeamInvoice> for TeamInvoiceDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("teams/{}/invoices/{}", self.team_id, self.invoice_id)
    }
}

/// Team Member List
///
/// List members of the team.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-member-list)
pub struct TeamMemberList<'a> {
    /// unique team identifier, either name or id
    pub team_id: &'a str,
}

impl<'a> TeamMemberList<'a> {
    pub fn new(team_id: &'a str) -> TeamMemberList {
        TeamMemberList { team_id }
    }
}

impl<'a> HerokuEndpoint<Vec<TeamMember>> for TeamMemberList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("teams/{}/members", self.team_id)
    }
}

/// Team Member Apps List
///
/// List the apps of a team member.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-member-list-by-member)
pub struct TeamMemberAppsList<'a> {
    /// unique team identifier, either name or id
    pub team_id: &'a str,
    /// unique team member identifier, either email or id
    pub member_id: &'a str,
}

impl<'a> TeamMemberAppsList<'a> {
    pub fn new(team_id: &'a str, member_id: &'a str) -> TeamMemberAppsList<'a> {
        TeamMemberAppsList { team_id, member_id }
    }
}

impl<'a> HerokuEndpoint<Vec<TeamApp>> for TeamMemberAppsList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("teams/{}/members/{}/apps", self.team_id, self.member_id)
    }
}
