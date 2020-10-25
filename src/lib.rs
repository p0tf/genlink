//! **Gen**eral interface for **Link**ers.
use std::io;
use std::process::Command;

/// General Linker Interface
pub struct GenLink<'a, L> {
    linker: L,
    dest: &'a str,
    command: Command,
}

impl<'a, L: Linker> GenLink<'a, L> {
    /// Create new instance.
    pub fn new(linker: L) -> Self {
        let mut command = Command::new(linker.name());
        linker.preproc(&mut command);
        Self {
            linker,
            dest: "a.out",
            command,
        }
    }

    /// Set the destination.
    pub fn dest(&mut self, path: &'a str) -> &mut Self {
        self.dest = path;
        self
    }

    /// Add an object.
    pub fn obj(&mut self, path: &'a str) -> &mut Self {
        self.linker.add_object(&mut self.command, path);
        self
    }

    /// Add objects.
    pub fn objs<T>(&mut self, paths: T) -> &mut Self
    where
        T: IntoIterator<Item = &'a str>,
    {
        for path in paths {
            self.obj(path);
        }
        self
    }

    /// Add a library.
    pub fn lib(&mut self, path: &'a str) -> &mut Self {
        self.linker.lib(&mut self.command, path);
        self
    }

    /// Add libraries.
    pub fn libs<T>(&mut self, paths: T) -> &mut Self
    where
        T: IntoIterator<Item = &'a str>,
    {
        for path in paths {
            self.lib(path);
        }
        self
    }

    /// Add a library path.
    pub fn lib_path(&mut self, path: &'a str) -> &mut Self {
        self.linker.lib_path(&mut self.command, path);
        self
    }

    /// Add library paths.
    pub fn lib_paths<T>(&mut self, paths: T) -> &mut Self
    where
        T: IntoIterator<Item = &'a str>,
    {
        for path in paths {
            self.lib_path(path);
        }
        self
    }

    /// Link the objects and return the path of the emitted file.
    pub fn link(&mut self) -> io::Result<&'a str> {
        self.linker.dest(&mut self.command, self.dest);
        self.command.output()?;
        Ok(self.dest)
    }
}

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
    fn add_object(&self, cmd: &mut Command, path: &str) {
        cmd.arg(path);
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
