//! Module for the available endpoints on the crate.

/// apps endpoints
pub mod apps; 
/// dynos endpoints
pub mod dynos;
/// account endpoints
pub mod account;
/// build endpoints
pub mod builds;
/// collaborators endpoints
pub mod collaborators;
/// domain endpoints
pub mod domains;
/// OAuth Authorization
pub mod oauth;
/// pipelines
pub mod pipelines;
/// formations endpoints
pub mod formations;
/// mixed endpoints
pub mod misc; // used for those one off endpoints e.g. ratelimiting, region, stacks etc.
/// slug endpoints
pub mod slugs;
/// config vars
// app config vars, pipeline config vars, release config vars
pub mod config_vars; 
/// custom endpoints
pub mod custom;