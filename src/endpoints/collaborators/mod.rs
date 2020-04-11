use crate::framework::response::ApiResult;
use serde::Deserialize;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::CollaboratorDelete;
pub use get::{
    CollaboratorDetails, CollaboratorList, TeamCollaboratorDetails, TeamCollaboratorList,
};
pub use post::{
    CollaboratorCreate, CollaboratorCreateParams, TeamCollaboratorCreate,
    TeamCollaboratorCreateParams,
};

pub use patch::{TeamCollaboratorUpdate, TeamCollaboratorUpdateParams};

impl ApiResult for Collaborator {}
impl ApiResult for Vec<Collaborator> {}

impl ApiResult for TeamCollaborator {}
impl ApiResult for Vec<TeamCollaborator> {}

pub use team_collaborator::TeamCollaborator;

/// Collaborator
///
/// Stability: production
///
/// A collaborator represents an account that has been given access to an app on Heroku.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#collaborator)
///
// TODO: (ben) inspect the nullable properties more. As of 22th March 2020, Heroku docs do not properly reflect nullable properties but some are,
//     and that's leading to an error decoding response body. e.g.  missing field `permissions` at line 1 column 297
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Collaborator {
    /// App struct
    pub app: App,
    /// when collaborator was created
    pub created_at: String,
    /// unique identifier of collaborator
    pub id: String,
    /// list of permissions this collaborator has.
    pub permissions: Option<Vec<Permission>>,
    /// role in the team. One of:"admin" or "collaborator" or "member" or "owner" or null
    pub role: Option<String>,
    /// when collaborator was updated
    pub updated_at: String,
    /// Account struct
    pub user: User,
}

/// App struct
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct App {
    /// name of app. pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
    pub name: String,
    /// unique identifier
    pub id: String,
}

/// Permission struct
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Permission {
    /// permission name
    pub name: String,
    /// brief description about the permission
    pub description: String,
}

/// Account struct
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct User {
    /// unique email address of account
    pub email: String,
    /// whether the user is federated and belongs to an Identity Provider
    pub federated: bool,
    // unique identifier of an account
    pub id: String,
}

mod team_collaborator {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Team App Collaborator
    ///
    /// Stability: development
    ///
    /// A team collaborator represents an account that has been given access to a team app on Heroku.
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#collaborator)
    ///
    // TODO: (ben) inspect the nullable properties more. As of 5th April 2020, Heroku docs do not properly reflect nullable properties.
    //     and that's leading to an error decoding response body. e.g.  missing field `permissions`
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct TeamCollaborator {
        /// App struct
        pub app: App,
        /// when collaborator was created
        pub created_at: DateTime<Utc>,
        /// unique identifier of collaborator
        pub id: String,
        /// list of permissions this collaborator has.
        pub permissions: Option<Vec<Permission>>,
        /// role in the team. One of:"admin" or "collaborator" or "member" or "owner" or null
        pub role: Option<String>,
        /// when collaborator was updated
        pub updated_at: DateTime<Utc>,
        /// Account struct
        pub user: User,
    }
    /// App struct
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct App {
        /// name of app. pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
        pub name: String,
        /// unique identifier
        pub id: String,
    }

    /// Permission struct
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Permission {
        /// permission name
        pub name: String,
        /// brief description about the permission
        pub description: String,
    }

    /// Account struct
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct User {
        /// unique email address of account
        pub email: String,
        /// whether the user is federated and belongs to an Identity Provider
        pub federated: bool,
        // unique identifier of an account
        pub id: String,
    }
}
