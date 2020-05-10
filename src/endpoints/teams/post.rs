//Anything related to POST requests for Teams and it's variations goes here.
use super::{Team, TeamApp, TeamInvitation, TeamMember};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Team Create
///
/// Create a new team.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-create)
pub struct TeamCreate<'a> {
    pub params: TeamCreateParams<'a>,
}

impl<'a> TeamCreate<'a> {
    // `new` method has only the required parameters
    pub fn new(name: &'a str) -> TeamCreate {
        TeamCreate {
            params: TeamCreateParams {
                name: name,
                address_1: None,
                address_2: None,
                card_number: None,
                city: None,
                country: None,
                cvv: None,
                expiration_month: None,
                expiration_year: None,
                first_name: None,
                last_name: None,
                other: None,
                postal_code: None,
                state: None,
                nonce: None,
                device_data: None,
            },
        }
    }

    pub fn address_1(&mut self, address_1: &'a str) -> &mut Self {
        self.params.address_1 = Some(address_1);
        self
    }

    pub fn address_2(&mut self, address_2: &'a str) -> &mut Self {
        self.params.address_2 = Some(address_2);
        self
    }

    pub fn card_number(&mut self, card_number: &'a str) -> &mut Self {
        self.params.card_number = Some(card_number);
        self
    }

    pub fn city(&mut self, city: &'a str) -> &mut Self {
        self.params.city = Some(city);
        self
    }

    pub fn country(&mut self, country: &'a str) -> &mut Self {
        self.params.country = Some(country);
        self
    }

    pub fn cvv(&mut self, cvv: &'a str) -> &mut Self {
        self.params.cvv = Some(cvv);
        self
    }

    pub fn expiration_month(&mut self, expiration_month: &'a str) -> &mut Self {
        self.params.expiration_month = Some(expiration_month);
        self
    }

    pub fn expiration_year(&mut self, expiration_year: &'a str) -> &mut Self {
        self.params.expiration_year = Some(expiration_year);
        self
    }

    pub fn first_name(&mut self, first_name: &'a str) -> &mut Self {
        self.params.first_name = Some(first_name);
        self
    }

    pub fn last_name(&mut self, last_name: &'a str) -> &mut Self {
        self.params.last_name = Some(last_name);
        self
    }

    pub fn other(&mut self, other: &'a str) -> &mut Self {
        self.params.other = Some(other);
        self
    }

    pub fn postal_code(&mut self, postal_code: &'a str) -> &mut Self {
        self.params.postal_code = Some(postal_code);
        self
    }
    pub fn state(&mut self, state: &'a str) -> &mut Self {
        self.params.state = Some(state);
        self
    }

    pub fn nonce(&mut self, nonce: &'a str) -> &mut Self {
        self.params.nonce = Some(nonce);
        self
    }

    pub fn device_data(&mut self, device_data: &'a str) -> &mut Self {
        self.params.device_data = Some(device_data);
        self
    }

    pub fn build(&self) -> TeamCreate<'a> {
        TeamCreate {
            params: TeamCreateParams {
                name: self.params.name,
                address_1: self.params.address_1,
                address_2: self.params.address_2,
                card_number: self.params.card_number,
                city: self.params.city,
                country: self.params.country,
                cvv: self.params.cvv,
                expiration_month: self.params.expiration_month,
                expiration_year: self.params.expiration_year,
                first_name: self.params.first_name,
                last_name: self.params.last_name,
                other: self.params.other,
                postal_code: self.params.postal_code,
                state: self.params.state,
                nonce: self.params.nonce,
                device_data: self.params.device_data,
            },
        }
    }
}

/// Create a new team with parameters
///
/// Only the name is required
///
/// [See Heroku documentation for more information about these paramters](https://devcenter.heroku.com/articles/platform-api-reference#team-create-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct TeamCreateParams<'a> {
    /// unique name of team
    pub name: &'a str,
    /// street address line 1
    pub address_1: Option<&'a str>,
    /// street address line 2
    pub address_2: Option<&'a str>,
    /// encrypted card number of payment method
    pub card_number: Option<&'a str>,
    /// city
    pub city: Option<&'a str>,
    /// country
    pub country: Option<&'a str>,
    /// card verification value
    pub cvv: Option<&'a str>,
    /// expiration month
    pub expiration_month: Option<&'a str>,
    /// expiration year
    pub expiration_year: Option<&'a str>,
    /// the first name for payment method
    pub first_name: Option<&'a str>,
    /// the last name for payment method
    pub last_name: Option<&'a str>,
    /// metadata
    pub other: Option<&'a str>,
    /// postal code
    pub postal_code: Option<&'a str>,
    /// state
    pub state: Option<&'a str>,
    /// Nonce generated by Braintree hosted fields form
    pub nonce: Option<&'a str>,
    /// Device data string generated by the client
    pub device_data: Option<&'a str>,
}

impl<'a> HerokuEndpoint<Team, (), TeamCreateParams<'a>> for TeamCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("teams")
    }
    fn body(&self) -> Option<TeamCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Team Create in Enterprise Account
///
/// Create a team in an enterprise account.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-create-in-enterprise-account)
pub struct TeamCreateByEA<'a> {
    /// unique account identifier
    pub account_id: &'a str,
    /// parameters to pass to Heroku
    pub params: TeamCreateByEAParams<'a>,
}

impl<'a> TeamCreateByEA<'a> {
    pub fn new(account_id: &'a str, name: &'a str) -> TeamCreateByEA<'a> {
        TeamCreateByEA {
            account_id: account_id,
            params: TeamCreateByEAParams { name },
        }
    }
}

/// Create a new team in an enterprise account with required parametesrs
///
/// [See Heroku documentation for more information about these paramters](https://devcenter.heroku.com/articles/platform-api-reference#team-create-in-enterprise-account-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct TeamCreateByEAParams<'a> {
    /// unique name of team
    pub name: &'a str,
}

impl<'a> HerokuEndpoint<Team, (), TeamCreateByEAParams<'a>> for TeamCreateByEA<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("enterprise-accounts/{}/teams", self.account_id)
    }
    fn body(&self) -> Option<TeamCreateByEAParams<'a>> {
        Some(self.params.clone())
    }
}

/// Team App Create
///
/// Create a new app in the specified team, in the default team if unspecified, or in personal account, if default team is not set.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-app-create)
pub struct TeamAppCreate<'a> {
    pub params: TeamAppCreateParams<'a>,
}

impl<'a> TeamAppCreate<'a> {
    pub fn new() -> TeamAppCreate<'a> {
        TeamAppCreate {
            params: TeamAppCreateParams {
                locked: None,
                name: None,
                team: None,
                personal: None,
                region: None,
                space: None,
                stack: None,
                internal_routing: None,
            },
        }
    }

    pub fn locked(&mut self, locked: bool) -> &mut Self {
        self.params.locked = Some(locked);
        self
    }

    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.params.name = Some(name);
        self
    }

    pub fn team(&mut self, team: &'a str) -> &mut Self {
        self.params.team = Some(team);
        self
    }

    pub fn personal(&mut self, personal: bool) -> &mut Self {
        self.params.personal = Some(personal);
        self
    }

    pub fn region(&mut self, region: &'a str) -> &mut Self {
        self.params.region = Some(region);
        self
    }

    pub fn space(&mut self, space: &'a str) -> &mut Self {
        self.params.space = Some(space);
        self
    }

    pub fn stack(&mut self, stack: &'a str) -> &mut Self {
        self.params.stack = Some(stack);
        self
    }

    pub fn internal_routing(&mut self, internal_routing: bool) -> &mut Self {
        self.params.internal_routing = Some(internal_routing);
        self
    }

    pub fn build(&self) -> TeamAppCreate<'a> {
        TeamAppCreate {
            params: TeamAppCreateParams {
                locked: self.params.locked,
                name: self.params.name,
                team: self.params.team,
                personal: self.params.personal,
                region: self.params.region,
                space: self.params.space,
                stack: self.params.stack,
                internal_routing: self.params.internal_routing,
            },
        }
    }
}

/// Create a new team app with parameters
///
/// All parameters are optional
///
/// [See Heroku documentation for more information about these paramters](https://devcenter.heroku.com/articles/platform-api-reference#team-app-create-optional-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct TeamAppCreateParams<'a> {
    /// are other team members forbidden from joining this app.
    pub locked: Option<bool>,
    /// name of app
    /// pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
    pub name: Option<&'a str>,
    /// unique name of team
    pub team: Option<&'a str>,
    /// force creation of the app in the user account even if a default team is set.
    pub personal: Option<bool>,
    /// name of region
    pub region: Option<&'a str>,
    /// unique name of space
    ///  pattern: `^[a-z0-9](?:[a-z0-9]
    pub space: Option<&'a str>,
    /// unique name
    pub stack: Option<&'a str>,
    /// describes whether a Private Spaces app is externally routable or not
    pub internal_routing: Option<bool>,
}

impl<'a> HerokuEndpoint<TeamApp, (), TeamAppCreateParams<'a>> for TeamAppCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("teams/apps")
    }
    fn body(&self) -> Option<TeamAppCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Team Invitation Accept
///
/// Accept Team Invitation
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-invitation-accept)
pub struct TeamInvitationAccept<'a> {
    /// unique token identifier
    pub token_id: &'a str,
}

impl<'a> TeamInvitationAccept<'a> {
    pub fn new(token_id: &'a str) -> TeamInvitationAccept<'a> {
        TeamInvitationAccept { token_id }
    }
}

impl<'a> HerokuEndpoint<TeamInvitation> for TeamInvitationAccept<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("teams/invitations/{}/accept", self.token_id)
    }
}

/// Team Member Create
///
/// Create a new team member.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-member-create)
pub struct TeamMemberCreate<'a> {
    /// unique team identifier
    pub team_id: &'a str,
    /// parameters to pass to Heroku
    pub params: TeamMemberCreateParams<'a>,
}

impl<'a> TeamMemberCreate<'a> {
    /// Only required parameters passed
    pub fn new(team_id: &'a str, email: &'a str, role: &'a str) -> TeamMemberCreate<'a> {
        TeamMemberCreate {
            team_id,
            params: TeamMemberCreateParams {
                email: email,
                role: role,
                federated: None,
            },
        }
    }

    pub fn federated(&mut self, federated: bool) -> &mut Self {
        self.params.federated = Some(federated);
        self
    }

    pub fn build(&self) -> TeamMemberCreate<'a> {
        TeamMemberCreate {
            team_id: self.team_id,
            params: TeamMemberCreateParams {
                email: self.params.email,
                role: self.params.role,
                federated: self.params.federated,
            },
        }
    }
}

/// Create team member with parameters
///
/// [See Heroku documentation for more information about these paramters](https://devcenter.heroku.com/articles/platform-api-reference#team-member-create-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct TeamMemberCreateParams<'a> {
    /// unique email address
    pub email: &'a str,
    /// Even though marked with `Option`, this parameter is NOT optional.
    /// role in the team
    /// one of:"admin" or "collaborator" or "member" or "owner" or null
    pub role: &'a str,
    /// whether the user is federated and belongs to an Identity Provider
    pub federated: Option<bool>,
}

impl<'a> HerokuEndpoint<TeamMember, (), TeamMemberCreateParams<'a>> for TeamMemberCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("teams/{}/members", self.team_id)
    }
    fn body(&self) -> Option<TeamMemberCreateParams<'a>> {
        Some(self.params.clone())
    }
}
