//! Module for the available endpoints on the crate.

/// account endpoints
#[cfg(feature = "account")]
pub mod account;
/// addons
#[cfg(feature = "addons")]
pub mod addons;
/// apps endpoints
#[cfg(feature = "apps")]
pub mod apps;
/// build endpoints
#[cfg(feature = "builds")]
pub mod builds;
/// collaborators endpoints
#[cfg(feature = "collaborators")]
pub mod collaborators;
/// config vars endpoints
// app config vars, pipeline config vars, release config vars
#[cfg(feature = "config_vars")]
pub mod config_vars;
/// custom endpoints
#[cfg(feature = "custom")]
pub mod custom;
/// domain endpoints
#[cfg(feature = "domains")]
pub mod domains;
/// dynos endpoints
#[cfg(feature = "dynos")]
pub mod dynos;
/// formations endpoints
#[cfg(feature = "formations")]
pub mod formations;
/// heroku logs endpoints
#[cfg(feature = "logs")]
pub mod logs;
/// mixed endpoints
// used for those one off endpoints e.g. ratelimiting, region, stacks etc.
#[cfg(feature = "misc")]
pub mod misc; 
/// oauth endpoints
#[cfg(feature = "oauth")]
pub mod oauth;
/// pipeline endpoints
#[cfg(feature = "pipelines")]
pub mod pipelines;
/// release endpoints
#[cfg(feature = "releases")]
pub mod releases;
/// app review endpoints
#[cfg(feature = "review")]
pub mod review;
/// slug endpoints
#[cfg(feature = "slugs")]
pub mod slugs;
/// spaces endpoints
#[cfg(feature = "space")]
pub mod space;
/// teams endpoints
#[cfg(feature = "teams")]
pub mod teams;
