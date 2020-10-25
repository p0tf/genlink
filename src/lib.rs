//! **Gen**eral interface for **Link**ers.
use std::path::PathBuf;
use std::io;
use std::process::Command;

// Re-exports
pub use std::ffi::{OsStr, OsString};

/// General Linker Interface
pub struct GenLink<L> {
    linker: L,
    dest: OsString,
    command: Command,
}

impl<L: Linker> GenLink<L> {
    /// Create new instance.
    pub fn new(linker: L) -> Self {
        let mut command = Command::new(linker.name());
        linker.preproc(&mut command);
        Self {
            linker,
            dest: OsStr::new("a.out").into(),
            command,
        }
    }

    /// Set the destination.
    pub fn dest<T: Into<OsString>>(&mut self, path: T) -> &mut Self {
        self.dest = path.into();
        self
    }

    /// Add an object.
    pub fn obj<T: AsRef<OsStr>>(&mut self, path: T) -> io::Result<&mut Self> {
        let path = std::fs::canonicalize(path.as_ref())?;
        self.linker.obj(&mut self.command, path.as_os_str());
        Ok(self)
    }

    /// Add objects.
    pub fn objs<T, TS>(&mut self, paths: TS) -> io::Result<&mut Self>
    where
        T: AsRef<OsStr>,
        TS: IntoIterator<Item = T>,
    {
        for path in paths {
            self.obj(path)?;
        }
        Ok(self)
    }

    /// Add a library.
    pub fn lib<T: AsRef<OsStr>>(&mut self, path: T) -> &mut Self {
        self.linker.lib(&mut self.command, path.as_ref());
        self
    }

    /// Add libraries.
    pub fn libs<T, TS>(&mut self, paths: TS) -> &mut Self
    where
        T: AsRef<OsStr>,
        TS: IntoIterator<Item = T>,
    {
        for path in paths {
            self.lib(path);
        }
        self
    }

    /// Add a library path.
    pub fn path<T: AsRef<OsStr>>(&mut self, path: T) -> &mut Self {
        self.linker.path(&mut self.command, path.as_ref());
        self
    }

    /// Add library paths.
    pub fn paths<T, TS>(&mut self, paths: TS) -> &mut Self
    where
        T: AsRef<OsStr>,
        TS: IntoIterator<Item = T>,
    {
        for path in paths {
            self.path(path);
        }
        self
    }

    /// Link the objects and return the path of the emitted file.
    pub fn link(&mut self) -> io::Result<PathBuf> {
        let dest = std::fs::canonicalize(&self.dest)?;
        self.linker.dest(&mut self.command, dest.as_os_str());
        self.command.output()?;
        Ok(dest)
    }
}

/// The Trait to Integrate Linkers
///
/// # Example
///
/// ```
/// # use genlink::{Linker, OsStr, OsString};
/// # use std::process::Command;
/// /// Microsoft's `link` linker.
/// struct Link;
///
/// impl Linker for Link {
///     fn name(&self) -> &'static OsStr { OsStr::new("link" ) }
///     fn dest(&self, cmd: &mut Command, path: &OsStr) {
///         let mut s = OsString::from("/out:");
///         s.push(path);
///         cmd.arg(s);
///     }
///
///     fn lib(&self, cmd: &mut Command, path: &OsStr) { cmd.arg(path); }
///     fn path(&self, cmd: &mut Command, path: &OsStr) {
///         let mut s = OsString::from("/libpath:");
///         s.push(path);
///         cmd.arg(s);
///     }
/// }
/// ```
pub trait Linker {
    /// Return the name of your linker.
    fn name(&self) -> &'static OsStr;

    /// Preprocess the linker. (set required arguments, variables, etc.)
    ///
    /// As the default, this function do nothing. You can modify the behavior if needed.
    #[allow(unused)]
    fn preproc(&self, cmd: &mut Command) {}

    /// Add an object.
    ///
    /// As the default, this function just add path to the arguments. You can modify the
    /// behavior if needed.
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

// -- Test --

struct Ld;

impl Linker for Ld {
    fn name(&self) -> &'static OsStr {
        OsStr::new("ld")
    }

    fn dest(&self, cmd: &mut Command, path: &OsStr) {
        cmd.args(&[OsStr::new("-o"), path]);
    }

    fn lib(&self, cmd: &mut Command, path: &OsStr) {
        cmd.args(&[OsStr::new("-l"), path]);
    }

    fn path(&self, cmd: &mut Command, path: &OsStr) {
        cmd.args(&[OsStr::new("-L"), path]);
    }
}
