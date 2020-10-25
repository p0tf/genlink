use crate::OsStr;
use std::process::Command;

/// The Trait to Integrate Linkers
pub trait Linker {
    /// Return the name of your linker.
    fn name(&self) -> &'static OsStr;

    /// Return the default output file.
    ///
    /// As the default this function returns "a.out".
    fn def_dest(&self) -> &'static OsStr {
        OsStr::new("a.out")
    }

    /// Preprocess the linker. (set required arguments, variables, etc.)
    ///
    /// As the default, this function do nothing.
    #[allow(unused)]
    fn preproc(&self, cmd: &mut Command) {}

    /// Add an object.
    ///
    /// As the default, this function just add path to the arguments.
    fn obj(&self, cmd: &mut Command, path: &OsStr) {
        cmd.arg(path);
    }

    /// Set destination to the file. ("-o" option in `ld`)
    fn dest(&self, cmd: &mut Command, path: &OsStr);

    /// Search and add library in the library path. ("-l" option in `ld`)
    fn lib(&self, cmd: &mut Command, path: &OsStr);

    /// Add library searching path. ("-L" option in `ld`)
    fn path(&self, cmd: &mut Command, path: &OsStr);
}
