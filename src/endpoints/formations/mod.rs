use crate::framework::response::ApiResult;
use serde::Deserialize;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use get::{FormationDetails, FormationList};
pub use patch::{FormationUpdate, FormationUpdateParams};

impl ApiResult for Formation {}
impl ApiResult for Vec<Formation> {}

/// Heroku Formation
///
/// Stability: production
///
/// Formation of processes that should be maintained for an app
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#formation)

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Formation {
    /// An app represents the program that you would like to deploy and run on Heroku
    pub app: App,
    /// command to use to launch this process
    pub command: String,
    /// when the process type was created
    pub created_at: String,
    /// unique identifier of this process type
    pub id: String,
    /// number of processes to maintain
    pub quantity: i32,
    /// dyno size
    pub size: String,
    /// type of process to maintain pattern: ^[-\w]{1,128}$
    pub r#type: String,
    /// when dyno type was updated
    pub updated_at: String,
}
 
/// An app represents the program that you would like to deploy and run on Heroku.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct App {
    /// unique identifier
    pub id: String,
    /// name of app pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
    pub name: String,
}