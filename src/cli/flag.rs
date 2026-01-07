#[derive(PartialEq)]
pub enum Flag {
    Src,
    OutDir,
    Name,
    Platform,
    Version,
    Help,
    UseDrawable,
    Round,
}

const BOOLEAN_FLAGS: [Flag; 4] = [Flag::Version, Flag::Help, Flag::UseDrawable, Flag::Round];

impl Flag {
    pub fn from_str(flag_str: &str) -> Option<Flag> {
        match flag_str {
            "--src" => Some(Flag::Src),
            "--outdir" => Some(Flag::OutDir),
            "--outDir" => {
                // Deprecated: --outDir is still supported for backwards compatibility
                eprintln!("Warning: --outDir is deprecated. Please use --outdir instead.");
                Some(Flag::OutDir)
            }
            "--name" => Some(Flag::Name),
            "--platform" => Some(Flag::Platform),
            "--version" => Some(Flag::Version),
            "--help" => Some(Flag::Help),
            "--use-drawable" => Some(Flag::UseDrawable),
            "--round" => Some(Flag::Round),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Flag::Src => "--src",
            Flag::OutDir => "--outdir",
            Flag::Name => "--name",
            Flag::Platform => "--platform",
            Flag::Version => "--version",
            Flag::Help => "--help",
            Flag::UseDrawable => "--use-drawable",
            Flag::Round => "--round",
        }
    }

    pub fn is_boolean(&self) -> bool {
        BOOLEAN_FLAGS.contains(self)
    }
}
