//Anything related to PATCH requests for Teams and it's variations goes here.
use super::{Team, TeamApp, TeamMember, TeamPreferences};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Team Update
///
/// Update team properties.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-update)
///
/// # Example:
///
/// TeamUpdate takes one required parameter, team_id and returns a [`Team`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(
///     &TeamUpdate::new("TEAM_ID")
///         .default(false)
///         .name("new-team-name")
///         .build(),
/// );
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
pub struct TeamUpdate<'a> {
    /// team_id is the unique team identifier.
    pub team_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: TeamUpdateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> TeamUpdate<'a> {
    pub fn new(team_id: &'a str) -> TeamUpdate<'a> {
        TeamUpdate {
            team_id,
            params: TeamUpdateParams {
                default: None,
                name: None,
            },
        }
    }

    /// # default: whether to use this team when none is specified
    pub fn default(&mut self, default: bool) -> &mut Self {
        self.params.default = Some(default);
        self
    }

    /// # name: unique name of team
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.params.name = Some(name);
        self
    }

    pub fn build(&self) -> TeamUpdate<'a> {
        TeamUpdate {
            team_id: self.team_id,
            params: TeamUpdateParams {
                default: self.params.default,
                name: self.params.name,
            },
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
        Some(self.params.clone())
    }
}

/// Team App Update Locked
///
/// Lock or unlock a team app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-app-update-locked)
///
/// # Example:
///
/// TeamAppUpdateLocked takes two required parameters, team_id and locked and returns the updated [`Team`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let locked = true;
/// let response = api_client.request(&TeamAppUpdateLocked::new("TEAM_ID", locked));
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
pub struct TeamAppUpdateLocked<'a> {
    /// team_id is the unique team identifier.
    pub team_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: TeamAppUpdateLockedParams,
}

#[cfg(feature = "builder")]
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
///
/// # Example:
///
/// TeamAppTransfer takes two required parameters, team_id and owner_id and returns the [`TeamApp`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&TeamAppTransfer::new("TEAM_ID", "OWNER_ID"));
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
/// [response]: ../struct.TeamApp.html
pub struct TeamAppTransfer<'a> {
    /// team_id is the unique team identifier.
    pub team_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: TeamAppTransferParams<'a>,
}

#[cfg(feature = "builder")]
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

/// Team Member Update
///
/// Update a team member.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-member-update)
///
/// # Example:
///
/// TeamMemberUpdate takes three required parameters, team_id email and role and returns the [`TeamMember`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&TeamMemberUpdate::new("TEAM_ID", "EMAIL", "ROLE"));
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
pub struct TeamMemberUpdate<'a> {
    /// unique team identifier
    pub team_id: &'a str,
    /// parameters to pass to Heroku
    pub params: TeamMemberUpdateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> TeamMemberUpdate<'a> {
    /// Only required parameters passed
    pub fn new(team_id: &'a str, email: &'a str, role: &'a str) -> TeamMemberUpdate<'a> {
        TeamMemberUpdate {
            team_id,
            params: TeamMemberUpdateParams {
                email: email,
                role: role,
                federated: None,
            },
        }
    }

    /// # federated: whether the user is federated and belongs to an Identity Provider
    pub fn federated(&mut self, federated: bool) -> &mut Self {
        self.params.federated = Some(federated);
        self
    }

    pub fn build(&self) -> TeamMemberUpdate<'a> {
        TeamMemberUpdate {
            team_id: self.team_id,
            params: TeamMemberUpdateParams {
                email: self.params.email,
                role: self.params.role,
                federated: self.params.federated,
            },
        }
    }
}

/// Update team member with parameters
///
/// [See Heroku documentation for more information about these paramters](https://devcenter.heroku.com/articles/platform-api-reference#team-member-update-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct TeamMemberUpdateParams<'a> {
    /// unique email address
    pub email: &'a str,
    /// Even though marked with `Option`, this parameter is NOT optional.
    /// role in the team
    /// one of:"admin" or "collaborator" or "member" or "owner" or null
    pub role: &'a str,
    /// whether the user is federated and belongs to an Identity Provider
    pub federated: Option<bool>,
}

impl<'a> HerokuEndpoint<TeamMember, (), TeamMemberUpdateParams<'a>> for TeamMemberUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("teams/{}/members", self.team_id)
    }
    fn body(&self) -> Option<TeamMemberUpdateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Team Preferences Update
///
/// Update Team Preferences
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-preferences-update)
///
/// # Example:
///
/// TeamPreferenceUpdate takes one required parameter, id and returns the [`TeamPreferences`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&TeamPreferenceUpdate::new("ID"));
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
/// [response]: ../struct.TeamPreferences.html
pub struct TeamPreferenceUpdate<'a> {
    /// unique identifier
    pub id: &'a str,
    /// parameters to pass to Heroku
    pub params: TeamPreferenceUpdateParams,
}

#[cfg(feature = "builder")]
impl<'a> TeamPreferenceUpdate<'a> {
    pub fn new(id: &'a str) -> TeamPreferenceUpdate<'a> {
        TeamPreferenceUpdate {
            id,
            params: TeamPreferenceUpdateParams {
                whitelisting_enabled: None,
            },
        }
    }

    /// # whitelisting_enabled: Whether whitelisting rules should be applied to add-on installations
    pub fn whitelisting_enabled(&mut self, whitelisting_enabled: bool) -> &mut Self {
        self.params.whitelisting_enabled = Some(whitelisting_enabled);
        self
    }

    pub fn build(&self) -> TeamPreferenceUpdate<'a> {
        TeamPreferenceUpdate {
            id: self.id,
            params: TeamPreferenceUpdateParams {
                whitelisting_enabled: self.params.whitelisting_enabled,
            },
        }
    }
}

/// Update team preference with parameters
///
/// [See Heroku documentation for more information about these paramters](https://devcenter.heroku.com/articles/platform-api-reference#team-preferences-update-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct TeamPreferenceUpdateParams {
    /// Whether whitelisting rules should be applied to add-on installations. [Nullable]
    #[serde(rename = "whitelisting-enabled")]
    pub whitelisting_enabled: Option<bool>,
}

impl<'a> HerokuEndpoint<TeamPreferences, (), TeamPreferenceUpdateParams> for TeamPreferenceUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("teams/{}/preferences", self.id)
    }
    fn body(&self) -> Option<TeamPreferenceUpdateParams> {
        Some(self.params.clone())
    }
}
