use crate::framework::response::ApiResult;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::LogDrainDelete;
pub use get::{LogDrainDetails, LogDrainList, LogDrainListByAddon};
pub use post::{LogDrainCreate, LogDrainCreateParams, LogSessionCreate, LogSessionCreateParams};
pub use put::{LogDrainUpdate, LogDrainUpdateParams};

impl ApiResult for LogDrain {}
impl ApiResult for Vec<LogDrain> {}

impl ApiResult for LogSession {}

pub use log_drains::LogDrain;
pub use log_sessions::LogSession;

// log drains submodule, anything from /log-drains goes here.
mod log_drains {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Log Drain
    ///
    /// Stability: production
    ///
    /// [Log drains](https://devcenter.heroku.com/articles/log-drains) provide a way to forward your Heroku logs to an external syslog server for long-term archiving.
    /// This external service must be configured to receive syslog packets from Heroku, whereupon its URL can be added to an app using this API.
    /// Some add-ons will add a log drain when they are provisioned to an app.
    /// These drains can only be removed by removing the add-on.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#log-drain)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct LogDrain {
        /// add-on that created the drain
        pub addon: Option<Addon>,
        /// when log drain was created
        pub created_at: DateTime<Utc>,
        /// unique identifier of this log drain
        pub id: String,
        /// token associated with the log drain
        pub token: String,
        /// when log drain was updated
        pub updated_at: DateTime<Utc>,
        /// url associated with the log drain
        pub url: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct Addon {
        /// unique identifier
        pub id: String,
        /// globally name of the add-on
        ///  pattern: ^[a-zA-Z][A-Za-z0-9_-]+$
        pub name: String,
    }
}

mod log_sessions {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Log Session
    ///
    /// Stability: production
    ///
    /// A log session is a reference to the http based log stream for an app.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#log-session)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct LogSession {
        /// when log connection was created
        pub created_at: DateTime<Utc>,
        /// unique identifier of this log session
        pub id: String,
        /// URL for log streaming session
        pub logplex_url: String,
        /// when log session was updated
        pub updated_at: DateTime<Utc>,
    }
}
