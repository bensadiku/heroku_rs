//Anything related to POST requests for Addons and it's variations goes here.
use super::Addon;
use crate::framework::endpoint::{HerokuEndpoint, Method};
use std::collections::HashMap;

/// Add-on Create
///
/// Create a new add-on.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-create)
pub struct AddonCreate<'a> {
    pub app_id: &'a str,
    params: AddonCreateParams<'a>,
}

impl<'a> AddonCreate<'a> {
    /// Create a new addon with required and optional parameters
    pub fn new(
        app_id: &'a str,
        plan: &'a str,
        attachment_name: Option<&'a str>,
        config: Option<HashMap<&'a str, &'a str>>,
        confirm: Option<&'a str>,
        name: Option<&'a str>,
    ) -> AddonCreate<'a> {
        AddonCreate {
            app_id,
            params: AddonCreateParams {
                attachment: Some(Attachment {
                    name: attachment_name,
                }),
                config: config,
                plan: plan,
                confirm: confirm,
                name: name,
            },
        }
    }

    /// Create a new addon without required parameters only
    pub fn create(app_id: &'a str, plan: &'a str) -> AddonCreate<'a> {
        AddonCreate {
            app_id,
            params: AddonCreateParams {
                attachment: None,
                config: None,
                plan: plan,
                confirm: None,
                name: None,
            },
        }
    }
}

/// Create add-on with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-create-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct AddonCreateParams<'a> {
    /// unique name for this add-on attachment to this app
    pub attachment: Option<Attachment<'a>>,
    /// custom add-on provisioning options
    pub config: Option<HashMap<&'a str, &'a str>>,
    /// name of billing entity for confirmation
    pub confirm: Option<&'a str>,
    /// unique identifier or name of this plan
    pub plan: &'a str,
    /// globally unique name of the add-on
    ///  pattern: ^[a-zA-Z][A-Za-z0-9_-]+$
    pub name: Option<&'a str>,
}

#[derive(Serialize, Clone, Debug)]
pub struct Attachment<'a> {
    /// unique name for this add-on attachment to this app
    pub name: Option<&'a str>,
}

impl<'a> HerokuEndpoint<Addon, (), AddonCreateParams<'a>> for AddonCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/addons", self.app_id)
    }
    fn body(&self) -> Option<AddonCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// Add-on Resolution
///
/// Resolve an add-on from a name, optionally passing an app name. If there are matches it returns at least one add-on (exact match) or many.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-resolution)
pub struct AddonResolutionCreate<'a> {
    /// parameters to pass to the Heroku API
    pub params: AddonResolutionCreateParams<'a>,
}

impl<'a> AddonResolutionCreate<'a> {
    /// Create a new addon with required and optional parameters
    pub fn new(
        addon: &'a str,
        addon_service: Option<&'a str>,
        app: Option<&'a str>,
    ) -> AddonResolutionCreate<'a> {
        AddonResolutionCreate {
            params: AddonResolutionCreateParams {
                addon,
                addon_service,
                app,
            },
        }
    }

    /// Create a new addon resolution without optional parameters
    pub fn create(addon: &'a str) -> AddonResolutionCreate<'a> {
        AddonResolutionCreate {
            params: AddonResolutionCreateParams {
                addon: addon,
                addon_service: None,
                app: None,
            },
        }
    }
}

/// Create add-on resolution with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#add-on-resolution-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct AddonResolutionCreateParams<'a> {
    /// globally unique name of the add-on
    ///  pattern: ^[a-zA-Z][A-Za-z0-9_-]+$
    pub addon: &'a str,
    /// unique name of this add-on-service
    pub addon_service: Option<&'a str>,
    /// unique name of app
    ///  pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
    pub app: Option<&'a str>,
}

impl<'a> HerokuEndpoint<Vec<Addon>, (), AddonResolutionCreateParams<'a>> for AddonResolutionCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("actions/addons/resolve")
    }
    fn body(&self) -> Option<AddonResolutionCreateParams<'a>> {
        Some(self.params.clone())
    }
}
