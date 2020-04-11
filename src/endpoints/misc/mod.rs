use crate::framework::response::ApiResult;

pub mod delete;
pub mod get;
pub mod patch;
pub mod post;
pub mod put;

pub use get::{RatelimitDetails, RegionDetails, RegionList, StackDetails, StackList};
pub use post::SourceCreate;

pub use ratelimit::Ratelimit;
pub use region::Region;
pub use sources::SourceBlob;
pub use stack::Stack;

impl ApiResult for Region {}
impl ApiResult for Vec<Region> {}

impl ApiResult for Ratelimit {}

impl ApiResult for Stack {}
impl ApiResult for Vec<Stack> {}

impl ApiResult for SourceBlob {}

mod region {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Region
    ///
    /// Stability: production
    ///
    /// A region represents a geographic location in which your application may run.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#region)
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Region {
        /// country where the region exists
        pub country: String,
        /// when region was created
        pub created_at: DateTime<Utc>,
        /// description of region
        pub description: String,
        /// unique identifier
        pub id: String,
        /// area in the country where the region exists
        pub locale: String,
        /// name of region
        pub name: String,
        /// whether or not region is available for creating a Private Space
        pub private_capable: bool,
        /// provider
        pub provider: Provider,
        /// when region was updated
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Provider {
        /// name of provider
        pub name: String,
        /// region name used by provider
        /// one of:"ap-south-1" or "eu-west-1" or "ap-southeast-1" or "ap-southeast-2" or "eu-central-1" or "ap-northeast-2" or "ap-northeast-1" or "us-east-1" or "sa-east-1" or "us-west-1" or "us-west-2"
        pub region: String,
    }
}

mod ratelimit {
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Ratelimit {
        pub remaining: i64,
    }
}

mod stack {
    use chrono::offset::Utc;
    use chrono::DateTime;

    /// Stack
    ///
    /// Stability: production
    ///
    /// Stacks are the different application execution environments available in the Heroku platform.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#stack)

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    pub struct Stack {
        /// indicates this stack is the default for new apps
        pub default: bool,
        /// when stack was introduced
        pub created_at: DateTime<Utc>,
        /// identifier of stack
        pub id: String,
        /// unique name
        pub name: String,
        /// availability of this stack: beta, deprecated or public
        pub state: String,
        /// when stack was last modified
        pub updated_at: DateTime<Utc>,
    }
}

mod sources {
    /// Source
    ///
    /// Stability: production
    ///
    /// A source is a location for uploading and downloading an application’s source code.
    ///
    /// [For more information please refer to the Heroku documentation](https://devcenter.heroku.com/articles/platform-api-reference#source)
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct SourceBlob {
        /// the urls which you can download or upload the source
        pub source_blob: SourceBlobData,
    }
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct SourceBlobData {
        /// URL to download the source
        pub get_url: String,
        /// URL to upload the source
        pub put_url: String,
    }
}
