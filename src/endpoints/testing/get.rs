//Anything related to GET requests for Heroku tests goes here.
use super::{TestCase, TestNode, TestRun};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Test Case List
///
/// List test cases
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#test-case-list)
/// 
/// # Example:
///
/// TestCaseList takes one required parameter, run_id, and returns a list of [`TestCases`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&TestCaseList::new("RUN_ID"));
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
/// [response]: ../struct.TestCase.html
pub struct TestCaseList<'a> {
    /// run_id is the test run unique identifier
    pub run_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> TestCaseList<'a> {
    pub fn new(run_id: &'a str) -> TestCaseList<'a> {
        TestCaseList { run_id }
    }
}

impl<'a> HerokuEndpoint<Vec<TestCase>> for TestCaseList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("test-runs/{}/test-cases", self.run_id)
    }
}

/// Test Node List
///
/// List test nodes
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#test-node-list)
/// 
/// # Example:
///
/// TestNodeList takes one required parameter, run_id, and returns a list of [`TestNodes`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&TestNodeList::new("RUN_ID"));
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
/// [response]: ../struct.TestNode.html
pub struct TestNodeList<'a> {
    /// run_id is the test run unique identifier
    pub run_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> TestNodeList<'a> {
    pub fn new(run_id: &'a str) -> TestNodeList<'a> {
        TestNodeList { run_id }
    }
}

impl<'a> HerokuEndpoint<Vec<TestNode>> for TestNodeList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("test-runs/{}/test-nodes", self.run_id)
    }
}

/// Test Run Info
///
/// Info for existing test-run.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#test-run-info)
/// 
/// # Example:
///
/// TestRunDetails takes one required parameter, run_id, and returns a [`TestRun`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&TestRunDetails::new("RUN_ID"));
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
/// [response]: ../struct.TestRun.html
pub struct TestRunDetails<'a> {
    /// run_id is the test run unique identifier
    pub run_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> TestRunDetails<'a> {
    pub fn new(run_id: &'a str) -> TestRunDetails<'a> {
        TestRunDetails { run_id }
    }
}

impl<'a> HerokuEndpoint<TestRun> for TestRunDetails<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("test-runs/{}", self.run_id)
    }
}

/// Test Run List
///
/// List existing test-runs for a pipeline.
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#test-run-list)
/// 
/// # Example:
///
/// TestRunList takes one required parameter, pipeline_id, and returns a list of [`TestRuns`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&TestRunList::new("PIPELINE_ID"));
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
/// [response]: ../struct.TestRun.html
pub struct TestRunList<'a> {
    /// pipeline_id is the test run pipeline identifier
    pub pipeline_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> TestRunList<'a> {
    pub fn new(pipeline_id: &'a str) -> TestRunList<'a> {
        TestRunList { pipeline_id }
    }
}

impl<'a> HerokuEndpoint<Vec<TestRun>> for TestRunList<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines/{}/test-runs", self.pipeline_id)
    }
}

/// Test Run Info By Pipeline
///
/// Info for existing test-run by Pipeline
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#test-run-info-by-pipeline)
/// 
/// # Example:
///
/// TestRunDetailsByPipeline takes two required parameters, pipeline_id and run_id, and returns a [`TestRun`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&TestRunDetailsByPipeline::new("PIPELINE_ID", "RUN_ID"));
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
/// [response]: ../struct.TestRun.html
pub struct TestRunDetailsByPipeline<'a> {
    /// pipeline_id is the test run pipeline identifier
    pub pipeline_id: &'a str,
    /// run_id is the test run unique identifier
    pub run_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> TestRunDetailsByPipeline<'a> {
    pub fn new(pipeline_id: &'a str, run_id: &'a str) -> TestRunDetailsByPipeline<'a> {
        TestRunDetailsByPipeline {
            pipeline_id,
            run_id,
        }
    }
}

impl<'a> HerokuEndpoint<TestRun> for TestRunDetailsByPipeline<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!("pipelines/{}/test-runs/{}", self.pipeline_id, self.run_id)
    }
}
