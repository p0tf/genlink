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
            i => *i,
        }
    }

    fn dynamic_(&self) -> Self {
        match self {
            StaticExe => DynamicExe,
            StaticPicExe => DynamicPicExe,
            StaticDyLib => DynamicDyLib,
            i => *i,
        }
    }

    fn pic_(&self) -> Self {
        match self {
            DynamicExe => DynamicPicExe,
            StaticExe => StaticPicExe,
            i => *i,
        }
    }

    fn no_pic_(&self) -> Self {
        match self {
            StaticPicExe => StaticExe,
            DynamicPicExe => DynamicExe,
            i => *i,
        }
    }

    fn lib_(&self) -> Self {
        match self {
            DynamicExe | DynamicPicExe => DynamicDyLib,
            StaticExe | StaticPicExe => StaticDyLib,
            i => *i,
        }
    }

    fn exe_(&self) -> Self {
        match self {
            StaticDyLib => StaticExe,
            DynamicDyLib => DynamicExe,
            i => *i,
        }
    }
}
