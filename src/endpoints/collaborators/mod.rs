use crate::framework::response::ApiResult;
use serde::Deserialize;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use get::{CollaboratorDetails, CollaboratorList};
pub use post::{CollaboratorCreate, CollaboratorCreateParams};
pub use delete::CollaboratorDelete;

impl ApiResult for Collaborator {}
impl ApiResult for Vec<Collaborator> {}

/// Collaborator
/// A collaborator represents an account that has been given access to an app on Heroku.
/// https://devcenter.heroku.com/articles/platform-api-reference#collaborator
// TODO: (ben) inspect the nullable properties more. As of 22th March 2020, Heroku docs do not properly reflect nullable properties but some are,
//     and that's leading to an error decoding response body. e.g.  missing field `permissions` at line 1 column 297
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Collaborator {
    pub app: App,
    pub created_at: String,
    pub id: String,
    pub permissions: Option<Vec<Permission>>,
    pub role: Option<String>,
    pub updated_at: String,
    pub user: User,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct App {
    pub name: String,
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Permission {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub email: String,
    pub federated: bool,
    pub id: String,
}
