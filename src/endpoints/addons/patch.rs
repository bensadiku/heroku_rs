//Anything related to PATCH requests for Addons and it's variations goes here.
use super::Addon;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Add-on Update
///
/// Change add-on plan. Some add-ons may not support changing plans. In that case, an error will be returned.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-update)
pub struct AddonUpdate<'a> {
    /// unique app identifier, either app name or app id
    pub app_id: &'a str,
    /// unique add-on identifier, either add-on id or add-on name
    pub addon_id: &'a str,
    /// parameters to pass to the Heroku API
    pub params: AddonUpdateParams<'a>,
}

impl<'a> AddonUpdate<'a> {
    pub fn new(
        app_id: &'a str,
        addon_id: &'a str,
        plan: &'a str,
        name: Option<&'a str>,
    ) -> AddonUpdate<'a> {
        AddonUpdate {
            app_id,
            addon_id,
            params: AddonUpdateParams { plan, name },
        }
    }

    pub fn create(app_id: &'a str, addon_id: &'a str, plan: &'a str) -> AddonUpdate<'a> {
        AddonUpdate {
            app_id,
            addon_id,
            params: AddonUpdateParams {
                plan: plan,
                name: None,
            },
        }
    }
}

/// Update add-on with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-update-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct AddonUpdateParams<'a> {
    /// unique identifier or name of this plan
    pub plan: &'a str,
    /// globally unique name of the add-on
    ///  pattern: ^[a-zA-Z][A-Za-z0-9_-]+$
    pub name: Option<&'a str>,
}

impl<'a> HerokuEndpoint<Addon, (), AddonUpdateParams<'a>> for AddonUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("apps/{}/addons/{}", self.app_id, self.addon_id)
    }
    fn body(&self) -> Option<AddonUpdateParams<'a>> {
        Some(self.params.clone())
    }
}
