//Anything related to POST requests for heroku logs and it's properties goes here.
use super::{LogDrain, LogSession};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Log Drain Create
///
/// Create a new log drain.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#log-drain-create)
pub struct LogDrainCreate<'a> {
    /// unique app identifier, either app name, or app id
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: LogDrainCreateParams<'a>,
}

impl<'a> LogDrainCreate<'a> {
    pub fn new(app_id: &'a str, url: &'a str) -> LogDrainCreate<'a> {
        LogDrainCreate {
            app_id,
            params: LogDrainCreateParams { url },
        }
    }
}

/// Create a new log drain with parameters.
///
/// [See Heroku documentation for more information about these paramters](https://devcenter.heroku.com/articles/platform-api-reference#log-drain-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct LogDrainCreateParams<'a> {
    /// url associated with the log drain
    pub url: &'a str,
}

impl<'a> HerokuEndpoint<LogDrain, (), LogDrainCreateParams<'a>> for LogDrainCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/log-drains", self.app_id)
    }
    fn body(&self) -> Option<LogDrainCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Log Session Create
///
/// Create a new log session.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#log-session-create)
pub struct LogSessionCreate<'a> {
    /// unique app identifier, either app name, or app id
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: Option<LogSessionCreateParams<'a>>,
}

impl<'a> LogSessionCreate<'a> {
    /// Create a new log session with required parameters
    pub fn new(
        app_id: &'a str,
        dyno: Option<&'a str>,
        lines: Option<i64>,
        source: Option<&'a str>,
        tail: Option<bool>,
    ) -> LogSessionCreate<'a> {
        LogSessionCreate {
            app_id,
            params: Some(LogSessionCreateParams {
                dyno: dyno,
                lines: lines,
                source: source,
                tail: tail,
            }),
        }
    }

    /// Create a new log session without parameters
    pub fn create(app_id: &'a str) -> LogSessionCreate<'a> {
        LogSessionCreate {
            app_id,
            params: None,
        }
    }
}

/// Create a new log drain with parameters.
///
/// [See Heroku documentation for more information about these paramters](https://devcenter.heroku.com/articles/platform-api-reference#log-session-create-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct LogSessionCreateParams<'a> {
    /// dyno to limit results to
    pub dyno: Option<&'a str>,
    /// number of log lines to stream at once
    pub lines: Option<i64>,
    /// log source to limit results to
    pub source: Option<&'a str>,
    /// whether to stream ongoing logswhether to stream ongoing logs
    pub tail: Option<bool>,
}

impl<'a> HerokuEndpoint<LogSession, (), LogSessionCreateParams<'a>> for LogSessionCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/log-sessions", self.app_id)
    }
    fn body(&self) -> Option<LogSessionCreateParams<'a>> {
        self.params.clone()
    }
}
