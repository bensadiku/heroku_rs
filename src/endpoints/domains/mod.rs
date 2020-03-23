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
/// 
/// Stability: production
/// 
/// Domains define what web routes should be routed to an app on Heroku.
/// 
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#domain)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Domain {
    /// status of this record’s ACM
    pub acm_status: Option<String>,
    /// reason for the status of this record’s ACM
    pub acm_status_reason: Option<String>,
    /// App
    pub app: App,
    /// canonical name record, the address to point a domain at
    pub cname: Option<String>,
    /// when domain was created
    pub created_at: String,
    /// full hostname of the domain
    pub hostname: String,
    /// unique identifier of this domain
    pub id: String,
    /// type of domain name. One of:"heroku" or "custom"
    pub kind: String,
    /// when domain was updated
    pub updated_at: String,
    /// status of this record’s cname
    pub status: String,
}

/// An app represents the program that you would like to deploy and run on Heroku.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct App {
    /// name of app. pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
    pub name: String,
    /// unique identifier
    pub id: String,
}
