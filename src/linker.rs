use crate::OsStr;
use std::process::Command;
use OutputFormat::*;

/// The output format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    DynamicExe,
    DynamicPicExe,
    StaticExe,
    StaticPicExe,
    DynamicDyLib,
    StaticDyLib,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::DynamicExe
    }
}

impl OutputFormat {
    /// Choose either the static (if `flag` is `true`), or dynamic (else) binary.
    pub fn stat(&self, flag: bool) -> Self {
        if flag {
            self.static_()
        } else {
            self.dynamic_()
        }
    }

    /// Choose either the PIC (if `flag` is `true`), or no-PIC (else) binary.
    pub fn pic(&self, flag: bool) -> Self {
        if flag {
            self.pic_()
        } else {
            self.no_pic_()
        }
    }

    /// Choose either the executable (if `flag` is `true`), or the dynamic-linked library (else).
    pub fn lib(&self, flag: bool) -> Self {
        if flag {
            self.lib_()
        } else {
            self.exe_()
        }
    }

    fn static_(&self) -> Self {
        match self {
            DynamicExe => StaticExe,
            DynamicPicExe => StaticPicExe,
            DynamicDyLib => StaticDyLib,
            i => *i
        }
    }

    fn dynamic_(&self) -> Self {
        match self {
            StaticExe => DynamicExe,
            StaticPicExe => DynamicPicExe,
            StaticDyLib => DynamicDyLib,
            i => *i
        }
    }

    fn pic_(&self) -> Self {
        match self {
            DynamicExe => DynamicPicExe,
            StaticExe => StaticPicExe,
            i => *i
        }
    }

    fn no_pic_(&self) -> Self {
        match self {
            StaticPicExe => StaticExe,
            DynamicPicExe => DynamicExe,
            i => *i
        }
    }

    fn lib_(&self) -> Self {
        match self {
            DynamicExe | DynamicPicExe => DynamicDyLib,
            StaticExe | StaticPicExe => StaticDyLib,
            i => *i
        }
    }

    fn exe_(&self) -> Self {
        match self {
            StaticDyLib => StaticExe,
            DynamicDyLib => DynamicExe,
            i => *i
        }
    }
}

/// The Trait to Integrate Linkers
pub trait Linker {
    /// Return the initialized command.
    fn cmd(&self) -> Command;

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

    #[deprecated(since = "0.2.0", note = "Please use `cmd` instead.")]
    fn name(&self) -> &'static OsStr {
        OsStr::new("`name` is deprecated. Use `cmd`.")
    }

    #[deprecated(since = "0.2.0", note = "This function is no longer used.")]
    fn default_output(&self) -> &'static OsStr {
        OsStr::new("a.out")
    }

    #[deprecated(since = "0.2.0", note = "Please use `cmd` instead.")]
    #[allow(unused)]
    fn preproc(&self, cmd: &mut Command) {}

    #[deprecated(since = "0.2.0", note = "Please use `dylib` or `staticlib` instead.")]
    #[allow(unused)]
    fn lib(&self, cmd: &mut Command, path: &OsStr) {}
}
