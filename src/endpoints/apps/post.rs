//Anything related to creating apps and it's properties goes here.
use super::{App, AppSetup, AppWebhook,SNI};
use std::collections::HashMap;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// App Create
///
/// Create a new app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-create)
pub struct AppCreate {
    /// The parameters to pass to the Heroku API
    pub params: AppCreateParams,
}

impl AppCreate {
    /// Create a new Heroku app with parameters
    pub fn new(name: Option<String>, region: Option<String>, stack: Option<String>) -> AppCreate {
        AppCreate {
            params: AppCreateParams {
                name,
                region,
                stack,
            },
        }
    }

    /// Create a new Heroku app without parameters
    pub fn create() -> AppCreate {
        AppCreate {
            params: AppCreateParams {
                name: None,
                region: None,
                stack: None,
            },
        }
    }
}

/// Create a new app with parameters.
///
/// All three paramemters are optional.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-create-optional-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct AppCreateParams {
    /// name of app. pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
    pub name: Option<String>,
    /// unique identifier or name of region
    pub region: Option<String>,
    /// unique name or identifier of stack
    pub stack: Option<String>,
}

impl HerokuEndpoint<App, (), AppCreateParams> for AppCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps")
    }
    fn body(&self) -> Option<AppCreateParams> {
        Some(self.params.clone())
    }
}

/// App Enable ACM
///
/// Enable ACM flag for an app
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-enable-acm)
pub struct AppEnableAcm {
    /// app_id can be the app id or name.
    pub app_id: String,
}

impl AppEnableAcm {
    pub fn new(app_id: String) -> AppEnableAcm {
        AppEnableAcm { app_id }
    }
}

impl HerokuEndpoint<App> for AppEnableAcm {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/acm", self.app_id)
    }
}

/// App Webhook Create
///
/// Create an app webhook subscription.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-create)
pub struct AppWebhookCreate {
    /// app_id can be the app name or the app id
    pub app_id: String,
    /// The parameters to pass to the Heroku API
    pub params: AppWebhookCreateParams,
}

impl AppWebhookCreate {
    /// Create a new webhook with optional parameters
    pub fn new(
        app_id: String,
        authorization: Option<String>,
        include: Vec<String>,
        level: String,
        secret: Option<String>,
        url: String,
    ) -> AppWebhookCreate {
        AppWebhookCreate {
            app_id,
            params: AppWebhookCreateParams {
                authorization,
                include,
                level,
                secret,
                url,
            },
        }
    }
    /// Create a new webhook without optional parameters
    pub fn create(
        app_id: String,
        include: Vec<String>,
        level: String,
        url: String,
    ) -> AppWebhookCreate {
        AppWebhookCreate {
            app_id: app_id,
            params: AppWebhookCreateParams {
                authorization: None,
                include: include,
                level: level,
                secret: None,
                url: url,
            },
        }
    }
}

/// Create a new app webhook with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct AppWebhookCreateParams {
    /// A custom Authorization header that Heroku will include with all webhook notifications
    pub authorization: Option<String>,
    /// The entities that the subscription provides notifications for
    pub include: Vec<String>,
    /// One of: "notify" or "sync"
    /// If notify, Heroku makes a single, fire-and-forget delivery attempt. If sync, Heroku attempts multiple deliveries until the request is successful or a limit is reached
    pub level: String,
    /// A value that Heroku will use to sign all webhook notification requests (the signature is included in the request’s Heroku-Webhook-Hmac-SHA256 header)
    pub secret: Option<String>,
    /// The URL where the webhook’s notification requests are sent
    pub url: String,
}

impl HerokuEndpoint<AppWebhook, (), AppWebhookCreateParams> for AppWebhookCreate {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/webhooks", self.app_id)
    }
    fn body(&self) -> Option<AppWebhookCreateParams> {
        Some(self.params.clone())
    }
}

/// App Setup Create
///
/// Create a new app setup from a gzipped tar archive containing an app.json manifest file.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-setup-create)
pub struct AppSetupCreate<'a> {
    /// The parameters to pass to the Heroku API
    pub params: AppSetupCreateParams<'a>,
}

impl<'a> AppSetupCreate<'a> {
    /// Create a new Heroku app with required  and optional parameters
    // :| needs a better solution. Builder pattern?
    pub fn new(
        locked: Option<bool>,
        name: Option<&'a str>,
        organization: Option<&'a str>,
        personal: Option<bool>,
        region: Option<&'a str>,
        space: Option<&'a str>,
        stack: Option<&'a str>,
        checksum: Option<&'a str>,
        url: &'a str,
        version: Option<&'a str>,
        buildpacks_list: Option<Vec<&'a str>>,
        env: Option<HashMap<&'a str, &'a str>>,
    ) -> AppSetupCreate<'a> {
        let buildpacks: Option<Vec<Buildpack>> = match buildpacks_list {
            Some(buidpacks) => {
                let mut buildpacks: Vec<Buildpack> = Vec::new();
                for var in buidpacks {
                    buildpacks.push(Buildpack { url: var });
                }
                Some(buildpacks)
            }
            None => None,
        };
        AppSetupCreate {
            params: AppSetupCreateParams {
                app: Some(SetupApp {
                    locked,
                    name,
                    organization,
                    personal,
                    region,
                    space,
                    stack,
                }),
                source_blob: SourceBlob {
                    checksum,
                    url,
                    version,
                },
                overrides: Some(Overrides { buildpacks, env }),
            },
        }
    }

    /// Create a new setup app with required parameters only
    pub fn create(
        checksum: Option<&'a str>,
        url: &'a str,
        version: Option<&'a str>,
    ) -> AppSetupCreate<'a> {
        AppSetupCreate {
            params: AppSetupCreateParams {
                app: None,
                source_blob: SourceBlob {
                    checksum,
                    url,
                    version,
                },
                overrides: None,
            },
        }
    }
}

/// Create a new  setup app with parameters.
///
/// All three papparamemters are optional.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-setup-create-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct AppSetupCreateParams<'a> {
    pub app: Option<SetupApp<'a>>,
    pub source_blob: SourceBlob<'a>,
    pub overrides: Option<Overrides<'a>>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct SetupApp<'a> {
    /// are other team members forbidden from joining this app.
    pub locked: Option<bool>,
    /// name of app
    ///  pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$ 
    pub name: Option<&'a str>,
    /// unique name of team
    pub organization: Option<&'a str>,
    /// force creation of the app in the user account even if a default team is set.
    pub personal: Option<bool>,
    /// name of region
    pub region: Option<&'a str>,
    /// unique name of space
    ///  pattern: `^[a-z0-9](?:[a-z0-9]
    pub space: Option<&'a str>,
    /// unique name
    pub stack: Option<&'a str>,
}

#[derive(Serialize, Clone, Debug)]
pub struct SourceBlob<'a> {
    /// an optional checksum of the gzipped tarball for verifying its integrity. [Nullable]
    pub checksum: Option<&'a str>,
    /// URL of gzipped tarball of source code containing app.json manifest file.
    pub url: &'a str,
    /// Version of the gzipped tarball. [Nullable]
    pub version: Option<&'a str>, 
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct Overrides<'a> {
    /// overrides the buildpacks specified in the app.json manifest file
    pub buildpacks: Option<Vec<Buildpack<'a>>>,
    /// overrides of the env specified in the app.json manifest file
    pub env: Option<HashMap<&'a str, &'a str>>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct Buildpack<'a> {
    pub url: &'a str,
}

impl<'a> HerokuEndpoint<AppSetup, (), AppSetupCreateParams<'a>> for AppSetupCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("app-setups")
    }
    fn body(&self) -> Option<AppSetupCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// SNI Endpoint Create
///
/// Create a new SNI endpoint.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#sni-endpoint-create)
pub struct SNICreate<'a> {
    /// unique app identifier, either app id or app name
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: SNICreateParams<'a>,
}

impl <'a>SNICreate <'a>{
    /// Create a new Heroku app SNI with parameters
    pub fn new(app_id: &'a str,certificate_chain: &'a str, private_key: &'a str) -> SNICreate<'a> {
        SNICreate {
            app_id,
            params: SNICreateParams {
                certificate_chain,
                private_key,
            },
        }
    }
}

/// Create a new app sni with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#sni-endpoint-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct SNICreateParams<'a> {
    /// raw contents of the public certificate chain (eg: .crt or .pem file)
    pub certificate_chain: &'a str,
    /// contents of the private key (eg .key file)
    pub private_key: &'a str,
}

impl <'a>HerokuEndpoint<SNI, (), SNICreateParams<'a>> for SNICreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/sni-endpoints", self.app_id)
    }
    fn body(&self) -> Option<SNICreateParams<'a>> {
        Some(self.params.clone())
    }
}