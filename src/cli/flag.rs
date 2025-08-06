#[derive(PartialEq)]
pub enum Flag {
    Src,
    OutDir,
    Name,
    Platform,
    Version,
    Help,
}

const BOOLEAN_FLAGS: [Flag; 2] = [Flag::Version, Flag::Help];

impl Flag {
    pub fn from_str(flag_str: &str) -> Option<Flag> {
        match flag_str {
            "--src" => Some(Flag::Src),
            "--outDir" => Some(Flag::OutDir),
            "--name" => Some(Flag::Name),
            "--platform" => Some(Flag::Platform),
            "--version" => Some(Flag::Version),
            "--help" => Some(Flag::Help),
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
            Flag::Help => "--help",
        }
    }

    pub fn is_boolean(&self) -> bool {
        return BOOLEAN_FLAGS.contains(&self);
    }
}
