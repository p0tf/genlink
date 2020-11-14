mod format;

pub use format::OutputFormat;

use crate::OsStr;
use std::process::Command;

/// The Trait to Integrate Linkers
pub trait Linker {
    /// Return the name of your command.
    fn name(&self) -> &'static OsStr;

    /// Pre-process your linker.
    ///
    /// As the default, this function does nothing.
    fn preproc(&self, _cmd: &mut Command) {}

    /// Check whether the linker is available on the runtime.
    fn ok(&self) -> bool;

    /// Set the output format.
    fn format(&self, cmd: &mut Command, format: OutputFormat);

    /// Add an object.
    ///
    /// As the default, this function just adds the path to the arguments.
    fn obj(&self, cmd: &mut Command, path: &OsStr) {
        cmd.arg(path);
    }

    /// Set the output file. (like `ld -o`)
    fn output(&self, cmd: &mut Command, path: &OsStr);

    /// Add a dynamic library. (like `ld -Bdynamic -l`)
    fn dylib(&self, cmd: &mut Command, name: &OsStr);

    /// Add a static library. (like `ld -Bdynamic -l`)
    fn staticlib(&self, cmd: &mut Command, name: &OsStr);

    /// Add library searching path. (like `ld -L`)
    fn path(&self, cmd: &mut Command, path: &OsStr);

    #[deprecated(since = "0.2.0", note = "This function is no longer used.")]
    fn default_output(&self) -> &'static OsStr {
        OsStr::new("a.out")
    }

    #[deprecated(since = "0.2.0", note = "Please use `dylib` or `staticlib` instead.")]
    #[allow(unused)]
    fn lib(&self, cmd: &mut Command, path: &OsStr) {}
}
