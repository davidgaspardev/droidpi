use std::collections::HashMap;
use std::env;
use std::path::Path;

pub const IMAGE_PATH: &'static str = "--imagepath";
pub const FLUTTER_PATH: &'static str = "--flutterpath";
pub const IMAGE_NAME: &'static str = "--name";

pub const FLAG_SRC: &'static str = "--src";
pub const FLAG_OUTDIR: &'static str = "--outDir";
pub const FLAG_NAME: &'static str = "--name";
pub const FLAG_TARGET: &'static str = "--target";

// Possible arguments for configuration
const POSSIBLE_FLAGS: &'static [&'static str] = &[FLAG_SRC, FLAG_OUTDIR, FLAG_NAME, FLAG_TARGET];

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

    for &argkey in POSSIBLE_FLAGS {
        if setting.get(argkey) == None {
            return Err(format!(r#""{}" argument not informed"#, argkey).to_string());
        }
    }

    Ok(setting)
}

fn get_args_number(args: &[String]) -> u8 {
    return (((args.len() - 1) / 2) as f32).ceil() as u8;
}

fn is_argkey_valid(argkey: &String) -> bool {
    for arg in POSSIBLE_FLAGS {
        if argkey.eq(*arg) {
            return true;
        }
    }

    false
}

fn is_argvalue_valid(argkey: &String, argvalue: &String) -> bool {
    if argkey == FLAG_SRC {
        if !Path::new(argvalue).is_file() || !is_path_imagefile(argvalue) {
            return false;
        }
    }

    if argkey == FLAG_OUTDIR {
        if !Path::new(argvalue).is_dir() {
            return false;
        }
    }

    if argkey == FLAG_TARGET {
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
