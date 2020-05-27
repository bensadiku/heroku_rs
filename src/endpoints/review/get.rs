//Anything related to GET requests for review app and it's properties goes here.
use super::{ReviewApp, ReviewAppConfig};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Get Review App
///
/// Gets an existing review app
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-get-review-app)
/// 
/// # Example:
///
/// ReviewAppDetails takes one required parameter, review_id, and returns the [`ReviewApp`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&ReviewAppDetails::new("REVIEW_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
///
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.ReviewApp.html
pub struct ReviewAppDetails<'a> {
    /// review_id is the unique identifier.
    pub review_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> ReviewAppDetails<'a> {
    pub fn new(review_id: &'a str) -> ReviewAppDetails<'a> {
        ReviewAppDetails { review_id }
    }
}

impl<'a> HerokuEndpoint<ReviewApp> for ReviewAppDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("review-apps/{}", self.review_id)
    }
}

/// Get Review App by App id
///
/// Get a review app using the associated app_id
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-get-review-app-by-app_id)
/// 
/// # Example:
///
/// ReviewAppByAppDetails takes one required parameter, app_id, and returns the [`ReviewApp`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&ReviewAppByAppDetails::new("APP_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
///
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.ReviewApp.html
pub struct ReviewAppByAppDetails<'a> {
    /// app_id is the unique identifier, app name or app id.
    pub app_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> ReviewAppByAppDetails<'a> {
    pub fn new(app_id: &'a str) -> ReviewAppByAppDetails<'a> {
        ReviewAppByAppDetails { app_id }
    }
}

impl<'a> HerokuEndpoint<ReviewApp> for ReviewAppByAppDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("apps/{}/review-app", self.app_id)
    }
}

/// Review App List by Pipeline id
///
/// List review apps for a pipeline
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-list)
/// 
/// # Example:
///
/// ReviewAppByPipelineList takes one required parameter, pipeline_id, and returns a list of [`ReviewApp`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&ReviewAppByPipelineList::new("PIPELINE_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
///
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.ReviewApp.html
pub struct ReviewAppByPipelineList<'a> {
    /// app_id is the unique identifier, app name or app id.
    pub pipeline_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> ReviewAppByPipelineList<'a> {
    pub fn new(pipeline_id: &'a str) -> ReviewAppByPipelineList<'a> {
        ReviewAppByPipelineList { pipeline_id }
    }
}

impl<'a> HerokuEndpoint<Vec<ReviewApp>> for ReviewAppByPipelineList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines/{}/review-apps", self.pipeline_id)
    }
}

/// Review App Configuration Info
///
/// Get review apps configuration for a pipeline
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-configuration-info)
/// 
/// # Example:
///
/// ReviewAppConfigDetails takes one required parameter, pipeline_id, and returns the [`ReviewApp`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&ReviewAppConfigDetails::new("PIPELINE_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
///
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.ReviewApp.html
pub struct ReviewAppConfigDetails<'a> {
    /// app_id is the unique identifier, app name or app id.
    pub pipeline_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> ReviewAppConfigDetails<'a> {
    pub fn new(pipeline_id: &'a str) -> ReviewAppConfigDetails<'a> {
        ReviewAppConfigDetails { pipeline_id }
    }
}

impl<'a> HerokuEndpoint<ReviewAppConfig> for ReviewAppConfigDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines/{}/review-app-config", self.pipeline_id)
    }
}
