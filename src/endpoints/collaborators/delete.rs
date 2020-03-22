//Anything related to DELETE requests for collaborators and it's properties goes here.
use super::Collaborator;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Collaborator Delete
/// Delete an existing collaborator.
/// https://devcenter.heroku.com/articles/platform-api-reference#collaborator-delete
pub struct CollaboratorDelete {
    /// app_id can be the app name or id.
    pub app_id: String,
    /// collaborator_id can be the collaborator email or id.
    pub collaborator_id: String,
}

impl HerokuEndpoint<Collaborator> for CollaboratorDelete {
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
