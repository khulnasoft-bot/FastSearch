#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Fastsearch
//!
//! Rust client library for Fastsearch
//!
//! # Examples
//!
//! ```
//! #[cfg(any(feature = "tokio_test", target_arch = "wasm32"))]
//! {
//! use serde::{Deserialize, Serialize};
//! use fastsearch::document::Document;
//! use fastsearch::Fastsearch;
//! use fastsearch::apis::collections_api;
//! use fastsearch::apis::configuration::{ApiKey, Configuration};
//!
//! #[derive(Fastsearch, Serialize, Deserialize)]
//! #[fastsearch(collection_name = "companies", default_sorting_field = "num_employees")]
//! struct Company {
//!     company_name: String,
//!     num_employees: i32,
//!     #[fastsearch(facet)]
//!     country: String,
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = Configuration {
//!         base_path: "http://localhost:5000".to_owned(),
//!         api_key: Some(ApiKey {
//!             prefix: None,
//!             key: "VerySecretKey".to_owned(),
//!         }),
//!         ..Default::default()
//!     };
//!
//!     let collection = collections_api::create_collection(&config, Company::collection_schema())
//!         .await
//!         .unwrap();
//! }
//! }
//! ```

pub mod collection_schema;
pub mod document;
pub mod field;
pub mod keys;

pub use fastsearch_codegen::*;

#[cfg(feature = "fastsearch_derive")]
#[doc(hidden)]
pub use fastsearch_derive::*;
