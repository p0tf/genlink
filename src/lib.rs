//! **Gen**eral interface for **Link**ers.
use std::process::Command;

/// The Trait to Integrate Linkers
///
/// # Example
///
/// ```
/// # use genlink::Linker;
/// # use std::process::Command;
/// /// Microsoft's `link` linker.
/// struct Link;
///
/// impl Linker for Link {
///     fn name(&self) -> &'static str { "link" }
///     fn dest(&self, cmd: &mut Command, path: &str) { cmd.arg(format!("/out:{}", path)); }
///     fn lib(&self, cmd: &mut Command, path: &str) { cmd.arg(path); }
///     fn lib_path(&self, cmd: &mut Command, path: &str) {
///         cmd.arg(format!("/libpath:{}", path));
///     }
/// }
/// ```
pub trait Linker {
    /// Return the name of your linker.
    fn name(&self) -> &'static str;

    /// Preprocess the linker. (set required arguments, variables, etc.)
    ///
    /// As the default, this function do nothing. You can modify the behavior if needed.
    #[allow(unused)]
    fn preproc(&self, cmd: &mut Command) {}

    /// Add an object.
    ///
    /// As the default, this function just add path to the arguments. You can modify the
    /// behavior if needed.
    fn add_object(&self, cmd: &mut Command, obj: String) {
        cmd.arg(obj);
    }

    /// Set destination to the file. ("-o" option in `ld`)
    fn dest(&self, cmd: &mut Command, path: &str);

    /// Search and add library in the library path. ("-l" option in `ld`)
    fn lib(&self, cmd: &mut Command, path: &str);

    /// Add library searching path. ("-L" option in `ld`)
    fn lib_path(&self, cmd: &mut Command, path: &str);
}

// -- Test --

struct Ld;

impl Linker for Ld {
    fn name(&self) -> &'static str {
        "ld"
    }

    fn dest(&self, cmd: &mut Command, path: &str) {
        cmd.args(&["-o", path]);
    }

    fn lib(&self, cmd: &mut Command, path: &str) {
        cmd.args(&["-l", path]);
    }

    fn lib_path(&self, cmd: &mut Command, path: &str) {
        cmd.args(&["-L", path]);
    }
}
