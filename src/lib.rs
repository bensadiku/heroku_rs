#![doc(html_root_url = "https://docs.rs/heroku_rs/0.5.1")]

//! # heroku_rs
//!
//! The `heroku_rs` crate provides convenient Rust bindings for the [Heroku V3 API][v3api], with a [`client`][client] built on top of [`Reqwest`][reqwest].
//!
//! ## Heroku Client
//!
//! Creating the Heroku client only takes 1 line. This client takes the API key and has a default 30 second http timeout, default http headers and points to the production Heroku API.
//!
//! #### Creating the Heroku Client
//!
//! ```rust
//! use heroku_rs::prelude::*;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let api_client = HttpApiClient::create("API_KEY")?;
//!   
//!    // You can start making requests here
//!    
//!    Ok(())
//! }
//! ```
//! If you want a more customized client, see the [client docs][client]
//!
//! ## Imports
//! 
//! heroku_rs has two major modules you'll use most. [Endpoints][endpoints] and [Framework][framework]
//! 
//! You can import both modules by using a global prelude import, which brings everything important into scope.
//! 
//! ```rust
//! use heroku_rs::prelude::*;
//! ```
//! Or you can import manually what you need if you're familiar with the crate structure.
//! 
//! e.g. importing space endpoints and the Heroku Client.
//! 
//! ```rust
//! use heroku_rs::endpoints::space;
//! use heroku_rs::framework::apiclient::HerokuApiClient;
//! ```
//!
//! ## API Structure
//!
//! This crate exposes a module [`endpoints`][endpoints] which gives you access to the specific Heroku endpoints you might need.
//!
//! Each endpoint on this crate has a unique struct with a consistent naming convention and builder pattern.
//!
//! ### - Naming Convention
//!
//!
//! The convention names are one of these: `List`, `Details`, `Update`, `Create`, `Delete`.
//!
//! The `List` structs are GET requests, used to return a list of the requested data. E.g. `AppList`
//!
//! The `Details` structs are also GET requests, used to return detailed information about the requested data. E.g. `AppDetails`
//!
//! The `Create` structs are typically POST requests, used to create something on Heroku. E.g. `AppCreate`
//!
//! The `Update` structs are typically PATCH requests, used to update some data on Heroku. E.g. `AppUpdate`
//!
//! The `Delete` structs are typically DELETE requests, used to delete some data on Heroku. E.g. `AppDelete`
//!
//! However, there are exceptions to this convention.
//!
//! If for example we want to rollback a release. Where does that fit in? In this case, the naming matches what we are trying to do. So it's a `ReleaseRollback`.
//!
//!
//! ### - Builder Pattern
//! 
//! Builder Pattern in this crate is implemented through struct methods. 
//! 
//! Anything passed through the builder pattern is a optional parameter and `build()` method is required to be called in the end, in order to finish the request. 
//!
//! This pattern is implemented this way in order to abstract away some complications and logic from the developer using the crate. They provide a convenient API that is enabled by default but can be [disabled][tomlfeature].
//!
//! As mentioned before, each endpoint is implemented as a unique struct. Each struct has it's own unique struct methods which can be chained as builder methods.
//!
//! One method that is consistent on every endpoint is the `new(...)` struct method. This method takes the minimal parameters needed but required to make a successful request to Heroku. That could mean no parameters or many parameters, depending on the endpoint's requirements.
//! 
//! 
//! #### Example 1: Requesting a list of apps from Heroku.
//!
//! This request returns a list of [`Apps`][app], if successful.
//!
//!
//! ```rust
//!use heroku_rs::prelude::*;
//!
//!# fn main() -> Result<(), Box<dyn std::error::Error>> {
//!# let api_client = HttpApiClient::create("API_KEY")?;
//! let response = api_client.request(&AppList::new());
//!
//! match response {
//!     Ok(success) => println!("Success: {:#?}", success),
//!     Err(e) => println!("Error: {}", e),
//! }
//!#
//!#    Ok(())
//!# }
//! ```
//!
//! 
//! #### Example 2: Creating a new app.
//! 
//!
//! As mentioned previously, struct methods are used to provide a more convenient API. 
//! 
//! In this case `new()` takes no parameters, because there are no required parameters needed to pass to Heroku.
//! 
//! However, there are three optional parameters we can pass to Heroku if we want to, and we can do that easily through the builder pattern. [See another example][appcreate]
//!
//! ```rust
//!use heroku_rs::prelude::*;
//!
//!# fn main() -> Result<(), Box<dyn std::error::Error>> {
//!#    let api_client = HttpApiClient::create("API_KEY")?;
//! let response = api_client.request(&AppCreate::new());
//!
//! match response {
//!     Ok(success) => println!("Success: {:#?}", success),
//!     Err(e) => println!("Error: {}", e),
//! }
//!#
//!# Ok(())
//!# }
//! ```
//! 
//! 
//! #### Example 3: Updating the app.
//! 
//!
//! Now that we created the app in the previous example, we can update it, if we want to.
//! 
//! In this example, we'll be setting the name "FOO" to it. 
//! 
//! This endpoint has one (1) required parameter and that is the unique app identifier or app_id. 
//! 
//! This is needed for the crate to know which app it will be updating. 
//! 
//! This endpoint also has three optional parameters, `build_stack`, `maintenance` and `name`. We'll only update the name for now.
//! 
//! Notice how we need to call `build()` on `AppUpdate`. That's because we are using the builder pattern, in this case `name("FOO")` to update the app name. So we use `build()` to consume the builder.
//!
//! ```rust
//!use heroku_rs::prelude::*;
//!
//!# fn main() -> Result<(), Box<dyn std::error::Error>> {
//!#    let api_client = HttpApiClient::create("API_KEY")?;
//! let response = api_client.request(&AppUpdate::new("APP_ID").name("FOO").build());
//!
//! match response {
//!     Ok(success) => println!("Success: {:#?}", success),
//!     Err(e) => println!("Error: {}", e),
//! }
//!#
//!#   Ok(())
//!# }
//! ```
//!
//! #### Example 4: Deleting the app.
//! 
//! 
//! Last but not least, we can also delete the app. This has one (1) required parameter, which is the app_id of the App we want to delete.
//!
//! ```rust
//!use heroku_rs::prelude::*;
//!
//!# fn main() -> Result<(), Box<dyn std::error::Error>> {
//!#    let api_client = HttpApiClient::create("API_KEY")?;
//! let response = api_client.request(&AppDelete::new("FOO"));
//!
//! match response {
//!     Ok(success) => println!("Success: {:#?}", success),
//!     Err(e) => println!("Error: {}", e),
//! }
//!#
//!#   Ok(())
//!# }
//! ```
//! Additional examples:
//!
//! - [Heroku_rs repository examples](https://github.com/bensadiku/heroku_rs/tree/master/examples)
//!
//! [reqwest]: https://docs.rs/reqwest/0.10.4/reqwest/
//! [client]: framework/struct.HttpApiClient.html
//! [v3api]: https://devcenter.heroku.com/articles/platform-api-reference#
//! [app]: endpoints/apps/struct.App.html
//! [appcreate]: endpoints/apps/post/struct.AppCreate.html
//! [endpoints]: endpoints/index.html
//! [framework]: framework/index.html
//! [example_custom]: https://github.com/bensadiku/heroku_rs/blob/5d26382089b608cc7dcb08ebd921aa7320e228f8/examples/src/main.rs#L57
//! [tomlfeature]: https://github.com/bensadiku/heroku_rs/blob/master/docs/FEATURES.md

extern crate chrono;
extern crate reqwest;
#[macro_use]
extern crate serde;
extern crate serde_json;

pub mod endpoints;
pub mod framework;

/// A module meant to be glob imported when using heroku_rs.
///
/// For instance:
///
/// ```
/// use heroku_rs::prelude::*;
/// ```
///
/// This module contains several important traits that provide many
/// of the convenience methods in heroku_rs.
pub mod prelude {
    #[cfg(feature = "account")]
    pub use crate::endpoints::account::*;
    #[cfg(feature = "addons")]
    pub use crate::endpoints::addons::*;
    #[cfg(feature = "apps")]
    pub use crate::endpoints::apps::*;
    #[cfg(feature = "builds")]
    pub use crate::endpoints::builds::*;
    #[cfg(feature = "collaborators")]
    pub use crate::endpoints::collaborators::*;
    #[cfg(feature = "config_vars")]
    pub use crate::endpoints::config_vars::*;
    #[cfg(feature = "custom")]
    pub use crate::endpoints::custom::*;
    #[cfg(feature = "domains")]
    pub use crate::endpoints::domains::*;
    #[cfg(feature = "dynos")]
    pub use crate::endpoints::dynos::*;
    #[cfg(feature = "formations")]
    pub use crate::endpoints::formations::*;
    #[cfg(feature = "logs")]
    pub use crate::endpoints::logs::*;
    #[cfg(feature = "misc")]
    pub use crate::endpoints::misc::*;
    #[cfg(feature = "oauth")]
    pub use crate::endpoints::oauth::*;
    #[cfg(feature = "pipelines")]
    pub use crate::endpoints::pipelines::*;
    #[cfg(feature = "releases")]
    pub use crate::endpoints::releases::*;
    #[cfg(feature = "review")]
    pub use crate::endpoints::review::*;
    #[cfg(feature = "slugs")]
    pub use crate::endpoints::slugs::*;
    #[cfg(feature = "space")]
    pub use crate::endpoints::space::*;
    #[cfg(feature = "teams")]
    pub use crate::endpoints::teams::*;
    #[cfg(feature = "testing")]
    pub use crate::endpoints::testing::*;
    #[doc(no_inline)]
    pub use crate::endpoints::*;
    #[doc(no_inline)]
    pub use crate::framework::endpoint::Method;
    #[doc(no_inline)]
    pub use crate::framework::{
        apiclient::HerokuApiClient, auth::Credentials, ApiEnvironment, HttpApiClient,
        HttpApiClientConfig,
    };
}
