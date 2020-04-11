//Anything related to PUT requests for heroku logs and it's properties goes here.
use super::LogDrain;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Log Drain Update
///
/// Update an add-on owned log drain.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#log-drain-update)
pub struct LogDrainUpdate<'a> {
    /// unique addon identifier
    pub addon_id: &'a str,
    /// unique drain identifier, either drain id, url or token
    pub drain_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: LogDrainUpdateParams<'a>,
}

impl<'a> LogDrainUpdate<'a> {
    pub fn new(addon_id: &'a str, drain_id: &'a str, url: &'a str) -> LogDrainUpdate<'a> {
        LogDrainUpdate {
            addon_id,
            drain_id,
            params: LogDrainUpdateParams { url },
        }
    }
}

/// Update an exisiton add-on owned log drain with parameters.
///
/// [See Heroku documentation for more information about these paramters](https://devcenter.heroku.com/articles/platform-api-reference#log-drain-update-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct LogDrainUpdateParams<'a> {
    /// url associated with the log drain
    pub url: &'a str,
}

impl<'a> HerokuEndpoint<LogDrain, (), LogDrainUpdateParams<'a>> for LogDrainUpdate<'a> {
    fn method(&self) -> Method {
        Method::Put
    }
    fn path(&self) -> String {
        format!("addons/{}/log-drains/{}", self.addon_id, self.drain_id)
    }
    fn body(&self) -> Option<LogDrainUpdateParams<'a>> {
        Some(self.params.clone())
    }
}
