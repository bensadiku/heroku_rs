
//! Library to used to access the Heroku API with Rust
#![allow(dead_code)] // Until every starting struct gets used
#![deny(//missing_docs,
        unsafe_code,
        unused_import_braces,
        unused_qualifications)]

#[macro_use]
extern crate error_chain;

#[macro_use]
mod macros;
mod utils;

pub mod client;
pub mod defaults;
pub mod errors;
pub mod headers;
pub mod apps;
pub mod teams;

pub use hyper::{HeaderMap, StatusCode};