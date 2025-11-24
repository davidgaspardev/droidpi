use crate::cli::CliArgs;

pub struct Context {
    pub args: CliArgs,
}

impl Context {
    pub fn new(args: CliArgs) -> Context {
        Context {
            args,
        }
    }

    pub fn get_arg_src(&self) -> &str {
        self.args.src.to_str().expect("Source path contains invalid UTF-8 characters")
    }

    pub fn get_arg_out_dir(&self) -> &str {
        self.args.out_dir.to_str().expect("Output directory path contains invalid UTF-8 characters")
    }

    pub fn get_arg_platform(&self) -> &str {
        self.args.platform.as_str()
    }

    pub fn get_arg_name(&self) -> &str {
        &self.args.name
    }
    
    pub fn use_drawable(&self) -> bool {
        self.args.use_drawable
    }
}
