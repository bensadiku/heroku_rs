use crate::framework::response::ApiResult;
use serde::Deserialize;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::BuildDelete;
pub use get::{BuildDetails, BuildList, BuildPackInstallationList};
pub use post::{BuildCreate, BuildCreateParams, BuildpackParam, SourceBlobParam};
pub use put::{BuildpackInstallationUpdate, BuildpackInstallationUpdateParams};

impl ApiResult for Build {}
impl ApiResult for Vec<Build> {}

impl ApiResult for BuildpackInstallation {}
impl ApiResult for Vec<BuildpackInstallation> {}

/// Build
///
/// Stability: production
///
/// A build represents the process of transforming a code tarball into a slug.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#build)
//  TODO: (ben) inspect the nullable properties more. As of 22th March 2020, Heroku docs do not properly reflect the nullable properties of this struct.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Build {
    /// App struct used for the Build
    pub app: App,
    /// buildpacks executed for this build, in order
    pub buildpacks: Option<Vec<Buildpack>>,
    /// when build was created
    pub created_at: String,
    /// unique identifier of build
    pub id: String,
    /// Build process output will be available from this URL as a stream.
    /// The stream is available as either text/plain or text/event-stream.
    /// Clients should be prepared to handle disconnects and can resume the stream by sending a Range header (for text/plain) or a Last-Event-Id header (for text/event-stream).
    pub output_stream_url: String,
    /// Source blob struct containing url, checksum and version
    pub source_blob: SourceBlob,
    /// release resulting from the build
    pub release: Option<Release>,
    /// slug created by this build
    pub slug: Option<Slug>,
    /// stack of build
    pub stack: String,
    /// status of build. One of:"failed" or "pending" or "succeeded"
    pub status: String,
    /// when build was updated
    pub updated_at: String,
    /// identifier of an account
    pub user: User,
}

/// App struct used for the Build
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct App {
    /// unique identifier
    pub id: String,
}

/// Buildpack
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Buildpack {
    /// location of the buildpack for the app. Either a url (unofficial buildpacks) or an internal urn (heroku official buildpacks).
    pub url: String,
    /// either the Buildpack Registry name or a URL of the buildpack for the app
    pub name: Option<String>,
}

/// SourceBlob
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct SourceBlob {
    /// an optional checksum of the gzipped tarball for verifying its integrity
    pub checksum: Option<String>,
    /// URL where gzipped tar archive of source code for build was downloaded.
    pub url: String,
    /// Version of the gzipped tarball.
    pub version: Option<String>,
}

/// release resulting from the build
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Release {
    /// unique identifier of release
    pub id: String,
}

/// slug created by this build
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Slug {
    /// unique identifier of slug
    pub id: String,
}

/// Account
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct User {
    /// identifier of an account
    pub id: String,
    /// unique email address
    pub email: String,
}

/// Buildpack Installations
///
/// A buildpack installation represents a buildpack that will be run against an app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#buildpack-installations)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct BuildpackInstallation {
    /// determines the order in which the buildpacks will execute
    pub ordinal: i64,
    /// The buildpack that will be executed
    pub buildpack: Buildpack,
}

/// The struct to update BuildpackInstallation
#[derive(Serialize, Clone, Debug)]
pub struct Update {
    /// The buildpack attribute can accept a name, a url, or a urn.
    pub buildpack: String,
}
