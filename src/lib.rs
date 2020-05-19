#![doc(html_root_url = "https://docs.rs/heroku_rs/0.5.1")]

//! # heroku_rs
//!
//! The `heroku_rs` crate provides convenient Rust bindings for the [Heroku V3 API][v3api].
//!
//! The heroku_rs [`HttpApiClient`][client] is blocking by deafult.
//!
//! Additional examples:
//!
//! - [Heroku_rs repository examples](https://github.com/bensadiku/heroku_rs/tree/master/examples)
//!
//!
//! ## Creating the Heroku [`HttpApiClient`][client]
//!
//! The heroku client is build on top of [`reqwest`][reqwest] library.
//!
//! Creating the Heroku client only takes 1 line. This client has the default 30s http timeout and points to the production Heroku API.
//!
//! # Example 1 - Creating a simple client
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
//! If you want a custom client, you need to specify three things. [Credentials][credentials], [HttpApiClientConfig][httpApiClientConfig] and [ApiEnvironment][apiEnviroment] See this [example][example_custom]
//!
//!
//! ## Making a GET request to Heroku.
//!
//!
//! This request returns a vector of [`app`][app]s, if successful.
//!
//!
//! ```rust
//!use heroku_rs::prelude::*;
//!
//!# fn main() -> Result<(), Box<dyn std::error::Error>> {
//!#
//!    let api_client = HttpApiClient::create("API_KEY")?;
//!
//!    let response = api_client.request(&apps::AppList {});
//!
//!    match response {
//!        Ok(success) => println!("Success: {:#?}", success),
//!        Err(e) => println!("Error: {}", e),
//!    }
//!#
//!#    Ok(())
//!# }
//! ```
//!
//! ## Making POST requests to Heroku.
//!
//!
//! Some POST requests do not need body paramers at all.
//!
//! This crate provides convinient parameter structs to inform you which endpoints take what parameters and which parameters are optional.
//!
//! If you can see the `params` parameter in this example, it takes three fields, all three are optional, matched from the Heroku documentation.
//!
//! ```rust
//!use heroku_rs::prelude::*;
//!
//!# fn main() -> Result<(), Box<dyn std::error::Error>> {
//!#
//!    let api_client = HttpApiClient::create("API_KEY")?;
//!
//!    let new_app = &apps::AppCreate::new().name("FOO").build();
//!    let response = api_client.request(new_app);
//!
//!    match response {
//!       Ok(success) => println!("Success: {:#?}", success),
//!       Err(e) => println!("Error: {}", e),
//!    }
//!#
//!# Ok(())
//!# }
//! ```
//!
//!
//! ## Making DELETE requests to Heroku.
//!
//!
//! Contraty to POST requests, DELETE requests do not need body parameters at all.
//!
//! Some DELETE requests return a body on the response if successful, some do not.
//!
//!
//! ```rust
//!use heroku_rs::prelude::*;
//!
//!# fn main() -> Result<(), Box<dyn std::error::Error>> {
//!
//!    let api_client = HttpApiClient::create("API_KEY")?;
//!
//!    // This will delete the `FOO` app.
//!    let response = api_client.request(&apps::AppDelete::new("FOO"));
//!
//!    match response {
//!       Ok(success) => println!("Success: {:#?}", success),
//!       Err(e) => println!("Error: {}", e),
//!    }
//!#
//!#   Ok(())
//!# }
//! ```
//!
//!
//! ## Making PATCH requests to Heroku.
//!
//!
//! Similar to POST requests, Some PATCH requests do not need body paramers, in this case we are updating the app name from "FOO" to "BAZ" and turning maintenance off.
//!
//! ```rust
//!use heroku_rs::prelude::*;
//!
//!# fn main() -> Result<(), Box<dyn std::error::Error>> {
//!
//!    let api_client = HttpApiClient::create("API_KEY")?;
//!
//!    let patch = &apps::AppUpdate::new("FOO")
//!         .name("BAZ")
//!         .maintenance(false)
//!         .build();
//!    let response = api_client.request(patch);
//!
//!    match response {
//!       Ok(success) => println!("Success: {:#?}", success),
//!       Err(e) => println!("Error: {}", e),
//!    }
//!#
//!#   Ok(())
//!# }
//! ```
//!
//! [reqwest]: https://github.com/seanmonstar/reqwest
//! [client]: framework/struct.HttpApiClient.html
//! [v3api]: https://devcenter.heroku.com/articles/platform-api-reference#
//! [app]: endpoints/apps/struct.App.html
//! [apiEnviroment]: framework/enum.ApiEnvironment.html
//! [httpApiClientConfig]: framework/struct.HttpApiClientConfig.html
//! [credentials]: framework/auth/enum.Credentials.html
//! [example_custom]: https://github.com/bensadiku/heroku_rs/blob/5d26382089b608cc7dcb08ebd921aa7320e228f8/examples/src/main.rs#L57

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
