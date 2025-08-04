pub enum Flag {
    Src,
    OutDir,
    Name,
    Platform,
}

impl Flag {
    pub fn from_str(flag_str: &str) -> Option<Flag> {
        match flag_str {
            "--src" => Some(Flag::Src),
            "--outDir" => Some(Flag::OutDir),
            "--name" => Some(Flag::Name),
            "--platform" => Some(Flag::Platform),
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
}
