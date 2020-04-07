//Anything related to DELETE requests for Addons and it's variations goes here.
use super::Addon;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Add-on Delete
///
/// Delete an existing add-on.
///
/// [See Heroku documentation for more information about this endpoint](hhttps://devcenter.heroku.com/articles/platform-api-reference#add-on-delete)
pub struct AddonDelete<'a> {
    /// unique app identifier, either app name or app id
    pub app_id: &'a str,
    /// unique add-on identifier, either add-on id or add-on name
    pub addon_id: &'a str,
}

impl<'a> AddonDelete<'a> {
    /// Delete addon
    pub fn new(app_id: &'a str, addon_id: &'a str) -> AddonDelete<'a> {
        AddonDelete { app_id, addon_id }
    }
}

impl<'a> HerokuEndpoint<Addon> for AddonDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("apps/{}/addons/{}", self.app_id, self.addon_id)
    }
}
