pub enum Flag {
    Src,
    OutDir,
    Name,
    Platform,
}

impl Flag {
    pub fn from_str(flag_str: &str) -> Option<Flag> {
        if Flag::Src.is_equal(flag_str) {
            return Some(Flag::Src);
        }

        if Flag::OutDir.is_equal(flag_str) {
            return Some(Flag::OutDir);
        }

        if Flag::Name.is_equal(flag_str) {
            return Some(Flag::Name);
        }

        if Flag::Platform.is_equal(flag_str) {
            return Some(Flag::Platform);
        }

        None
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Flag::Src => "--src",
            Flag::OutDir => "--outDir",
            Flag::Name => "--name",
            Flag::Platform => "--platform",
        }
    }

    pub fn get_all_required() -> [&'static str; 4] {
        return [
            Flag::Src.as_str(),
            Flag::OutDir.as_str(),
            Flag::Name.as_str(),
            Flag::Platform.as_str(),
        ];
    }

    pub fn is_equal(&self, flag: &str) -> bool {
        self.as_str() == flag
    }

    pub fn is_src(flag: &str) -> bool {
        Flag::Src.is_equal(flag)
    }

    pub fn is_out_dir(flag: &str) -> bool {
        Flag::OutDir.is_equal(flag)
    }

    pub fn is_name(flag: &str) -> bool {
        Flag::Name.is_equal(flag)
    }

    pub fn is_platform(flag: &str) -> bool {
        Flag::Platform.is_equal(flag)
    }
}
