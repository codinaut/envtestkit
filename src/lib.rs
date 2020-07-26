//! Test kit for logics which involves environment variables.
//!
//! # Usage
//!
//! To test environment-related logics:
//! ```
//! use envtestkit::lock::lock_test;
//! use envtestkit::set_env;
//! use std::ffi::OsString;
//!
//! let _lock = lock_test();
//! let _test = set_env(OsString::from("ENV_KEY"), "value");
//! // Do your test here ...
//! ```
//!
//! Complete examples can be found in `examples/` directory of this crate.
//!
//! # Race Condition
//!
//! Environment variables are global states. These things need to be ensured for building robust tests:
//! - Run environment-modifying tests in serial.
//! - Run environment-sensitive tests while no environment-modifying tests are running at the moment.
//!
//! For that, please use appropriate locks in your tests. See the [lock](lock/index.html) module.

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

#[cfg(feature = "lock")]
#[macro_use]
extern crate lazy_static;

/// Tools for serializing environment-modifying tests.
#[cfg(feature = "lock")]
pub mod lock;

mod testkit;
pub use testkit::*;
