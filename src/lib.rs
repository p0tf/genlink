//! **Gen**eral interface for **Link**ers.
//!
//! For example implementations, see
//! [genlink-impl](https://github.com/watcol/genlink-impl).

mod error;
mod genlink;
mod linker;

// Re-exports
pub use error::{LinkError, LinkResult};
pub use genlink::GenLink;
pub use linker::Linker;
pub use std::ffi::{OsStr, OsString};
