//Anything related to POST requests for heroku logs and it's properties goes here.
use super::{LogDrain, LogSession};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Log Drain Create
///
/// Create a new log drain.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#log-drain-create)
/// 
/// # Example:
///
/// LogDrainCreate takes two required parameters, app_id and url, and returns the new [`LogDrain`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let url = "https://mycoolherokuappname.herokuapp.com/";
/// let response = api_client.request(&LogDrainCreate::new("APP_ID", url));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
///
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.LogDrain.html
pub struct LogDrainCreate<'a> {
    /// unique app identifier, either app name, or app id
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: LogDrainCreateParams<'a>,
}

#[cfg(feature = "builder")]
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
/// 
/// # Example:
///
/// LogSessionCreate takes one required parameter, app_id, and returns the new [`LogSession`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(
///     &LogSessionCreate::new("APP_ID")
///         .dyno("web.1")
///         .source("app")
///         .lines(10)
///         .tail(false)
///         .build(),
/// );
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
///
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.LogSession.html
pub struct LogSessionCreate<'a> {
    /// unique app identifier, either app name, or app id
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: LogSessionCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> LogSessionCreate<'a> {
    /// Create a new log session with required parameters
    pub fn new(app_id: &'a str) -> LogSessionCreate<'a> {
        LogSessionCreate {
            app_id,
            params: LogSessionCreateParams {
                dyno: None,
                lines: None,
                source: None,
                tail: None,
            },
        }
    }

    /// # dyno: dyno to limit results to
    pub fn dyno(&mut self, dyno: &'a str) -> &mut Self {
        self.params.dyno = Some(dyno);
        self
    }
    /// # lines: number of log lines to stream at once
    pub fn lines(&mut self, lines: i64) -> &mut Self {
        self.params.lines = Some(lines);
        self
    }
    /// # source: log source to limit results to
    pub fn source(&mut self, source: &'a str) -> &mut Self {
        self.params.source = Some(source);
        self
    }
    /// # tail: whether to stream ongoing logs
    pub fn tail(&mut self, tail: bool) -> &mut Self {
        self.params.tail = Some(tail);
        self
    }

    pub fn build(&self) -> LogSessionCreate<'a> {
        LogSessionCreate {
            app_id: self.app_id,
            params: LogSessionCreateParams {
                dyno: self.params.dyno,
                lines: self.params.lines,
                source: self.params.source,
                tail: self.params.tail,
            },
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
        Some(self.params.clone())
    }
}
