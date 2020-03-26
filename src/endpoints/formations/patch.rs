//Anything related to PATCH requests for formations and it's properties goes here.

use super::{Formation};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Formation Update
///
/// Update process type
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#formation-update)
pub struct FormationUpdate {
    /// app_id can be the app name or the app id
    pub app_id: String,
    /// formation_id can be the formation id or type
    pub formation_id: String,
    /// params are the parameters sent to the API to patch the Formation
    pub params: FormationUpdateParams,
}

/// Update formation with parameters
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#formation-update-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct FormationUpdateParams {
    /// number of processes to maintain
    pub quantity: Option<i32>,
    /// dyno size
    pub size: Option<String>
}

impl HerokuEndpoint<Formation, (), FormationUpdateParams> for FormationUpdate {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("apps/{}/formation/{}", self.app_id, self.formation_id)
    }
    fn body(&self) -> Option<FormationUpdateParams> {
        Some(self.params.clone())
    }
}