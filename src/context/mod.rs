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
            } else {
                Mode::RunMainLogic
            },
            args,
        }
    }

    pub fn get_arg_src(&self) -> &String {
        if let Some(optional_arg_src) = self.args.get(Flag::Src.as_str()) {
            if let Some(arg_src) = optional_arg_src {
                return arg_src;
            }
        }
        eprintln!("Fail to get src argument");
        exit(1)
    }

    pub fn get_arg_out_dir(&self) -> &String {
        if let Some(optional_arg_out_dir) = self.args.get(Flag::OutDir.as_str()) {
            if let Some(arg_out_dir) = optional_arg_out_dir {
                return arg_out_dir;
            }
        }
        eprintln!("Fail to get out dir argument");
        exit(1)
    }

    pub fn get_arg_platform(&self) -> &String {
        if let Some(optional_arg_platform) = self.args.get(Flag::Platform.as_str()) {
            if let Some(arg_platform) = optional_arg_platform {
                return arg_platform;
            }
        }
        eprintln!("Fail to get platform argument");
        exit(1)
    }

    pub fn get_arg_name(&self) -> &String {
        if let Some(optional_arg_name) = self.args.get(Flag::Name.as_str()) {
            if let Some(arg_name) = optional_arg_name {
                return arg_name;
            }
        }
        eprintln!("Fail to get name argument");
        exit(1)
    }
}
