extern crate image;
use std::env;
use std::collections::HashMap;

// Possible arguments for configuration 
const ARG_KEYS: &'static[&'static str] = &["imgpath", "flutterdir"];

fn main() {
    println!("Welcome to AndroiDPI");

    let args: Vec<String> = env::args().collect();
    let args_number = get_args_number(&args);

    let mut setting = HashMap::new();

    for i in 1..(args_number + 1) {
        let index = usize::from((i * 2) - 1);
        let ref argkey = &args[index];
        if is_argkey_valid(*argkey) {
            let ref argvalue = &args[index + 1];
            setting.insert((*argkey).to_string(), (*argvalue).to_string());
        }
    }

    for &argkey in ARG_KEYS {
        match setting.get(argkey) {
            Some(value) => println!("{}: {}", argkey, value),
            None => println!("\"{}\" argument not informed", argkey),
        }
    }
}

fn get_args_number(args: &[String]) -> u8 {
    return (((args.len() - 1) / 2) as f32).ceil() as u8;
}

fn is_argkey_valid(argkey: &String) -> bool {
    for key in ARG_KEYS {
        if *key == argkey {
            return true;
        }
    }

    return false;
}
