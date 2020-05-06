use crate::framework::response::ApiResult;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use post::{SpaceCreate, SpaceCreateParams};
pub use patch::{SpaceUpdate, SpaceUpdateParams};
pub use get::{SpaceDetails, SpaceList};

impl ApiResult for Space {}
impl ApiResult for Vec<Space> {}

pub use spaces::Space;

mod spaces {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Space
    ///
    /// Stability: prototype
    ///
    /// A space is an isolated, highly available, secure app execution environments, running in the modern VPC substrate.
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#space-1)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Space {
        /// when space was created
        pub created_at: DateTime<Utc>,
        /// unique identifier of space
        pub id: String,
        /// unique name of space
        ///  pattern: `^[a-z0-9](?:[a-z0-9]
        pub name: String,
        /// organization this space is in
        pub organization: Organization,
        /// team this space is in
        pub team: Team,
        /// region of this space
        pub region: Region,
        /// true if this space has shield enabled
        pub shield: bool,
        /// availability of this space
        ///  one of:"allocating" or "allocated" or "deleting"
        pub state: String,
        /// when space was updated
        pub updated_at: DateTime<Utc>,
        /// The RFC-1918 CIDR the Private Space will use. It must be a /16 in 10.0.0.0/8, 172.16.0.0/12 or 192.168.0.0/16
        ///  default: "10.0.0.0/16"
        ///  pattern: `^((?:10
        pub cidr: String,
        /// The RFC-1918 CIDR that the Private Space will use for the Heroku-managed peering connection that’s automatically created when using Heroku Data add-ons. It must be between a /16 and a /20
        pub data_cidr: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Organization {
        /// unique name of team
        pub name: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Team {
        /// unique identifier of team
        pub id: String,
        /// unique name of team
        pub name: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Region {
        /// unique identifier
        pub id: String,
        /// name of region
        pub name: String,
    }
}
