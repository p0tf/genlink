//! **Gen**eral interface for **Link**ers.
//!
//! This crate provides general interface to integrate many linkers
//! overflowing in the world (such as [Gnu ld](https://sourceware.org/binutils/),
//! [LLD](https://lld.llvm.org/), etc.) and use as *one* rust library.
//!
//! See also [genlink-impl](https://github.com/watcol/genlink-impl) for concrete
//! implementations for the popular linkers.
//! (They are also good models to implement by yourself. ^^)

mod error;
mod genlink;
mod linker;

// Re-exports
pub use crate::{
    error::{LinkError, LinkResult},
    genlink::GenLink,
    linker::Linker,
};

pub use std::ffi::{OsStr, OsString};
