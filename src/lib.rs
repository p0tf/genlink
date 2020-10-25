//! **Gen**eral interface for **Link**ers.

/// The Linker Property
///
/// Set linker property to this structure.
#[derive(Debug, Clone)]
pub struct Property {
    /// The command name of the linker.
    pub name: &'static str,
    /// Required arguments which are always passed.
    pub required_args: Vec<Arg>,
    /// Basic arguments.
    pub basic_args: BasicArgs,
}

/// Basic Arguments
///
/// This struct is used by `Property` and to enumerate basic options
/// that all linkers should have.
#[derive(Debug, Clone)]
pub struct BasicArgs {}

/// A Command Line Argument
///
/// This sturct is to express a command line argument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arg {
    pub arg: &'static str,
    pub value: Option<String>,
}

impl Arg {
    /// Create new argument.
    pub fn new(arg: &'static str) -> Self {
        Self { arg, value: None }
    }

    /// Pass a value.
    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }
}

/// The Trait to Integrate Linkers
///
/// # Example
///
/// ```
/// # use genlink::{Linker, Property, BasicArgs, Arg};
/// # use std::process::Command;
/// /// Microsoft's `link` linker.
/// struct Link;
///
/// impl Linker for Link {
///     fn options(&self) -> Property {
///         Property {
///             name: "link",
///             required_args: vec![],
///             basic_args: BasicArgs {},
///         }
///     }
///
///     fn add_arg(&self, arg: Arg) -> Vec<String> {
///         match arg.value {
///             Some(v) => vec![format!("/{}:{}", arg.arg, v)],
///             None => vec![format!("/{}", arg.arg)],
///         }
///     }
/// }
/// ```
pub trait Linker {
    /// Return `Option` of your linker.
    fn options(&self) -> Property;

    /// Add an object.
    ///
    /// > **Note**
    /// >
    /// > Most linkers work with the default implementation, but you can edit it if needed.
    fn add_object(&self, value: String) -> Vec<String> {
        vec![value]
    }

    /// Add an argument.
    fn add_arg(&self, arg: Arg) -> Vec<String>;
}

// -- Test --

struct Ld;

impl Linker for Ld {
    fn options(&self) -> Property {
        Property {
            name: "ld",
            required_args: vec![],
            basic_args: BasicArgs {},
        }
    }

    fn add_arg(&self, arg: Arg) -> Vec<String> {
        let mut v = vec![arg.arg.to_string()];
        if let Some(s) = arg.value {
            v.push(s);
        }
        v
    }
}
