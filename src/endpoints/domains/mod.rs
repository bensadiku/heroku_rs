use crate::framework::response::ApiResult;
use serde::Deserialize;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::DomainDelete;
pub use get::{DomainDetails, DomainList};
pub use post::{DomainCreate, DomainCreateParams};

impl ApiResult for Domain {}
impl ApiResult for Vec<Domain> {}

/// Domain
/// Stability: production
/// Domains define what web routes should be routed to an app on Heroku.
/// https://devcenter.heroku.com/articles/platform-api-reference#domain
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Domain {
    pub acm_status: Option<String>,
    pub acm_status_reason: Option<String>,
    pub app: App,
    pub cname: Option<String>,
    pub created_at: String,
    pub hostname: String,
    pub id: String,
    pub kind: String,
    pub updated_at: String,
    pub status: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct App {
    pub name: String,
    pub id: String,
}
