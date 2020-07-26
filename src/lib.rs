#[cfg(feature = "lock")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "lock")]
pub mod lock;

mod testkit;
pub use testkit::*;
