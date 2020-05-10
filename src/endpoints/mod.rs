//! Module for the available endpoints on the crate.

/// account endpoints
pub mod account;
/// addons
pub mod addons;
/// apps endpoints
pub mod apps;
/// build endpoints
pub mod builds;
/// collaborators endpoints
pub mod collaborators;
/// config vars endpoints
// app config vars, pipeline config vars, release config vars
pub mod config_vars;
/// custom endpoints
pub mod custom;
/// domain endpoints
pub mod domains;
/// dynos endpoints
pub mod dynos;
/// formations endpoints
pub mod formations;
/// heroku logs endpoints
pub mod logs;
/// mixed endpoints
pub mod misc; // used for those one off endpoints e.g. ratelimiting, region, stacks etc.
/// oauth endpoints
pub mod oauth;
/// pipeline endpoints
pub mod pipelines;
/// release endpoints
pub mod releases;
/// app review endpoints
pub mod review;
/// slug endpoints
pub mod slugs;
/// spaces endpoints
pub mod space;
/// teams endpoints
pub mod teams;
