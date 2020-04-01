#![doc(html_root_url = "https://docs.rs/heroku_rs/0.3.1")]

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
//! If you wish to custumize the http timeout or the base endpoint. See the Example 2 
//! 
//! # Example 1 - Creating a simple client
//! 
//! ```rust
//! use heroku_rs::framework::{apiclient::HerokuApiClient, HttpApiClient};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let api_client = HttpApiClient::create("API_KEY")?;
//!   
//!    // You can start making requests here
//!    
//!    Ok(())
//! }
//! ```
//! In order to have a fully functional custom client you need to specify three things. [Credentials][credentials], [HttpApiClientConfig][httpApiClientConfig] and [ApiEnvironment][apiEnviroment]
//! 
//! # Example 2 - Creating a custom client
//! ```
//! use heroku_rs::framework::{
//! auth::Credentials,
//! apiclient::HerokuApiClient,
//! ApiEnvironment, HttpApiClient, HttpApiClientConfig,
//! };
//! use heroku_rs::endpoints::apps;
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//! 
//!   let credentials = Credentials::UserAuthToken {
//!       token: String::from("TOKEN_HERE"),
//!   };
//!   
//!   let api_client = HttpApiClient::new(
//!       credentials,
//!       HttpApiClientConfig::default(),
//!       ApiEnvironment::Production,
//!   )?;
//!   
//!   let response = api_client.request(&apps::AppList {});
//!   
//!   match response {
//!       Ok(success) => println!("Success: {:#?}", success),
//!       Err(e) => println!("Error: {}", e),
//!   }
//!   
//!   Ok(())
//! }
//! ```
//! ## Making a GET request to Heroku.
//! 
//!
//! This request returns a vector of [`app`][app]s, if successful.
//! 
//!
//! ```rust
//!# use heroku_rs::framework::{
//!#    auth::Credentials,
//!#    response::{ApiResponse, ApiResult},
//!#    apiclient::HerokuApiClient,
//!#    ApiEnvironment, HttpApiClient, HttpApiClientConfig,
//!# };
//!use heroku_rs::endpoints::apps;
//!
//!# fn main() -> Result<(), Box<dyn std::error::Error>> {
//!#
//!#    let credentials = Credentials::UserAuthToken {
//!#        token: String::from("TOKEN_HERE"),
//!#    };
//!#
//!#    let api_client = HttpApiClient::new(
//!#        credentials,
//!#        HttpApiClientConfig::default(),
//!#        ApiEnvironment::Production,
//!#    )?;
//!#
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
//!# use heroku_rs::framework::{
//!#    auth::Credentials,
//!#    response::{ApiResponse, ApiResult},
//!#    apiclient::HerokuApiClient,
//!#    ApiEnvironment, HttpApiClient, HttpApiClientConfig,
//!# };
//!use heroku_rs::endpoints::apps;
//!
//!# fn main() -> Result<(), Box<dyn std::error::Error>> {
//!#
//!#   let credentials = Credentials::UserAuthToken {
//!#       token: String::from("TOKEN_HERE"),
//!#   };
//!#
//!#   let api_client = HttpApiClient::new(
//!#       credentials,
//!#       HttpApiClientConfig::default(),
//!#       ApiEnvironment::Production,
//!#   )?;
//!#
//!    let app_name = String::from("FOO");
//! 
//!    let response = api_client.request(&apps::AppCreate {
//!        // This will create an app with the name name `FOO_APP`
//!        params: apps::AppCreateParams {
//!            name: Some(app_name),
//!            region: None,
//!            stack: None,
//!        },
//!    });
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
//!# use heroku_rs::framework::{
//!#    auth::Credentials,
//!#    response::{ApiResponse, ApiResult},
//!#    apiclient::HerokuApiClient,
//!#    ApiEnvironment, HttpApiClient, HttpApiClientConfig,
//!# };
//!use heroku_rs::endpoints::apps;
//!
//!# fn main() -> Result<(), Box<dyn std::error::Error>> {
//!#
//!#   let credentials = Credentials::UserAuthToken {
//!#       token: String::from("TOKEN_HERE"),
//!#   };
//!#
//!#   let api_client = HttpApiClient::new(
//!#       credentials,
//!#       HttpApiClientConfig::default(),
//!#       ApiEnvironment::Production,
//!#   )?;
//!#
//!    let app_id = String::from("FOO");
//! 
//!    // This will delete the `FOO` app.
//!    let response = api_client.request(&apps::AppDelete { app_id });
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
//! Similar to POST requests, Some PATCH requests do not need body paramers. 
//! 
//! 
//! ```rust
//!# use heroku_rs::framework::{
//!#    auth::Credentials,
//!#    response::{ApiResponse, ApiResult},
//!#    apiclient::HerokuApiClient,
//!#    ApiEnvironment, HttpApiClient, HttpApiClientConfig,
//!# };
//!use heroku_rs::endpoints::apps;
//!
//!# fn main() -> Result<(), Box<dyn std::error::Error>> {
//!#
//!#   let credentials = Credentials::UserAuthToken {
//!#       token: String::from("TOKEN_HERE"),
//!#   };
//!#
//!#   let api_client = HttpApiClient::new(
//!#       credentials,
//!#       HttpApiClientConfig::default(),
//!#       ApiEnvironment::Production,
//!#   )?;
//!#
//!    let app_id = String::from("FOO");
//! 
//!    // This will enable maintenance for the "FOO" app.
//!     let response = api_client.request(&apps::AppUpdate {
//!         app_id,
//!         params: apps::AppUpdateParams {
//!             build_stack: None,
//!             maintenance: Some(true),
//!             name: None,
//!         },
//!     });
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

extern crate chrono;
extern crate reqwest;
#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate serde_qs;
extern crate url;

pub mod endpoints;
pub mod framework;