//Anything related to DELETE requests for collaborators and it's properties goes here.
use super::{Collaborator, TeamCollaborator};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Collaborator Delete
///
/// Delete an existing collaborator.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#collaborator-delete)
///
/// # Example:
///
/// CollaboratorDelete takes two required parameters, app_id and collaborator_id, and returns the deleted [`Collaborator`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&CollaboratorDelete::new("APP_ID_HERE", "COLLABORATOR_EMAIL_OR_ID"));
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
pub struct CollaboratorDelete<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// collaborator_id can be the collaborator email or id.
    pub collaborator_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> CollaboratorDelete<'a> {
    pub fn new(app_id: &'a str, collaborator_id: &'a str) -> CollaboratorDelete<'a> {
        CollaboratorDelete {
            app_id,
            collaborator_id,
        }
    }
}

impl<'a> HerokuEndpoint<Collaborator> for CollaboratorDelete<'a> {
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
///
/// # Example:
///
/// TeamCollaboratorDelete takes two required parameters, app_id and collaborator_id, and returns the deleted [`TeamCollaborator`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&TeamCollaboratorDelete::new("APP_ID_HERE", "COLLABORATOR_EMAIL_OR_ID"));
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
pub struct TeamCollaboratorDelete<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// collaborator_id can be the collaborator email or id.
    pub collaborator_id: &'a str,
}

#[cfg(feature = "builder")]
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
