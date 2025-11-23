#[derive(PartialEq)]
pub enum Flag {
    Src,
    OutDir,
    Name,
    Platform,
    Version,
    Help,
    UseDrawable,
}

const BOOLEAN_FLAGS: [Flag; 3] = [Flag::Version, Flag::Help, Flag::UseDrawable];

impl Flag {
    pub fn from_str(flag_str: &str) -> Option<Flag> {
        match flag_str {
            "--src" => Some(Flag::Src),
            "--outDir" => Some(Flag::OutDir),
            "--name" => Some(Flag::Name),
            "--platform" => Some(Flag::Platform),
            "--version" => Some(Flag::Version),
            "--help" => Some(Flag::Help),
            "--use-drawable" => Some(Flag::UseDrawable),
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
            Flag::UseDrawable => "--use-drawable",
        }
    }

    pub fn is_boolean(&self) -> bool {
        BOOLEAN_FLAGS.contains(&self)
    }
}
