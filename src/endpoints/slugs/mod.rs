use crate::framework::response::ApiResult;

use chrono::offset::Utc;
use chrono::DateTime;
use std::collections::HashMap;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use get::SlugDetails;
pub use post::{SlugCreate, SlugCreateParams};

impl ApiResult for Slug {}
impl ApiResult for Vec<Slug> {}

/// Slug
///
/// Stability: production
///
/// A slug is a snapshot of your application code that is ready to run on the platform.
///
/// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#slug)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Slug {
    /// slug blob
    pub blob: Blob,
    /// description from buildpack of slug
    pub buildpack_provided_description: Option<String>,
    /// an optional checksum of the slug for verifying its integrity
    pub checksum: Option<String>,
    /// identification of the code with your version control system (eg: SHA of the git HEAD)
    pub commit: Option<String>,
    /// an optional description of the provided commit
    pub commit_description: Option<String>,
    /// when slug was created
    pub created_at: DateTime<Utc>,
    /// unique identifier of slug
    pub id: String,
    /// hash mapping process type names to their respective command
    pub process_types: Option<HashMap<String, String>>,
    /// size of slug, in bytes
    pub size: Option<i64>,
    /// stack
    pub stack: Stack,
    /// when slug was updated
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Blob {
    /// method to be used to interact with the slug blob
    pub method: String,
    /// URL to interact with the slug blob
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ProcessTypes {
    /// hash mapping process type names to their respective command
    pub web: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Stack {
    /// identifier of stack
    pub id: String,
    /// when slug was updated
    pub name: String,
}
