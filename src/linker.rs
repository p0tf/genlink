use crate::OsStr;
use std::process::Command;

/// The Trait to Integrate Linkers
pub trait Linker {
    /// Return the name of your linker.
    fn name(&self) -> &'static OsStr;

    /// Return the default output file.
    ///
    /// As the default, this function returns `"a.out"`.
    fn default_output(&self) -> &'static OsStr {
        OsStr::new("a.out")
    }

    /// Preprocess the linker. (set required arguments, variables, etc.)
    ///
    /// As the default, this function does nothing.
    #[allow(unused)]
    fn preproc(&self, cmd: &mut Command) {}

    /// Add an object.
    ///
    /// As the default, this function just adds the path to the arguments.
    fn obj(&self, cmd: &mut Command, path: &OsStr) {
        cmd.arg(path);
    }

    /// Set the output file. (like `ld -o`)
    fn output(&self, cmd: &mut Command, path: &OsStr);

    /// Search and add library in the library path. (like `ld -l`)
    fn lib(&self, cmd: &mut Command, path: &OsStr);

    /// Add library searching path. (like `ld -L`)
    fn path(&self, cmd: &mut Command, path: &OsStr);
}
