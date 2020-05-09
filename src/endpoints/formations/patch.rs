//Anything related to PATCH requests for formations and it's properties goes here.

use super::Formation;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Formation Update
///
/// Update process type
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#formation-update)
pub struct FormationUpdate<'a> {
    /// app_id can be the app name or the app id
    pub app_id: &'a str,
    /// formation_id can be the formation id or type
    pub formation_id: &'a str,
    /// params are the parameters sent to the API to patch the Formation
    pub params: FormationUpdateParams<'a>,
}

impl<'a> FormationUpdate<'a> {
    pub fn new(app_id: &'a str, formation_id: &'a str) -> FormationUpdate<'a> {
        FormationUpdate {
            app_id,
            formation_id,
            params: FormationUpdateParams {
                quantity: None,
                size: None,
            },
        }
    }

    pub fn size(&mut self, size: &'a str) -> &mut Self {
        self.params.size = Some(size);
        self
    }

    pub fn quantity(&mut self, quantity: i32) -> &mut Self {
        self.params.quantity = Some(quantity);
        self
    }

    pub fn build(&self) -> FormationUpdate<'a> {
        FormationUpdate {
            app_id: self.app_id,
            formation_id: self.app_id,
            params: FormationUpdateParams {
                quantity: self.params.quantity,
                size: self.params.size,
            },
        }
    }
}

/// Update formation with parameters
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#formation-update-optional-parameters)
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct FormationUpdateParams<'a> {
    /// number of processes to maintain
    pub quantity: Option<i32>,
    /// dyno size
    pub size: Option<&'a str>,
}

impl<'a> HerokuEndpoint<Formation, (), FormationUpdateParams<'a>> for FormationUpdate<'a> {
    fn method(&self) -> Method {
        Method::Patch
    }
    fn path(&self) -> String {
        format!("apps/{}/formation/{}", self.app_id, self.formation_id)
    }
    fn body(&self) -> Option<FormationUpdateParams<'a>> {
        Some(self.params.clone())
    }
}
