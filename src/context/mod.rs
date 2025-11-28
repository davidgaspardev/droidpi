use std::{collections::HashMap, process::exit};

pub mod mode;
pub use mode::Mode;

use crate::cli::Flag;

pub struct Context {
    pub name: String,
    pub version: String,
    pub mode: Mode,
    pub args: HashMap<String, Option<String>>,
}

impl Context {
    pub fn new(args: HashMap<String, Option<String>>) -> Context {
        Context {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            mode: if args.contains_key(Flag::Version.as_str()) {
                Mode::ShowVersion
            } else if args.contains_key(Flag::Help.as_str()) || args.is_empty() {
                Mode::ShowHelp
            } else {
                Mode::RunMainLogic
            },
            args,
        }
    }

    /// Helper method to get a required argument value
    fn get_required_arg(&self, flag: Flag) -> &String {
        let flag_str = flag.as_str();

        match self.args.get(flag_str) {
            Some(Some(value)) => value,
            Some(None) => {
                eprintln!("Argument '{flag_str}' was provided but has no value. Please provide a valid value for '{flag_str}'.");
                exit(1)
            }
            None => {
                eprintln!("Missing required argument '{flag_str}'. Please provide it using '{flag_str} <value>'.");
                exit(1)
            }
        }
    }

    pub fn get_arg_src(&self) -> &String {
        self.get_required_arg(Flag::Src)
    }

    pub fn get_arg_out_dir(&self) -> &String {
        self.get_required_arg(Flag::OutDir)
    }

    pub fn get_arg_platform(&self) -> &String {
        self.get_required_arg(Flag::Platform)
    }

    pub fn get_arg_name(&self) -> &String {
        self.get_required_arg(Flag::Name)
    }
}
