use crate::framework::response::ApiResult;
use serde::Deserialize;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use delete::{DynoAllRestart, DynoRestart};
pub use get::{DynoDetails, DynoList, DynoSizeDetails, DynoSizeList};
pub use post::{DynoActionStop, DynoCreate, DynoCreateParams};

impl ApiResult for Dyno {}
impl ApiResult for Vec<Dyno> {}

impl ApiResult for DynoSize {}
impl ApiResult for Vec<DynoSize> {}

pub use dyno_size::DynoSize;
/// Heroku Dyno
///
/// Stability: production
///
/// Dynos encapsulate running processes of an app on Heroku
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#dyno)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Dyno {
    /// An app represents the program that you would like to deploy and run on Heroku.
    pub app: App,
    /// a URL to stream output from for attached processes or null for non-attached processes
    pub attach_url: Option<String>,
    /// command used to start this process
    pub command: String,
    /// when dyno was created
    pub created_at: String,
    /// unique identifier of this dyno
    pub id: String,
    /// the name of this process on this dyno
    pub name: String,
    /// A release represents a combination of code, config vars and add-ons for an app on Heroku.
    pub release: Release,
    /// dyno size (default: “standard-1X”)
    pub size: String,
    /// current status of process (either: crashed, down, idle, starting, or up)
    pub state: String,
    /// type of process
    pub r#type: String, //type is a keyword in Rust
    /// when process last changed state
    pub updated_at: String,
}

/// An app represents the program that you would like to deploy and run on Heroku.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct App {
    /// unique identifier
    pub id: String,
    /// name of app pattern: ^[a-z][a-z0-9-]{1,28}[a-z0-9]$
    pub name: String,
}

/// A release represents a combination of code, config vars and add-ons for an app on Heroku.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Release {
    /// unique identifier of release
    pub id: String,
    /// unique version assigned to the release
    pub version: i64,
}

mod dyno_size {
    /// Heroku Dyno Size
    ///
    /// Stability: prototype
    ///
    /// Dyno sizes are the values and details of sizes that can be assigned to dynos. This information can also be found [here](https://devcenter.heroku.com/articles/dyno-types)
    ///
    /// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#dyno-size)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct DynoSize {
        /// minimum vCPUs, non-dedicated may get more depending on load
        pub compute: i64,
        /// price information for this dyno size
        pub cost: Option<Cost>,
        /// whether this dyno will be dedicated to one user
        pub dedicated: bool,
        /// unit of consumption for Heroku Enterprise customers
        pub dyno_units: i64,
        /// unique identifier of this dyno size
        pub id: String,
        /// amount of RAM in GB
        pub memory: f64,
        /// the name of this dyno-size
        pub name: String,
        /// whether this dyno can only be provisioned in a private space
        pub private_space_only: bool,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Cost {
        pub cents: Option<i64>,
        pub unit: Option<String>,
    }
}
