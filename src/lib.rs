//! **Gen**eral interface for **Link**ers.
//!
//! For example implementations, see
//! [genlink-impl](https://github.com/watcol/genlink-impl).

mod genlink;
mod linker;
mod error;

// Re-exports
pub use genlink::GenLink;
pub use linker::Linker;
pub use error::{LinkError, LinkResult};
pub use std::ffi::{OsStr, OsString};
