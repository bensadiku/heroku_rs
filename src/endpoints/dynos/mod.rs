use crate::framework::response::ApiResult;
use serde::Deserialize;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::{DynoAllRestart, DynoRestart};
pub use get::{DynoDetails, DynoList};
pub use post::{DynoCreate, DynoCreateParams};

impl ApiResult for Dyno {}
impl ApiResult for Vec<Dyno> {}

/// Heroku Dyno
///
/// Stability: production
///
/// Dynos encapsulate running processes of an app on Heroku
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#dyno)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Dyno {
    /// An app represents the program that you would like to deploy and run on Heroku.
    pub app: App,
    /// a URL to stream output from for attached processes or null for non-attached processes
    pub attach_url: Option<String>,
    /// command used to start this process
    pub command: String,
    /// when dyno was created
    pub created_at: String,
    /// unique identifier of this dyno
    pub id: String,
    /// the name of this process on this dyno
    pub name: String,
    /// A release represents a combination of code, config vars and add-ons for an app on Heroku.
    pub release: Release,
    /// dyno size (default: “standard-1X”)
    pub size: String,
    /// current status of process (either: crashed, down, idle, starting, or up)
    pub state: String,
    /// type of process
    pub r#type: String, //type is a keyword in Rust
    /// when process last changed state
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

/// A release represents a combination of code, config vars and add-ons for an app on Heroku.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Release {
    /// unique identifier of release
    pub id: String,
    /// unique version assigned to the release
    pub version: i64,
}
