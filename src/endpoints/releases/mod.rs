use crate::framework::response::ApiResult;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use get::{ReleaseDetails, ReleaseList};
pub use post::{ReleaseCreate, ReleaseCreateParams, ReleaseRollback, ReleaseRollbackParams};

pub use app_release::AppRelease;

impl ApiResult for AppRelease {}
impl ApiResult for Vec<AppRelease> {}

// app realease submodule, anything from apps/{app_id_or_name}/releases goes here.
mod app_release {

    use chrono::offset::Utc;
    use chrono::DateTime;

    /// App Release
    ///
    /// Stability: production
    ///
    /// A release represents a combination of code, config vars and add-ons for an app on Heroku.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#release)
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct AppRelease {
        /// add-on plans installed on the app for this release
        pub addon_plan_names: Vec<String>,
        /// app released
        pub app: App,
        /// when release was created
        pub created_at: DateTime<Utc>,
        /// description of changes in this release
        pub description: String,
        /// unique identifier of release
        pub id: String,
        /// when release was updated
        pub updated_at: DateTime<Utc>,
        /// slug running in this release
        pub slug: Option<Slug>,
        /// current status of the release
        /// one of:"failed" or "pending" or "succeeded"
        pub status: String,
        /// user
        pub user: User,
        /// unique version assigned to the release
        pub version: i64,
        /// indicates this release as being the current one for the app
        pub current: bool,
        /// Release command output will be available from this URL as a stream.
        /// The stream is available as either text/plain or text/event-stream.
        /// Clients should be prepared to handle disconnects and can resume the stream by sending a Range header (for text/plain) or a Last-Event-Id header (for text/event-stream).
        pub output_stream_url: Option<String>,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct App {
        /// name of app.
        /// pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
        pub name: String,
        /// unique identifier
        pub id: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Slug {
        /// unique identifier of slug
        pub id: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct User {
        /// identifier of an account
        pub id: String,
        /// unique email address
        pub email: String,
    }
}
