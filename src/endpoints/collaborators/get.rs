//Anything related to GET requests for collaborators and it's properties goes here.
use super::Collaborator;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Collaborator List
/// List existing collaborators.
/// https://devcenter.heroku.com/articles/platform-api-reference#collaborator-list
pub struct CollaboratorList {
    /// app_id can be the app name or id.
    pub app_id: String,
}

impl HerokuEndpoint<Vec<Collaborator>> for CollaboratorList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/collaborators", self.app_id)
    }
}

/// Collaborator Info
/// Info for existing collaborator.
/// https://devcenter.heroku.com/articles/platform-api-reference#collaborator-list
pub struct CollaboratorDetails {
    /// app_id can be the app name or id.
    pub app_id: String,
    /// collaborator_id can be the collaborator email or id.
    pub collaborator_id: String,
}

impl HerokuEndpoint<Collaborator> for CollaboratorDetails {
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
