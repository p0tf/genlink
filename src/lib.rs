//! **Gen**eral interface for **Link**ers.
use std::process::Command;

/// List of Command Line Options
///
/// Enumerate options of your linker with this struct.
/// It'll be used by "Linker::options".
///
/// You can describe the basic options (such as "-o", "-L", etc.) to each field, and
/// linker-specific options to `custom` field as `HashSet`.
///
/// > **Note**:
/// >
/// > If some of the basic options are not supported by your linker, please use `unimplemented`
/// > macro.
///
/// # Example
///
/// ```
/// # use genlink::Options;
/// // Gnu's `ld` linker.
/// Options {
///     name: "ld",
/// };
/// ```
pub struct Options {
    pub name: &'static str,
}

/// The Trait to Integrate Linkers
///
/// # Example
///
/// ```
/// # use genlink::{Linker, Options};
/// # use std::process::Command;
/// /// Microsoft's `link` linker.
/// struct Link;
///
/// impl Linker for Link {
///     fn options(&self) -> Options {
///         Options { name: "link" }
///     }
///
///     fn add_arg(&self, cmd: &mut Command, arg: &'static str, value: Option<&'static str>) {
///         match value {
///             Some(v) => cmd.arg(format!("/{}:{}", arg, v)),
///             None => cmd.arg(format!("/{}", arg)),
///         };
///     }
/// }
/// ```
pub trait Linker {
    /// Return `Option` of your linker.
    fn options(&self) -> Options;

    /// Add an object.
    ///
    /// > **Note**
    /// >
    /// > Most linkers work with the default implementation, but you can edit it if needed.
    fn add_object(&self, cmd: &mut Command, value: &'static str) {
        cmd.arg(value);
    }

    /// Add an argument.
    fn add_arg(&self, cmd: &mut Command, arg: &'static str, value: Option<&'static str>);
}

struct Ld;

impl Linker for Ld {
    fn options(&self) -> Options {
        Options { name: "ld" }
    }

    fn add_arg(&self, cmd: &mut Command, arg: &'static str, value: Option<&'static str>) {
        cmd.arg(arg);
        if let Some(s) = value {
            cmd.arg(s);
        }
    }
}
