//! Config organizes hierarchical or layered configurations for Rust applications.
//!
//! This is a temporary fork of the [config](https://docs.rs/config) crate for the needs of the
//! [spirit](https://docs.rs/spirit) framework. You should use the original.
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unknown_lints)]
// #![warn(missing_docs)]

#[macro_use]
extern crate serde;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate nom;

#[macro_use]
extern crate lazy_static;

#[cfg(feature = "toml")]
extern crate toml;

#[cfg(feature = "json")]
extern crate serde_json;

#[cfg(feature = "yaml")]
extern crate yaml_rust;

#[cfg(feature = "hjson")]
extern crate serde_hjson;

#[cfg(feature = "ini")]
extern crate ini;

mod config;
mod de;
mod env;
mod error;
mod file;
mod path;
mod ser;
mod source;
mod value;

pub use config::Config;
pub use env::Environment;
pub use error::ConfigError;
pub use file::{File, FileFormat, FileSourceFile, FileSourceString};
pub use source::Source;
pub use value::Value;
