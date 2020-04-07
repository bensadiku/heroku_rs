//Anything related to GET requests for Addons and it's variations goes here.
use super::Addon;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Add-on Info
///
/// Info for an existing add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-info)
pub struct AddonDetails<'a> {
    pub addon_id: &'a str,
}

impl<'a> AddonDetails<'a> {
    pub fn new(addon_id: &'a str) -> AddonDetails {
        AddonDetails { addon_id }
    }
}

impl<'a> HerokuEndpoint<Addon> for AddonDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("addons/{}", self.addon_id)
    }
}

/// Add-on List
///
/// List all existing add-ons.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-list)
pub struct AddonList {}

impl AddonList {
    pub fn new() -> AddonList {
        AddonList {}
    }
}

impl HerokuEndpoint<Vec<Addon>> for AddonList {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("addons")
    }
}

/// Add-on Info By App
///
/// Info for an existing add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-info-by-app)
pub struct AddonDetailsByApp<'a> {
    /// unique app identifier, either app name or app id
    pub app_id: &'a str,
    /// unique add-on identifier, either add-on id or add-on name
    pub addon_id: &'a str,
}

impl<'a> AddonDetailsByApp<'a> {
    pub fn new(app_id: &'a str, addon_id: &'a str) -> AddonDetailsByApp<'a> {
        AddonDetailsByApp { app_id, addon_id }
    }
}

impl<'a> HerokuEndpoint<Addon> for AddonDetailsByApp<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/addons/{}", self.app_id, self.addon_id)
    }
}

/// Add-on List By App
///
/// List existing add-ons for an app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-list-by-app)
pub struct AddonListByApp<'a> {
    /// unique app identifier, either app name or app id
    pub app_id: &'a str,
}

impl<'a> AddonListByApp<'a> {
    pub fn new(app_id: &'a str) -> AddonListByApp<'a> {
        AddonListByApp { app_id }
    }
}

impl<'a> HerokuEndpoint<Vec<Addon>> for AddonListByApp<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/addons", self.app_id)
    }
}

/// Add-on List By User
///
/// List all existing add-ons a user has access to
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-list-by-user)
pub struct AddonListByAccount<'a> {
    /// unique account identifier, either account email or account id
    pub account_id: &'a str,
}

impl<'a> AddonListByAccount<'a> {
    pub fn new(account_id: &'a str) -> AddonListByAccount<'a> {
        AddonListByAccount { account_id }
    }
}

impl<'a> HerokuEndpoint<Vec<Addon>> for AddonListByAccount<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("users/{}/addons", self.account_id)
    }
}

/// Add-on List By Team
///
/// List add-ons used across all Team apps
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-list-by-team)
pub struct AddonListByTeam<'a> {
    /// unique team identifier, either team name or team id
    pub team_id: &'a str,
}

impl<'a> AddonListByTeam<'a> {
    pub fn new(team_id: &'a str) -> AddonListByTeam<'a> {
        AddonListByTeam { team_id }
    }
}

impl<'a> HerokuEndpoint<Vec<Addon>> for AddonListByTeam<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("teams/{}/addons", self.team_id)
    }
}
