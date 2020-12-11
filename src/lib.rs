//! **Gen**eral interface for **Link**ers.
//!
//! This crate provides general interface to integrate many linkers
//! overflowing in the world (such as [Gnu ld](https://sourceware.org/binutils/),
//! [LLD](https://lld.llvm.org/), etc.) and use as *one* rust library.

mod error;
mod genlink;
mod linker;

// Re-exports
pub use crate::{
    error::{LinkError, LinkResult},
    genlink::GenLink,
    linker::{Linker, OutputFormat},
};

pub use std::ffi::{OsStr, OsString};
