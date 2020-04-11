//Anything related to GET requests for heroku logs and it's properties goes here.
use super::LogDrain;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Log Drain List
///
/// List existing log drains.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#log-drain-list)
pub struct LogDrainList<'a> {
    /// unique app identifier, either app name, or app id
    pub app_id: &'a str,
}

impl<'a> LogDrainList<'a> {
    pub fn new(app_id: &'a str) -> LogDrainList {
        LogDrainList { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<LogDrain>> for LogDrainList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/log-drains", self.app_id)
    }
}

/// Log Drain Info
///
/// Info for existing log drain.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#log-drain-info)
pub struct LogDrainDetails<'a> {
    /// unique app identifier, either app name, or app id
    pub app_id: &'a str,
    /// unique log drain identifier, either drain id, url or token
    pub drain_id: &'a str,
}

impl<'a> LogDrainDetails<'a> {
    pub fn new(app_id: &'a str, drain_id: &'a str) -> LogDrainDetails<'a> {
        LogDrainDetails { app_id, drain_id }
    }
}

impl<'a> HerokuEndpoint<LogDrain> for LogDrainDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/log-drains/{}", self.app_id, self.drain_id)
    }
}

/// Log Drain List By Add-on
///
/// List existing log drains for an add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#log-drain-list-by-add-on)
pub struct LogDrainListByAddon<'a> {
    /// unique addon identifier
    pub addon_id: &'a str,
}

impl<'a> LogDrainListByAddon<'a> {
    pub fn new(addon_id: &'a str) -> LogDrainListByAddon<'a> {
        LogDrainListByAddon { addon_id }
    }
}

impl<'a> HerokuEndpoint<LogDrain> for LogDrainListByAddon<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("addons/{}/log-drains", self.addon_id)
    }
}
