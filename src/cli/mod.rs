use std::env;
use std::collections::HashMap;

pub const IMAGE_PATH: &'static str = "--imagepath";
pub const FLUTTER_PATH: &'static str = "--flutterpath";

// Possible arguments for configuration 
const POSSIBLE_ARGS: &'static[&'static str] = &[
    IMAGE_PATH,
    FLUTTER_PATH
];

pub fn get_arguments() -> Result<HashMap<String, String>, String> {

    let args: Vec<String> = env::args().collect();

    return load_args(args);
}

fn load_args(args: Vec<String>) -> Result<HashMap<String, String>, String> {
    let args_number = get_args_number(&args);

    let mut setting = HashMap::new();

    for i in 1..(args_number + 1) {
        let index = usize::from((i * 2) - 1);
        let ref argkey = &args[index];
        if is_argkey_valid(*argkey) {
            let ref argvalue = &args[index + 1];
            setting.insert((*argkey).to_string(), (*argvalue).to_string());
        } else {
            return Err(String::from("Arg key invalid"));
        }
    }

    for &argkey in POSSIBLE_ARGS {
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
    for arg in POSSIBLE_ARGS {
        if *arg == argkey {
            return true;
        }
    }

    return false;
}
