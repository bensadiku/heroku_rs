//Anything related to GET requests for collaborators and it's properties goes here.
use super::{Collaborator, TeamCollaborator};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Collaborator List
///
/// List existing collaborators.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#collaborator-list)
///
/// # Example:
///
/// CollaboratorList takes one required parameter, app_id, and returns a list of [`Collaborators`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&CollaboratorList::new("APP_ID_HERE"));
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
/// [response]: ../struct.Collaborator.html
pub struct CollaboratorList<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> CollaboratorList<'a> {
    pub fn new(app_id: &'a str) -> CollaboratorList {
        CollaboratorList { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<Collaborator>> for CollaboratorList<'a> {
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
/// # Example:
///
/// CollaboratorDetails takes two required parameters, app_id and collaborator_id, and returns a [`Collaborator`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&CollaboratorDetails::new("APP_ID_HERE", "COLLABORATOR_EMAIL_OR_ID"));
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
/// [response]: ../struct.Collaborator.html
pub struct CollaboratorDetails<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// collaborator_id can be the collaborator email or id.
    pub collaborator_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> CollaboratorDetails<'a> {
    pub fn new(app_id: &'a str, collaborator_id: &'a str) -> CollaboratorDetails<'a> {
        CollaboratorDetails {
            app_id,
            collaborator_id,
        }
    }
}

impl<'a> HerokuEndpoint<Collaborator> for CollaboratorDetails<'a> {
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
///
/// # Example:
///
/// TeamCollaboratorList takes one required parameter, app_id, and returns a list of [`TeamCollaborators`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&TeamCollaboratorList::new("APP_ID"));
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
/// [response]: ../struct.TeamCollaborator.html
pub struct TeamCollaboratorList<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
}
#[cfg(feature = "builder")]
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
/// # Example:
///
/// TeamCollaboratorDetails takes two required parameters, app_id and collaborator_id, and returns a [`TeamCollaborator`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&TeamCollaboratorDetails::new("APP_ID_HERE", "COLLABORATOR_EMAIL_OR_ID"));
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
/// [response]: ../struct.TeamCollaborator.html
pub struct TeamCollaboratorDetails<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// collaborator_id can be the collaborator email or id.
    pub collaborator_id: &'a str,
}

#[cfg(feature = "builder")]
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
