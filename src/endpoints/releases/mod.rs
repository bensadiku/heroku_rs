use crate::framework::response::ApiResult;
use serde::Deserialize;

pub mod get;
pub mod post;

pub use get::{ReleaseList, ReleaseInfo};

impl ApiResult for Release {}
impl ApiResult for Vec<Release> {}

/// Heroku Release
///
/// Stability: production
///
/// A release represents a combination of code, config vars and add-ons for an app on Heroku
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#release)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Release {
   /// add-on plans installed on the app for this release
   pub addon_plan_names: Vec<i64>,
   /// An app represents the program that you would like to deploy and run on Heroku
   pub app: App,
   /// when release was created
   pub created_at: String,
   /// indicates this release as being the current one for the app
   pub current: bool,
   /// description of changes in this release
   pub description: String,
   /// unique identifier of this process type
   pub id: String,
   /// Relase command output will be available from this URL as a stream 
   pub output_stream_url: Option<String>,
   /// slug running this release
   pub slug: Option<Slug>,
   /// current status of the release - failed, pending, or succeeded
   pub status: String,
   /// when release was updated
   pub updated_at: String,
   /// user account running release
   pub user: User,
   /// unique version assigned to the release
   pub version: i64
}

/// An app represents the program that you would like to deploy and run on Heroku.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct App {
    /// unique identifier
    pub id: String,
    /// name of app pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
    pub name: String,
}

/// A slug running the release
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Slug {
    /// unique identifier of slug
    pub id: String
}

/// User account running release
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct User {
   /// identifier of an account
   pub id: String,
   /// unique email address
   pub email: String,
}