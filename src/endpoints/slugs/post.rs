//Anything related to POST requests for slugs and it's variations goes here.
use super::Slug;

use crate::framework::endpoint::{HerokuEndpoint, Method};
use std::collections::HashMap;

/// Create Slug
///
/// Create a new slug. For more information please refer to ]Deploying Slugs using the Platform API.](https://devcenter.heroku.com/articles/platform-api-deploying-slugs)
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#slug-create)
/// 
/// # Example:
///
/// SlugCreate takes two required parameters, app_id and process_types, and returns the created [`Slug`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let mut process_types = HashMap::new();
/// process_types.insert("web", "./bin/web -p $PORT");
/// 
/// let response = api_client.request(
///     &SlugCreate::new("APP_ID", process_types)
///         .commit("60883d9e8947a57e04dc9124f25df004866a2051")
///         .commit_description("fixed a bug with API documentation")
///         .buildpack_provided_description("Ruby/Rack")
///         .checksum("SHA256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
///         .stack("cedar-14")
///         .build(),
/// );
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
/// [response]: ../struct.Slug.html
pub struct SlugCreate<'a> {
    /// app_id is the unique app identifier.
    pub app_id: &'a str,
    /// The parameters to pass to the Heroku API
    pub params: SlugCreateParams<'a>,
}

#[cfg(feature = "builder")]
impl<'a> SlugCreate<'a> {
    pub fn new(app_id: &'a str, process_types: HashMap<&'a str, &'a str>) -> SlugCreate<'a> {
        SlugCreate {
            app_id,
            params: SlugCreateParams {
                process_types: process_types,
                buildpack_provided_description: None,
                checksum: None,
                commit: None,
                commit_description: None,
                stack: None,
            },
        }
    }

    /// # buildpack_provided_description: description from buildpack of slug
    pub fn buildpack_provided_description(
        &mut self,
        buildpack_provided_description: &'a str,
    ) -> &mut Self {
        self.params.buildpack_provided_description = Some(buildpack_provided_description);
        self
    }

    /// # checksum: an optional checksum of the slug for verifying its integrity
    pub fn checksum(&mut self, checksum: &'a str) -> &mut Self {
        self.params.checksum = Some(checksum);
        self
    }

    /// # commit: identification of the code with your version control system (eg: SHA of the git HEAD)
    pub fn commit(&mut self, commit: &'a str) -> &mut Self {
        self.params.commit = Some(commit);
        self
    }

    /// # commit_description: an optional description of the provided commit
    pub fn commit_description(&mut self, commit_description: &'a str) -> &mut Self {
        self.params.commit_description = Some(commit_description);
        self
    }

    /// # stack: unique name or identifier of stack
    pub fn stack(&mut self, stack: &'a str) -> &mut Self {
        self.params.stack = Some(stack);
        self
    }

    pub fn build(&self) -> SlugCreate<'a> {
        SlugCreate {
            app_id: self.app_id,
            params: SlugCreateParams {
                process_types: self.params.process_types.clone(),
                buildpack_provided_description: self.params.buildpack_provided_description,
                checksum: self.params.checksum,
                commit: self.params.commit,
                commit_description: self.params.commit_description,
                stack: self.params.stack,
            },
        }
    }
}

/// Create a new slug with parameters.
///
/// [See Heroku documentation for more information about these required parameters](https://devcenter.heroku.com/articles/platform-api-reference#slug-create-required-parameters)
///
/// [See Heroku documentation for more information about these optional parameters](https://devcenter.heroku.com/articles/platform-api-reference#slug-create-optional-parameters)
#[derive(Serialize, Clone, Debug)]
pub struct SlugCreateParams<'a> {
    /// hash mapping process type names to their respective command. [Nullable]
    pub process_types: HashMap<&'a str, &'a str>,
    /// human-friendly description from buildpack of slug. [Nullable]
    pub buildpack_provided_description: Option<&'a str>,
    //// an optional checksum of the slug for verifying its integrity. [Nullable]
    pub checksum: Option<&'a str>,
    /// identification of the code with your version control system (eg: SHA of the git HEAD). [Nullable]
    pub commit: Option<&'a str>,
    /// an optional description of the provided commit. [Nullable]
    pub commit_description: Option<&'a str>,
    /// unique name or identifier of stack
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<&'a str>,
}

impl<'a> HerokuEndpoint<Slug, (), SlugCreateParams<'a>> for SlugCreate<'a> {
    fn method(&self) -> Method {
        Method::Post
    }
    fn path(&self) -> String {
        format!("apps/{}/slugs", self.app_id)
    }
    fn body(&self) -> Option<SlugCreateParams<'a>> {
        Some(self.params.clone())
    }
}
