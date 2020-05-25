//Anything related to POST requests for collaborators and it's properties goes here.
use super::{Collaborator, TeamCollaborator};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Collaborator Create
///
/// Create a new collaborator.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#collaborator-create)
///
/// # Example:
///
/// CollaboratorCreate takes two required parameters, app_id, user, and returns the created [`Collaborator`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let app_collab = &CollaboratorCreate::new("APP_ID", "ACCOUNT_ID")
///     .silent(false)
///     .build();
/// let response = api_client.request(app_collab);
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
pub struct CollaboratorCreate<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: CollaboratorCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> CollaboratorCreate<'a> {
    pub fn new(app_id: &'a str, user: &'a str) -> CollaboratorCreate<'a> {
        CollaboratorCreate {
            app_id,
            params: CollaboratorCreateParams { user, silent: None },
        }
    }

    /// # silent: whether to suppress email invitation when creating collaborator
    pub fn silent(&mut self, silent: bool) -> &mut Self {
        self.params.silent = Some(silent);
        self
    }

    pub fn build(&self) -> CollaboratorCreate<'a> {
        CollaboratorCreate {
            app_id: self.app_id,
            params: CollaboratorCreateParams {
                user: self.params.user,
                silent: self.params.silent,
            },
        }
    }
}

/// Create collaborator with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#collaborator-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct CollaboratorCreateParams<'a> {
    /// unique email address, identifier of an account or Implicit reference to currently authorized user
    pub user: &'a str,
    /// whether to suppress email invitation when creating collaborator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
}

impl<'a> HerokuEndpoint<Collaborator, (), CollaboratorCreateParams<'a>> for CollaboratorCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/collaborators", self.app_id)
    }
    fn body(&self) -> Option<CollaboratorCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Team App Collaborator Create
///
/// Create a new collaborator on a team app. Use this endpoint instead of the /apps/{app_id_or_name}/collaborator endpoint when you want the collaborator to be granted permissions according to their role in the team.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-app-collaborator-create)
///
/// # Example:
///
/// TeamCollaboratorCreate takes two required parameters, app_id, user, and returns the created [`TeamCollaborator`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let permissions = vec!["view"];
///
/// let team_app_collab = &TeamCollaboratorCreate::new("APP_ID", "ACCOUNT_ID")
///     .permissions(permissions)
///     .silent(false)
///     .build();
/// let response = api_client.request(team_app_collab);
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
pub struct TeamCollaboratorCreate<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: TeamCollaboratorCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> TeamCollaboratorCreate<'a> {
    pub fn new(app_id: &'a str, user: &'a str) -> TeamCollaboratorCreate<'a> {
        TeamCollaboratorCreate {
            app_id,
            params: TeamCollaboratorCreateParams {
                user: user,
                silent: None,
                permissions: None,
            },
        }
    }

    /// # silent: whether to suppress email invitation when creating collaborator
    pub fn silent(&mut self, silent: bool) -> &mut Self {
        self.params.silent = Some(silent);
        self
    }

    /// # permissions: An array of permissions to give to the collaborator.
    pub fn permissions(&mut self, permissions: Vec<&'a str>) -> &mut Self {
        self.params.permissions = Some(permissions);
        self
    }

    pub fn build(&self) -> TeamCollaboratorCreate<'a> {
        TeamCollaboratorCreate {
            app_id: self.app_id,
            params: TeamCollaboratorCreateParams {
                user: self.params.user,
                silent: self.params.silent,
                permissions: self.params.permissions.clone(),
            },
        }
    }
}

/// Create team app collaborator with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#team-app-collaborator-create-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct TeamCollaboratorCreateParams<'a> {
    /// unique email address, identifier of an account or Implicit reference to currently authorized user
    pub user: &'a str,
    /// whether to suppress email invitation when creating collaborator
    pub silent: Option<bool>,
    /// An array of permissions to give to the collaborator.
    pub permissions: Option<Vec<&'a str>>,
}

impl<'a> HerokuEndpoint<TeamCollaborator, (), TeamCollaboratorCreateParams<'a>>
    for TeamCollaboratorCreate<'a>
{
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("teams/apps/{}/collaborators", self.app_id)
    }
    fn body(&self) -> Option<TeamCollaboratorCreateParams<'a>> {
        Some(self.params.clone())
    }
}
