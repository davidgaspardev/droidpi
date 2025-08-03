pub mod flag;
use flag::Flag;

use std::collections::HashMap;
use std::env;
use std::path::Path;

pub fn get_arguments() -> Result<HashMap<String, String>, String> {
    let args: Vec<String> = env::args().collect();

    return load_args(args);
}

/// Load the arguments from the command line to a HashMap.
/// The keys are the argument names and the values are the argument values.
/// Returns an error if the arguments are not valid.
fn load_args(args: Vec<String>) -> Result<HashMap<String, String>, String> {
    let args_number = get_args_number(&args);
    let mut setting = HashMap::new();

    for i in 1..(args_number + 1) {
        let index = usize::from((i * 2) - 1);

        // Checking arg key
        let argkey = &args[index];
        if !is_argkey_valid(argkey) {
            return Err(format!(r#""{}" argument not valid"#, argkey).to_string());
        }

        // Checking arg value
        let argvalue = &args[index + 1];
        if !is_argvalue_valid(argkey, argvalue) {
            return Err(format!(r#""{}" argument value not valid"#, argvalue).to_string());
        }

        // Add in the setting
        setting.insert((*argkey).to_string(), (*argvalue).to_string());
    }

    for flag in Flag::get_all_required() {
        if setting.get(flag) == None {
            return Err(format!(r#""{}" argument not informed"#, flag).to_string());
        }
    }

    Ok(setting)
}

fn get_args_number(args: &[String]) -> u8 {
    return (((args.len() - 1) / 2) as f32).ceil() as u8;
}

fn is_argkey_valid(argkey: &String) -> bool {
    Flag::from_str(argkey).is_some()
}

fn is_argvalue_valid(argkey: &String, argvalue: &String) -> bool {
    if Flag::is_src(argkey) {
        if !Path::new(argvalue).is_file() || !is_path_imagefile(argvalue) {
            return false;
        }
    }

    if Flag::is_out_dir(argkey) {
        if !Path::new(argvalue).is_dir() {
            return false;
        }
    }

    if Flag::is_platform(argkey) {
        if argvalue != "android" && argvalue != "flutter" {
            return false;
        }
    }

    true
}

fn is_path_imagefile(argvalue: &String) -> bool {
    let path_ext: String = argvalue.chars().skip(argvalue.len() - 4).collect();
    path_ext == ".png" || path_ext == ".jpg" || path_ext == "jpeg"
}
