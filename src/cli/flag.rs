pub enum Flag {
    Src,
    OutDir,
    Name,
    Platform,
}

impl Flag {
    pub fn from_str(flag_str: &str) -> Option<Flag> {
        match flag_str {
            s if Flag::Src.is_equal(s) => Some(Flag::Src),
            s if Flag::OutDir.is_equal(s) => Some(Flag::OutDir),
            s if Flag::Name.is_equal(s) => Some(Flag::Name),
            s if Flag::Platform.is_equal(s) => Some(Flag::Platform),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Flag::Src => "--src",
            Flag::OutDir => "--outDir",
            Flag::Name => "--name",
            Flag::Platform => "--platform",
        }
    }

    pub fn get_all_required() -> Vec<&'static str> {
        vec![
            Flag::Src.as_str(),
            Flag::OutDir.as_str(),
            Flag::Name.as_str(),
            Flag::Platform.as_str(),
        ]
    }

    pub fn is_equal(&self, flag: &str) -> bool {
        self.as_str() == flag
    }
}
