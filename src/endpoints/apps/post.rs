//Anything related to creating apps and it's properties goes here.
use super::{App, AppSetup, AppWebhook, SNI, SSL};
use std::collections::HashMap;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// App Create
///
/// Create a new app.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-create)
pub struct AppCreate<'a> {
    /// The parameters to pass to the Heroku API
    pub params: AppCreateParams<'a>,
}

impl<'a> AppCreate<'a> {
    /// Create a new Heroku app without parameters
    pub fn new() -> AppCreate<'a> {
        AppCreate {
            params: AppCreateParams {
                name: None,
                region: None,
                stack: None,
            },
        }
    }

    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.params.name = Some(name);
        self
    }

    pub fn region(&mut self, region: &'a str) -> &mut Self {
        self.params.region = Some(region);
        self
    }

    pub fn stack(&mut self, stack: &'a str) -> &mut Self {
        self.params.stack = Some(stack);
        self
    }

    pub fn build(&self) -> AppCreate<'a> {
        AppCreate {
            params: AppCreateParams {
                name: self.params.name,
                region: self.params.region,
                stack: self.params.stack,
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
pub struct AppCreateParams<'a> {
    /// name of app. pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
    pub name: Option<&'a str>,
    /// unique identifier or name of region
    pub region: Option<&'a str>,
    /// unique name or identifier of stack
    pub stack: Option<&'a str>,
}

impl<'a> HerokuEndpoint<App, (), AppCreateParams<'a>> for AppCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps")
    }
    fn body(&self) -> Option<AppCreateParams<'a>> {
        Some(self.params.clone())
    }
}

/// App Enable ACM
///
/// Enable ACM flag for an app
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-enable-acm)
pub struct AppEnableAcm<'a> {
    /// app_id can be the app id or name.
    pub app_id: &'a str,
}

impl<'a> AppEnableAcm<'a> {
    pub fn new(app_id: &'a str) -> AppEnableAcm<'a> {
        AppEnableAcm { app_id }
    }
}

impl<'a> HerokuEndpoint<App> for AppEnableAcm<'a> {
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
pub struct AppWebhookCreate<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: AppWebhookCreateParams<'a>,
}

impl<'a> AppWebhookCreate<'a> {
    /// Create a new webhook without optional parameters
    pub fn new(
        app_id: &'a str,
        include: Vec<&'a str>,
        level: &'a str,
        url: &'a str,
    ) -> AppWebhookCreate<'a> {
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

    pub fn authorization(&mut self, authorization: &'a str) -> &mut Self {
        self.params.authorization = Some(authorization);
        self
    }

    pub fn secret(&mut self, secret: &'a str) -> &mut Self {
        self.params.secret = Some(secret);
        self
    }

    pub fn build(&self) -> AppWebhookCreate<'a> {
        AppWebhookCreate {
            app_id: self.app_id,
            params: AppWebhookCreateParams {
                authorization: self.params.authorization,
                include: self.params.include.clone(),
                level: self.params.level,
                secret: self.params.secret,
                url: self.params.url,
            },
        }
    }
}

/// Create a new app webhook with parameters.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#app-webhook-create-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct AppWebhookCreateParams<'a> {
    /// A custom Authorization header that Heroku will include with all webhook notifications
    pub authorization: Option<&'a str>,
    /// The entities that the subscription provides notifications for
    pub include: Vec<&'a str>,
    /// One of: "notify" or "sync"
    /// If notify, Heroku makes a single, fire-and-forget delivery attempt. If sync, Heroku attempts multiple deliveries until the request is successful or a limit is reached
    pub level: &'a str,
    /// A value that Heroku will use to sign all webhook notification requests (the signature is included in the request’s Heroku-Webhook-Hmac-SHA256 header)
    pub secret: Option<&'a str>,
    /// The URL where the webhook’s notification requests are sent
    pub url: &'a str,
}

impl<'a> HerokuEndpoint<AppWebhook, (), AppWebhookCreateParams<'a>> for AppWebhookCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/webhooks", self.app_id)
    }
    fn body(&self) -> Option<AppWebhookCreateParams<'a>> {
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
    /// Create a new setup app with required parameters only
    pub fn new(url: &'a str) -> AppSetupCreate<'a> {
        AppSetupCreate {
            params: AppSetupCreateParams {
                app: SetupApp {
                    locked: None,
                    name: None,
                    organization: None,
                    personal: None,
                    region: None,
                    space: None,
                    stack: None,
                },
                source_blob: SourceBlob {
                    checksum: None,
                    url: url,
                    version: None,
                },
                overrides: Overrides {
                    buildpacks: None,
                    env: None,
                },
            },
        }
    }

    pub fn version(&mut self, version: &'a str) -> &mut Self {
        self.params.source_blob.version = Some(version);
        self
    }
    pub fn checksum(&mut self, checksum: &'a str) -> &mut Self {
        self.params.source_blob.checksum = Some(checksum);
        self
    }

    pub fn locked(&mut self, locked: bool) -> &mut Self {
        self.params.app.locked = Some(locked);
        self
    }
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.params.app.name = Some(name);
        self
    }
    pub fn organization(&mut self, organization: &'a str) -> &mut Self {
        self.params.app.organization = Some(organization);
        self
    }
    pub fn personal(&mut self, personal: bool) -> &mut Self {
        self.params.app.personal = Some(personal);
        self
    }
    pub fn region(&mut self, region: &'a str) -> &mut Self {
        self.params.app.region = Some(region);
        self
    }
    pub fn space(&mut self, space: &'a str) -> &mut Self {
        self.params.app.space = Some(space);
        self
    }
    pub fn stack(&mut self, stack: &'a str) -> &mut Self {
        self.params.app.stack = Some(stack);
        self
    }

    pub fn buildpacks(&mut self, buildpacks_list: Vec<&'a str>) -> &mut Self {
        let mut buildpacks: Vec<Buildpack> = Vec::new();
        for var in buildpacks_list {
            buildpacks.push(Buildpack { url: var });
        }
        self.params.overrides.buildpacks = Some(buildpacks);
        self
    }
    pub fn env(&mut self, env: HashMap<&'a str, &'a str>) -> &mut Self {
        self.params.overrides.env = Some(env);
        self
    }
    /// Create a new Heroku app with required  and optional parameters
    pub fn build(&self) -> AppSetupCreate<'a> {
        AppSetupCreate {
            params: AppSetupCreateParams {
                app: self.params.app.clone(),
                source_blob: self.params.source_blob.clone(),
                overrides: self.params.overrides.clone(),
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
    pub app: SetupApp<'a>,
    pub source_blob: SourceBlob<'a>,
    pub overrides: Overrides<'a>,
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

impl<'a> SNICreate<'a> {
    /// Create a new Heroku app SNI with parameters
    pub fn new(app_id: &'a str, certificate_chain: &'a str, private_key: &'a str) -> SNICreate<'a> {
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

impl<'a> HerokuEndpoint<SNI, (), SNICreateParams<'a>> for SNICreate<'a> {
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

/// SSL Endpoint Create
///
/// Create a new SSL endpoint.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#ssl-endpoint-create)
pub struct SSLCreate<'a> {
    /// unique app identifier, either app id or app name
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: SSLCreateParams<'a>,
}

impl<'a> SSLCreate<'a> {
    /// Update Heroku app SSL with parameters
    pub fn new(
        app_id: &'a str,
        certificate_chain: &'a str,
        private_key: &'a str,
    ) -> SSLCreate<'a> {
        SSLCreate {
            app_id,
            params: SSLCreateParams {
                certificate_chain: certificate_chain,
                private_key: private_key,
                preprocess: None,
            },
        }
    }

    pub fn preprocess(&mut self, preprocess: bool) -> &mut Self {
        self.params.preprocess = Some(preprocess);
        self
    }
    pub fn build(&self) -> SSLCreate<'a> {
        SSLCreate {
            app_id: self.app_id,
            params: SSLCreateParams {
                certificate_chain: self.params.certificate_chain,
                private_key: self.params.private_key,
                preprocess: self.params.preprocess,
            },
        }
    }
}

/// Create a new app ssl endpoint with parameters.
///
/// [See Heroku documentation for more information about this endpoint](hhttps://devcenter.heroku.com/articles/platform-api-reference#ssl-endpoint-create-required-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct SSLCreateParams<'a> {
    /// raw contents of the public certificate chain (eg: .crt or .pem file)
    pub certificate_chain: &'a str,
    /// contents of the private key (eg .key file)
    pub private_key: &'a str,
    /// allow Heroku to modify an uploaded public certificate chain if deemed advantageous by adding missing intermediaries, stripping unnecessary ones, etc.
    ///  default: true
    pub preprocess: Option<bool>,
}

impl<'a> HerokuEndpoint<SSL, (), SSLCreateParams<'a>> for SSLCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/ssl-endpoints", self.app_id)
    }
    fn body(&self) -> Option<SSLCreateParams<'a>> {
        Some(self.params.clone())
    }
}
