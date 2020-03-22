use crate::framework::response::ApiResult;
use serde::Deserialize;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::BuildDelete;
pub use get::{BuildDetails, BuildList, BuildPackInstallationList};
pub use post::{BuildCreate, BuildCreateParams};
pub use put::{BuildpackInstallationUpdate, BuildpackInstallationUpdateParams};

impl ApiResult for Build {}
impl ApiResult for Vec<Build> {}

impl ApiResult for BuildpackInstallation {}
impl ApiResult for Vec<BuildpackInstallation> {}

/// Build
/// Stability: production
/// A build represents the process of transforming a code tarball into a slug.
/// https://devcenter.heroku.com/articles/platform-api-reference#build
//  TODO: (ben) inspect the nullable properties more. As of 22th March 2020, Heroku docs do not properly reflect the nullable properties of this struct.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Build {
    pub app: App,
    pub buildpacks: Option<Vec<Buildpack>>,
    pub created_at: String,
    pub id: String,
    pub output_stream_url: String,
    pub source_blob: SourceBlob,
    pub release: Option<Release>,
    pub slug: Option<Slug>,
    pub stack: String,
    pub status: String,
    pub updated_at: String,
    pub user: User,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct App {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Buildpack {
    pub url: String,
    pub name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct SourceBlob {
    pub checksum: Option<String>,
    pub url: String,
    pub version: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Release {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Slug {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub email: String,
}

/// Buildpack Installations
/// A buildpack installation represents a buildpack that will be run against an app.
/// https://devcenter.heroku.com/articles/platform-api-reference#buildpack-installations
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct BuildpackInstallation {
    pub ordinal: i64,
    pub buildpack: Buildpack,
}

#[derive(Serialize, Clone, Debug)]
pub struct Update {
    pub buildpack: String,
}
