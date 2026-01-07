pub mod flag;
pub use flag::Flag;

use std::collections::HashMap;
use std::env;
use std::path::Path;

pub fn get_arguments() -> Result<HashMap<String, Option<String>>, String> {
    let args: Vec<String> = env::args().collect();

    load_args(args)
}

/// Load the arguments from the command line to a HashMap.
/// The keys are the argument names and the values are the argument values.
/// Returns an error if the arguments are not valid.
fn load_args(args: Vec<String>) -> Result<HashMap<String, Option<String>>, String> {
    let args_number = get_args_number(&args);
    let mut setting = HashMap::new();

    for i in 1..(args_number + 1) {
        let index = usize::from((i * 2) - 1);

        // Checking arg key
        let argkey = &args[index];
        let flag = get_flag_from_arg(argkey);

        match flag {
            Some(flag) => {
                if flag.is_boolean() {
                    setting.insert(flag.as_str().to_string(), None);
                } else {
                    // Checking arg value
                    let argvalue = &args[index + 1];
                    if !is_argvalue_valid(argkey, argvalue) {
                        return Err(format!(r#""{}" argument value not valid"#, argvalue));
                    }

                    // Add in the setting
                    setting.insert(flag.as_str().to_string(), Some((*argvalue).to_string()));
                }
            }
            None => {
                return Err(format!(r#""{}" argument not valid"#, argkey));
            }
        }
    }

    Ok(setting)
}

fn get_args_number(args: &[String]) -> u8 {
    args.iter().filter(move |arg| arg.starts_with("--")).count() as u8
}

fn get_flag_from_arg(argkey: &str) -> Option<Flag> {
    Flag::from_str(argkey)
}

fn is_argvalue_valid(argkey: &str, argvalue: &String) -> bool {
    match Flag::from_str(argkey) {
        Some(Flag::Src) => Path::new(argvalue).is_file() && is_path_imagefile(argvalue),
        Some(Flag::OutDir) => Path::new(argvalue).is_dir(),
        Some(Flag::Platform) => argvalue == "android" || argvalue == "flutter",
        Some(_) => true,
        _ => false,
    }
}

fn is_path_imagefile(argvalue: &str) -> bool {
    let path_ext: String = argvalue.chars().skip(argvalue.len() - 4).collect();
    path_ext == ".png" || path_ext == ".jpg" || path_ext == "jpeg"
}
