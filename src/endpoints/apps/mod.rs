use crate::framework::response::ApiResult;
use serde::Deserialize;

pub mod apps;
pub use apps::{AppCreate, AppCreateParams, AppDelete, AppDetails, AppList};

impl ApiResult for App {}
impl ApiResult for Vec<App> {}


/// Heroku App
/// An app represents the program that you would like to deploy and run on Heroku.
/// https://devcenter.heroku.com/articles/platform-api-reference#app
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct App {
    pub acm: bool,
    pub archived_at: Option<String>,
    pub buildpack_provided_description: Option<String>,
    pub build_stack: BuildStack,
    pub created_at: String,
    pub git_url: String,
    pub id: String,
    pub internal_routing: Option<bool>,
    pub maintenance: bool,
    pub name: String,
    pub owner: Owner,
    pub organization: Option<Organization>,
    pub team: Option<Team>,
    pub region: Region,
    pub released_at: Option<String>,
    pub repo_size: Option<i64>,
    pub slug_size: Option<i64>,
    pub space: Option<Space>,
    pub stack: Stack,
    pub updated_at: String,
    pub web_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct BuildStack {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Owner {
    pub email: String,
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Organization {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Team {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Region {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Space {
    pub id: String,
    pub name: String,
    pub shield: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Stack {
    pub id: String,
    pub name: String,
}
