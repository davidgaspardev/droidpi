pub enum Flag {
    Src,
    OutDir,
    Name,
    Platform,
    Version,
}

impl Flag {
    pub fn from_str(flag_str: &str) -> Option<Flag> {
        match flag_str {
            "--src" => Some(Flag::Src),
            "--outDir" => Some(Flag::OutDir),
            "--name" => Some(Flag::Name),
            "--platform" => Some(Flag::Platform),
            "--version" => Some(Flag::Version),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Flag::Src => "--src",
            Flag::OutDir => "--outDir",
            Flag::Name => "--name",
            Flag::Platform => "--platform",
            Flag::Version => "--version",
        }
    }

    pub fn has_value(&self) -> bool {
        if self.as_str() == Flag::Version.as_str() {
            false
        } else {
            true
        }
    }
}
