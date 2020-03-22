use crate::framework::response::ApiResult;
use serde::Deserialize;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::{DynoAllRestart, DynoRestart};
pub use get::{DynoDetails, DynoList};

impl ApiResult for Dyno {}
impl ApiResult for Vec<Dyno> {}

/// Heroku Dyno
/// Stability: production
/// Dynos encapsulate running processes of an app on Heroku
/// https://devcenter.heroku.com/articles/platform-api-reference#dyno
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Dyno {
    pub app: App,
    pub attach_url: Option<String>,
    pub command: String,
    pub created_at: String,
    pub id: String,
    pub name: String,
    pub release: Release,
    pub size: String,
    pub state: String,
    pub r#type: String, //type is a keyword in Rust
    pub updated_at: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct App {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Release {
    pub id: String,
    pub version: i64,
}
