//Anything related to PUT requests for build and it's properties goes here.
use super::{BuildpackInstallation, Update};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Buildpack Installations Update
///
/// Update an app’s buildpack installations.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#buildpack-installations-update)
pub struct BuildpackInstallationUpdate<'a> {
    /// app_id can be the app name or id.
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: BuildpackInstallationUpdateParams,
}

#[cfg(feature = "builder")]
impl<'a> BuildpackInstallationUpdate<'a> {
    pub fn new(app_id: &'a str, buildpacks: Vec<&'a str>) -> BuildpackInstallationUpdate<'a> {
        let mut updates = Vec::new();
        for var in buildpacks {
            updates.push(Update {
                buildpack: var.to_owned(),
            });
        }

        BuildpackInstallationUpdate {
            app_id,
            params: BuildpackInstallationUpdateParams { updates },
        }
    }
}

/// Update an app’s buildpack installations.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#buildpack-installations-update-required-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct BuildpackInstallationUpdateParams {
    /// The buildpack attribute can accept a name, a url, or a urn.
    pub updates: Vec<Update>,
}

impl<'a> HerokuEndpoint<Vec<BuildpackInstallation>, (), BuildpackInstallationUpdateParams>
    for BuildpackInstallationUpdate<'a>
{
    fn method(&self) -> Method {
        Method::Put
    }
    fn path(&self) -> String {
        format!("apps/{}/buildpack-installations", self.app_id)
    }
    fn body(&self) -> Option<BuildpackInstallationUpdateParams> {
        Some(self.params.clone())
    }
}
